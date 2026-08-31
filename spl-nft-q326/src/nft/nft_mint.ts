import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import wallet from "/home/debasritasaha/.config/solana/id.json";
import {
  createSignerFromKeypair,
  generateSigner,
  signerIdentity,
} from "@metaplex-foundation/umi";
import { create, mplCore } from "@metaplex-foundation/mpl-core";
import { base58 } from "@metaplex-foundation/umi/serializers";

const umi = createUmi(
  process.env.SOLANA_RPC_URL ?? "https://api.devnet.solana.com",
);

const keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(signerIdentity(signer));

umi.use(mplCore());

(async () => {
  try {
    const metadataUri =
      " https://gateway.irys.xyz/5Uq29GzRa9KBnu6zovtgUCJz781Ew21E2xJTiKAbaNtL ";
    const asset = generateSigner(umi);

    //add you nft name and metadata uri
     const tx = await create(umi, {
      asset,
      name: "Purple Dream",
      uri: metadataUri,
    });
    
    const result = await tx.sendAndConfirm(umi);

    const signature = base58.deserialize(result.signature)[0];

     console.log(`signature ${signature} , asset : ${asset.publicKey}`);
  } catch (e) {
    console.log(`errior ${e}`);
  }
})();

// nft-mint.ts
// signature 4xe6KKoLCbDaQubYDgJ6a3jK9FEYsyM27Qtx3mevYKPoSrSPLwGWYkiMZv7rkfWx2jFzKbjekHy7eGoTnmvtGE5i , 
// asset : 8vbYaa2Xr9N3cH449iWWWPGshcG9HzJ4ATKgHsDtB23R

//new nft minted
//signature 5C8LkEVTkfxhuEKuMX4uctFeQeM4grDZUKqACjvWWcC6wfmaiSAw9FY958YH1GRcQKHQHzPxfbrMAp2ddtb3mBMY , 
//asset : Eu2YxsTk3HV681X9ADXM7LGDskxnUX4stgW8SAk8Gc5h