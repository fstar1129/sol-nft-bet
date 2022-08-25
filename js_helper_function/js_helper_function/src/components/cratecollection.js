import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import {
  Keypair,
  PublicKey,
  SystemProgram,
  //Transaction,
  TransactionInstruction,
} from "@solana/web3.js";
import { connection } from "./connection";
import { sendTxUsingExternalSignature } from "./externalwallet";
import { programID ,collection_mint} from "./ids";

const BN = require("bn.js");

export const create_collection = async (owner) => {

 
  const collection_state_account = await PublicKey.findProgramAddress(
    [
      Buffer.from("collection"),
      collection_mint.toBuffer(),

    ],
    programID,
  );

  console.log("Collection State PDA",collection_state_account[0].toString());


  const initEscrowIx = new TransactionInstruction({
    programId: programID,
    keys: [
     
      { pubkey: owner, isSigner: true, isWritable: false },

      { pubkey: collection_state_account[0], isSigner: false, isWritable: true },

      { pubkey: collection_mint, isSigner: false, isWritable: true },

      { pubkey: SystemProgram.programId, isSigner: false, isWritable: true },


    ],
    data: Buffer.from(Uint8Array.of(1  )),
      
  });

  await sendTxUsingExternalSignature(
    [initEscrowIx],
    connection,
    null,
    [],
    new PublicKey(owner)
  );
  await new Promise((resolve) => setTimeout(resolve, 2000));


  console.log(`collection state created ***********************  \n`);
};
