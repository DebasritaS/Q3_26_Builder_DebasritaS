import {
  appendTransactionMessageInstruction,
  appendTransactionMessageInstructions,
  assertIsTransactionMessageWithBlockhashLifetime,
  assertIsTransactionWithBlockhashLifetime,
  createKeyPairSignerFromBytes,
  createSolanaRpc,
  createSolanaRpcSubscriptions,
  createTransactionMessage,
  generateKeyPairSigner,
  getSignatureFromTransaction,
  sendAndConfirmTransactionFactory,
  setTransactionMessageFeePayerSigner,
  setTransactionMessageLifetimeUsingBlockhash,
  signTransactionMessageWithSigners,
} from "@solana/kit";
import {
  getInitializeMintInstruction,
  getMintSize,
  TOKEN_PROGRAM_ADDRESS,
} from "@solana-program/token";
import { getCreateAccountInstruction } from "@solana-program/system";

//import your wallet
import wallet from "/home/debasritasaha/.config/solana/id.json";

const rpc = createSolanaRpc("https://api.devnet.solana.com");

const rpcSubscriptions = createSolanaRpcSubscriptions(
  "wss://api.devnet.solana.com",
);

(async () => {
  try {

    //create a signer from your wallet
    const signer = await createKeyPairSignerFromBytes(new Uint8Array(wallet));
    //generate a new mint signer for address
    const mint = await generateKeyPairSigner();

    //get the size of the mint account
    const space = BigInt(getMintSize()); 

    //get the minimum balance for rent exemption
    const rent = await rpc.getMinimumBalanceForRentExemption(space).send();
    
    const {value: latestBlockhash} = await rpc.getLatestBlockhash().send();

    const sendAndConfirm = sendAndConfirmTransactionFactory({
      rpc,
      rpcSubscriptions,
    });

    const msg = createTransactionMessage({ version: 0 });
    
    const msgWithPayer = setTransactionMessageFeePayerSigner(signer, msg);  

    const msgWithLifetime = setTransactionMessageLifetimeUsingBlockhash(
      latestBlockhash,
      msgWithPayer,
    );

    const txMessage = appendTransactionMessageInstructions(
      [
         getCreateAccountInstruction({
          payer: signer,
          newAccount: mint,
          lamports: rent,
          space,
          programAddress: TOKEN_PROGRAM_ADDRESS,
        }),
        getInitializeMintInstruction({
          mint: mint.address,
          decimals: 6,
          mintAuthority: signer.address,
        }),


      ],
      msgWithLifetime,
    );
    
    const signedTx = await signTransactionMessageWithSigners(txMessage);

    assertIsTransactionWithBlockhashLifetime(signedTx);

    const signature = getSignatureFromTransaction(signedTx);

    await sendAndConfirm(signedTx, {
      commitment: "confirmed",
    });

    console.log(`mint address: ${mint.address}, Transaction signature: ${signature}`);




  } catch (error) {
    console.log(error);
  }
})();

// OUTPUT:

// mint address: HoeRyA8MGTpVLhgB1fbbWfdbkFvUe3obfrHLzY6wJG6c , 
// Transaction signature: 2QhmKer1ZL3Qe8bZwA9mwPWf85AqdMM5PBgHXDoFJ1CPuUBDZy445vMncT2ceUmpzMJaEz3UCu2NXsr4viuDSFd6
