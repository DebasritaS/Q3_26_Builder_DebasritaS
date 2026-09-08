use {
    anchor_lang::{
        prelude::msg,
        solana_program::{
            instruction::Instruction,
            program_pack::Pack,
        },
        system_program::ID as SYSTEM_PROGRAM_ID,
        AccountDeserialize,
        InstructionData,
        ToAccountMetas,
    },
    anchor_spl::{
        associated_token::{self, ID as ASSOCIATED_TOKEN_PROGRAM_ID},
        token::spl_token,
    },
    litesvm::LiteSVM,
    litesvm_token::{
        spl_token::ID as TOKEN_PROGRAM_ID,
        CreateAssociatedTokenAccount,
        CreateMint,
        MintTo,
    },
    solana_keypair::{Address, Keypair},
    solana_message::Message,
    solana_pubkey::Pubkey,
    solana_signer::Signer,
    solana_transaction::Transaction,
};


// ------------------------------------------------------------
// Test environment
// ------------------------------------------------------------

fn setup() -> (
    LiteSVM,
    Keypair,
    Address,
    Keypair,
    Address,
    Address,
    Address,
    Address,
    Address,
    Address,
    Address,
    Address,
) {
    let payer = Keypair::new();
    let mut svm = LiteSVM::new();

    let program_id = escrowq32026::id();

    let program_bytes = include_bytes!(concat!(
        env!("CARGO_TARGET_TMPDIR"),
        "/../deploy/escrowq32026.so"
    ));

    svm.add_program(program_id, program_bytes).unwrap();
    svm.airdrop(&payer.pubkey(), 1_000_000_000).unwrap();

    let maker = payer.pubkey();

    // Create the two assets used by the swap.
    let mint_a = CreateMint::new(&mut svm, &payer)
        .decimals(6)
        .authority(&maker)
        .send()
        .unwrap();

    let mint_b = CreateMint::new(&mut svm, &payer)
        .decimals(6)
        .authority(&maker)
        .send()
        .unwrap();

    // Maker accounts for both tokens.
    let maker_ata_a = CreateAssociatedTokenAccount::new(
        &mut svm,
        &payer,
        &mint_a,
    )
    .owner(&maker)
    .send()
    .unwrap();

    let maker_ata_b = CreateAssociatedTokenAccount::new(
        &mut svm,
        &payer,
        &mint_b,
    )
    .owner(&maker)
    .send()
    .unwrap();

    // PDA used by this test suite.
    let seed = 123u64;

    let escrow = Pubkey::find_program_address(
        &[
            b"escrow",
            maker.as_ref(),
            &seed.to_le_bytes(),
        ],
        &program_id,
    )
    .0;

    let vault =
        associated_token::get_associated_token_address(&escrow, &mint_a);

    // Give the maker token A for the initial deposit.
    MintTo::new(
        &mut svm,
        &payer,
        &mint_a,
        &maker_ata_a,
        1_000_000_000,
    )
    .send()
    .unwrap();

    // Create the second participant.
    let taker = Keypair::new();

    svm.airdrop(&taker.pubkey(), 1_000_000_000)
        .unwrap();

    let taker_ata_a = CreateAssociatedTokenAccount::new(
        &mut svm,
        &taker,
        &mint_a,
    )
    .owner(&taker.pubkey())
    .send()
    .unwrap();

    let taker_ata_b = CreateAssociatedTokenAccount::new(
        &mut svm,
        &taker,
        &mint_b,
    )
    .owner(&taker.pubkey())
    .send()
    .unwrap();

    (
        svm,
        payer,
        maker,
        taker,
        mint_a,
        mint_b,
        maker_ata_a,
        maker_ata_b,
        taker_ata_a,
        taker_ata_b,
        escrow,
        vault,
    )
}


// ------------------------------------------------------------
// Make + Refund
// ------------------------------------------------------------

