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
import { programID,treasury_accoun } from "./ids";

import { randomString } from "./random_string_gen";

const BN = require("bn.js");

export const init_platform = async (owner,nonce,fees) => {

  const server_seed = new PublicKey(randomString());
  
  const platform_data_account = await PublicKey.findProgramAddress(
    [
      Buffer.from("betting_contract"),
    ],
    programID,
  );

  console.log("Platform State PDA",platform_data_account[0].toString());


  const initEscrowIx = new TransactionInstruction({
    programId: programID,
    keys: [
     
      { pubkey: owner, isSigner: true, isWritable: false },

      { pubkey: server_seed, isSigner: false, isWritable: true },

      { pubkey: treasury_accoun, isSigner: false, isWritable: false },

      { pubkey: platform_data_account[0], isSigner: false, isWritable: true },


      { pubkey: SystemProgram.programId, isSigner: false, isWritable: true },


    ],
    data: Buffer.from(Uint8Array.of(0, 
      ...new BN(nonce).toArray("le", 8),
      ...new BN(fees).toArray("le", 8),


      )),
      
  });

  await sendTxUsingExternalSignature(
    [initEscrowIx],
    connection,
    null,
    [],
    new PublicKey(owner)
  );
  await new Promise((resolve) => setTimeout(resolve, 2000));


  console.log(`Platform data initilized sucssfully ***********************  \n`);
};
