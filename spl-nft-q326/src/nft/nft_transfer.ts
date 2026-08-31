import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";

import {
  createSignerFromKeypair,
  signerIdentity,
  publicKey,
} from "@metaplex-foundation/umi";

import {
  transfer,
  mplCore,
  fetchAsset
  } from "@metaplex-foundation/mpl-core";

import wallet from "/home/debasritasaha/.config/solana/id.json";


const umi = createUmi(
  "https://api.devnet.solana.com"
).use(mplCore());


// Load your wallet
const keypair = umi.eddsa.createKeypairFromSecretKey(
  new Uint8Array(wallet)
);


// Create signer
const signer = createSignerFromKeypair(umi, keypair);


// Tell Umi this wallet is the identity/update authority
umi.use(signerIdentity(signer));


const assetAddress = publicKey(
  "Eu2YxsTk3HV681X9ADXM7LGDskxnUX4stgW8SAk8Gc5h"
);


(async () => {
  try {

   
    // Fetch the complete NFT asset from Solana
    const asset = await fetchAsset(umi, assetAddress);

    console.log("Current owner:", asset.owner);

    // 1. Transfer the NFT "Purple Dream" of generug2.png to the new owner
    const result = await transfer(umi, {
      asset ,
      newOwner : publicKey("2XqwL7FCLsWDrh9uDjxEtREMiRnMnqJGKjZEGAc6TpxH"),
      }).sendAndConfirm(umi);


   
    console.log("Asset transferred successfully");
    

    

  } catch (e) {
    console.log("Error:", e);
  }
})();

// nft-update.ts
// signature:   3pViq2ELWgUkkuTGeYpfFbun46tpNPFMoV2JsVV3Cu4eRB9tVvKvevACV49uCwBohDBDLCDQ9nVzhxSSFLKkB14a
