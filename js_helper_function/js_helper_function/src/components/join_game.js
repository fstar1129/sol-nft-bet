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
import { programID ,collection_mint, platform_data_account} from "./ids";
import { getOrCreateAssociatedAccount} from "./getOrCreateAssociatedAccount";


const BN = require("bn.js");


export  const getTokenAccountFromMint = async (MintPubKey) => {
    const dataFromChain = await connection.getTokenLargestAccounts(
      new PublicKey(MintPubKey),
    );
    const tokenAccount = dataFromChain.value.filter((a) => a.amount === '1')[0]
      .address;
    return tokenAccount.toString();
  };

export const join_game = async (user,mint) => {

    const tokenAccount = await getTokenAccountFromMint(mint);
  
    const user_token_account = tokenAccount;
  
    const Nft_mint = new PublicKey(mint);

  const collection_state_account = await PublicKey.findProgramAddress(
    [
      Buffer.from("collection"),
      collection_mint.toBuffer(),

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
  console.log("Game State PDA", game_state_account[0].toString());
  console.log("Mint State PDA", mint_state[0].toString());

  const game_token_account = await getOrCreateAssociatedAccount(
    game_state_account[0],
    Nft_mint,
    user,
  );

  const initEscrowIx = new TransactionInstruction({
    programId: programID,
    keys: [
     
      { pubkey: user, isSigner: true, isWritable: false },

      { pubkey: user_token_account, isSigner: false, isWritable: true },

      { pubkey: Nft_mint, isSigner: false, isWritable: true },

      { pubkey: collection_mint, isSigner: false, isWritable: true },

      { pubkey: game_state_account[0], isSigner: false, isWritable: true },
      
      { pubkey: mint_state[0], isSigner: false, isWritable: true },

      { pubkey: collection_state_account[0], isSigner: false, isWritable: true },

      { pubkey: game_token_account, isSigner: false, isWritable: true },

      { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },

      { pubkey: platform_data_account, isSigner: false, isWritable: true },


    ],
    data: Buffer.from(Uint8Array.of(3, 
  
     )),
        
      
  });

  await sendTxUsingExternalSignature(
    [initEscrowIx],
    connection,
    null,
    [],
    new PublicKey(user)
  );
  await new Promise((resolve) => setTimeout(resolve, 2000));


  console.log(`collection state created ***********************  \n`);
};