#[test]
fn test_make_and_refund() {
    let (
        mut svm,
        payer,
        maker,
        _taker,
        mint_a,
        mint_b,
        maker_ata_a,
        maker_ata_b,
        _taker_ata_a,
        _taker_ata_b,
        escrow,
        vault,
    ) = setup();

    let make_ix = Instruction {
        program_id: escrowq32026::id(),

        accounts: escrowq32026::accounts::Make {
            maker,
            mint_a,
            mint_b,
            maker_ata_a,
            maker_ata_b,
            escrow,
            vault,
            associated_token_program: ASSOCIATED_TOKEN_PROGRAM_ID,
            token_program: TOKEN_PROGRAM_ID,
            system_program: SYSTEM_PROGRAM_ID,
        }
        .to_account_metas(None),

        data: escrowq32026::instruction::Make {
            deposit: 10_000_000,
            seed: 123u64,
            receive: 10_000_000,
            expiration: 17_780_206_209,
        }
        .data(),
    };

    let message =
        Message::new(&[make_ix], Some(&payer.pubkey()));

    let transaction = Transaction::new(
        &[&payer],
        message,
        svm.latest_blockhash(),
    );

    let result = svm.send_transaction(transaction).unwrap();

    msg!("Make transaction successful");
    msg!("Compute units: {}", result.compute_units_consumed);
    msg!("Signature: {}", result.signature);

    // Verify deposited amount in the vault.
    let vault_account = svm.get_account(&vault).unwrap();

    let vault_data =
        spl_token::state::Account::unpack(&vault_account.data)
            .unwrap();

    assert_eq!(vault_data.amount, 10_000_000);
    assert_eq!(vault_data.owner, escrow);
    assert_eq!(vault_data.mint, mint_a);

    // Verify escrow state.
    let escrow_account = svm.get_account(&escrow).unwrap();

    let escrow_data =
        escrowq32026::state::Escrow::try_deserialize(
            &mut escrow_account.data.as_ref(),
        )
        .unwrap();

    assert_eq!(escrow_data.seed, 123u64);
    assert_eq!(escrow_data.maker, maker);
    assert_eq!(escrow_data.mint_a, mint_a);
    assert_eq!(escrow_data.mint_b, mint_b);
    assert_eq!(escrow_data.receive, 10_000_000);
    assert_eq!(escrow_data.maker_ata_b, maker_ata_b);

    // Refund the escrow.
    let refund_ix = Instruction {
        program_id: escrowq32026::id(),

        accounts: escrowq32026::accounts::Refund {
            maker,
            mint_a,
            maker_ata_a,
            escrow,
            vault,
            token_program: TOKEN_PROGRAM_ID,
            system_program: SYSTEM_PROGRAM_ID,
        }
        .to_account_metas(None),

        data: escrowq32026::instruction::Refund {}.data(),
    };

    let message =
        Message::new(&[refund_ix], Some(&payer.pubkey()));

    let transaction = Transaction::new(
        &[&payer],
        message,
        svm.latest_blockhash(),
    );

    let result = svm.send_transaction(transaction).unwrap();

    msg!("Refund transaction successful");
    msg!("Compute units: {}", result.compute_units_consumed);
    msg!("Signature: {}", result.signature);

    // Both program-owned accounts should be closed.
    assert!(svm.get_account(&escrow).is_none());
    assert!(svm.get_account(&vault).is_none());
}


// ------------------------------------------------------------
// Take
// ------------------------------------------------------------

#[test]
fn test_take() {
    let (
        mut svm,
        payer,
        maker,
        taker,
        mint_a,
        mint_b,
        maker_ata_a,
        maker_ata_b,
        taker_ata_a,
        taker_ata_b,
        escrow,
        vault,
    ) = setup();

    // Open the escrow.
    let make_ix = Instruction {
        program_id: escrowq32026::id(),

        accounts: escrowq32026::accounts::Make {
            maker,
            mint_a,
            mint_b,
            maker_ata_a,
            maker_ata_b,
            escrow,
            vault,
            associated_token_program: ASSOCIATED_TOKEN_PROGRAM_ID,
            token_program: TOKEN_PROGRAM_ID,
            system_program: SYSTEM_PROGRAM_ID,
        }
        .to_account_metas(None),

        data: escrowq32026::instruction::Make {
            deposit: 10_000_000,
            seed: 123u64,
            receive: 10_000_000,
            expiration: 17_780_206_209,
        }
        .data(),
    };

    let message =
        Message::new(&[make_ix], Some(&payer.pubkey()));

    let transaction = Transaction::new(
        &[&payer],
        message,
        svm.latest_blockhash(),
    );

    svm.send_transaction(transaction).unwrap();

    // Fund the taker with token B.
    MintTo::new(
        &mut svm,
        &payer,
        &mint_b,
        &taker_ata_b,
        10_000_000,
    )
    .send()
    .unwrap();

    let take_ix = Instruction {
        program_id: escrowq32026::id(),

        accounts: escrowq32026::accounts::Take {
            taker: taker.pubkey(),
            mint_a,
            taker_ata_b,
            mint_b,
            maker_ata_a,
            maker_ata_b,
            taker_ata_a,
            escrow,
            maker: maker.into(),
            vault,
            token_program: TOKEN_PROGRAM_ID,
            system_program: SYSTEM_PROGRAM_ID,
        }
        .to_account_metas(None),

        data: escrowq32026::instruction::Take {}.data(),
    };

    let message =
        Message::new(&[take_ix], Some(&taker.pubkey()));

    let transaction = Transaction::new(
        &[&taker],
        message,
        svm.latest_blockhash(),
    );

    let result = svm.send_transaction(transaction).unwrap();

    msg!("Take transaction successful");
    msg!("Compute units: {}", result.compute_units_consumed);
    msg!("Signature: {}", result.signature);

    // Escrow and vault are closed after a successful swap.
    assert!(svm.get_account(&escrow).is_none());
    assert!(svm.get_account(&vault).is_none());

    // Maker receives token B.
    let maker_b =
        spl_token::state::Account::unpack(
            &svm.get_account(&maker_ata_b).unwrap().data,
        )
        .unwrap();

    assert_eq!(maker_b.amount, 10_000_000);

    // Taker receives token A.
    let taker_a =
        spl_token::state::Account::unpack(
            &svm.get_account(&taker_ata_a).unwrap().data,
        )
        .unwrap();

    assert_eq!(taker_a.amount, 10_000_000);
}


