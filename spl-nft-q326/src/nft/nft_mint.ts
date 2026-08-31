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
      " https://gateway.irys.xyz/3cdZ1uJ3RofiQd8A4wZKUna2ccMDsDyNW5HcWzRQJ8nq  ";
    const asset = generateSigner(umi);

    //add you nft name and metadata uri
     const tx = await create(umi, {
      asset,
      name: "Debrug",
      uri: metadataUri,
    });
    
    const result = await tx.sendAndConfirm(umi);

    const signature = base58.deserialize(result.signature)[0];

     console.log(`signature ${signature} , asset : ${asset.publicKey}`);
  } catch (e) {
    console.log(`errior ${e}`);
  }
})();
