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

import { randomString } from "./random_string_gen";

const BN = require("bn.js");

export const create_game = async (owner) => {

  const client_seed = new PublicKey(randomString());
 
  const collection_state_account = await PublicKey.findProgramAddress(
    [
      Buffer.from("collection"),
      collection_mint.toBuffer(),

    ],
    programID,
  );

  const platform_state_account = await PublicKey.findProgramAddress(
    [
      Buffer.from("betting_contract"),

    ],
    programID,
  );

  const game_state_account = await PublicKey.findProgramAddress(
    [
      Buffer.from("0"),
      collection_state_account[0].toBuffer(),

    ],
    programID,
  );

  const mint_state = await PublicKey.findProgramAddress(
    [
      Buffer.from("mints"),
      game_state_account[0].toBuffer(),

    ],
    programID,
  );

  console.log("Collection State PDA ",game_state_account[0].toString());
  console.log("Platform State PDA ",platform_state_account[0].toString());
  console.log("Game State PDA", game_state_account[0].toString());
  console.log("Mint State PDA", mint_state[0].toString());


  const initEscrowIx = new TransactionInstruction({
    programId: programID,
    keys: [
     
      { pubkey: owner, isSigner: true, isWritable: false },

      { pubkey: client_seed, isSigner: false, isWritable: false },

      { pubkey: game_state_account[0], isSigner: false, isWritable: true },
      
      { pubkey: platform_state_account[0], isSigner: false, isWritable: true },

      { pubkey: mint_state[0], isSigner: false, isWritable: true },

      { pubkey: collection_state_account[0], isSigner: false, isWritable: true },

      { pubkey: collection_mint, isSigner: false, isWritable: true },

      { pubkey: SystemProgram.programId, isSigner: false, isWritable: true },


    ],
    data: Buffer.from(Uint8Array.of(2, 
  
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


  console.log(`collection state created ***********************  \n`);
};