// ------------------------------------------------------------
// Update
// ------------------------------------------------------------

#[test]
fn test_update() {
    let (
        mut svm,
        payer,
        maker,
        _taker,
        mint_a,
        mint_b,
        maker_ata_a,
        maker_ata_b,
        _taker_ata_a,
        _taker_ata_b,
        escrow,
        vault,
    ) = setup();

    let seed = 123u64;
    let initial_receive = 10_000_000u64;
    let initial_expiration = 17_780_206_209i64;

    // First create the escrow.
    let make_ix = Instruction {
        program_id: escrowq32026::id(),

        accounts: escrowq32026::accounts::Make {
            maker,
            mint_a,
            mint_b,
            maker_ata_a,
            maker_ata_b,
            escrow,
            vault,
            associated_token_program: ASSOCIATED_TOKEN_PROGRAM_ID,
            token_program: TOKEN_PROGRAM_ID,
            system_program: SYSTEM_PROGRAM_ID,
        }
        .to_account_metas(None),

        data: escrowq32026::instruction::Make {
            deposit: 10_000_000,
            seed,
            receive: initial_receive,
            expiration: initial_expiration,
        }
        .data(),
    };

    let message =
        Message::new(&[make_ix], Some(&payer.pubkey()));

    let transaction = Transaction::new(
        &[&payer],
        message,
        svm.latest_blockhash(),
    );

    svm.send_transaction(transaction).unwrap();

    // Confirm the initial values.
    let account = svm.get_account(&escrow).unwrap();

    let before =
        escrowq32026::state::Escrow::try_deserialize(
            &mut account.data.as_ref(),
        )
        .unwrap();

    assert_eq!(before.receive, initial_receive);
    assert_eq!(before.expiration, initial_expiration);

    // Update both receive amount and expiration.
    let changed_receive = 20_000_000u64;
    let changed_expiration = 18_880_206_209i64;

    let update_ix = Instruction {
        program_id: escrowq32026::id(),

        accounts: escrowq32026::accounts::Update {
            maker,
            escrow,
        }
        .to_account_metas(None),

        data: escrowq32026::instruction::Update {
            receive: changed_receive,
            expiration: changed_expiration,
        }
        .data(),
    };

    let message =
        Message::new(&[update_ix], Some(&payer.pubkey()));

    let transaction = Transaction::new(
        &[&payer],
        message,
        svm.latest_blockhash(),
    );

    let result = svm.send_transaction(transaction).unwrap();

    msg!("Update transaction successful");
    msg!("Compute units: {}", result.compute_units_consumed);
    msg!("Signature: {}", result.signature);

    // Check the updated state.
    let account = svm.get_account(&escrow).unwrap();

    let after =
        escrowq32026::state::Escrow::try_deserialize(
            &mut account.data.as_ref(),
        )
        .unwrap();

    assert_eq!(after.receive, changed_receive);
    assert_eq!(after.expiration, changed_expiration);

    // Other escrow fields should remain unchanged.
    assert_eq!(after.seed, seed);
    assert_eq!(after.maker, maker);
    assert_eq!(after.mint_a, mint_a);
    assert_eq!(after.mint_b, mint_b);
    assert_eq!(after.maker_ata_b, maker_ata_b);
}

