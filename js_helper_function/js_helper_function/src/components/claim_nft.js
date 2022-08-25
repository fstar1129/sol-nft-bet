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
import {
  programID,
  collection_mint,
  platform_data_account,
  mint_state,
  treasury_accoun,
} from "./ids";
import { getOrCreateAssociatedAccount } from "./getOrCreateAssociatedAccount";
import { MINT_LAYOUT } from "./utils";

const BN = require("bn.js");

export const claim_nft = async (user, flore_prise) => {
 

  const collection_state_account = await PublicKey.findProgramAddress(
    [Buffer.from("collection"), collection_mint.toBuffer()],
    programID
  );

  const game_state_account = await PublicKey.findProgramAddress(
    [Buffer.from("0"), collection_state_account[0].toBuffer()],
    programID
  );

  const mint_state = await PublicKey.findProgramAddress(
    [Buffer.from("mints"), game_state_account[0].toBuffer()],
    programID
  );

  const mint_state_account = await connection.getAccountInfo(mint_state[0]);

  const data = MINT_LAYOUT.decode(mint_state_account.data);

  const mint1 = new PublicKey(data.mint1);
  const mint2 = new PublicKey(data.mint2);
  const mint3 = new PublicKey(data.mint3);
  const mint4 = new PublicKey(data.mint4);
  const mint5 = new PublicKey(data.mint5);

  // const mint6=new PublicKey(data.mint6).toString();

  const mint_array = [mint1, mint2, mint3, mint4, mint5];

  console.log(mint_array);

  for (var i = 0; i < mint_array.length; i++) {
    const nft_mint = mint_array[i];

    const game_token_account = await getOrCreateAssociatedAccount(
      game_state_account[0],
      nft_mint,
      user
    );
    const user_token_account = await getOrCreateAssociatedAccount(
      user,
      nft_mint,
      user
    );

    const initEscrowIx = new TransactionInstruction({
      programId: programID,
      keys: [
        { pubkey: user, isSigner: true, isWritable: false },

        { pubkey: user_token_account, isSigner: false, isWritable: true },

        { pubkey: game_state_account[0], isSigner: false, isWritable: true },

        {
          pubkey: collection_state_account[0],
          isSigner: false,
          isWritable: true,
        },

        { pubkey: game_token_account, isSigner: false, isWritable: true },

        { pubkey: collection_mint, isSigner: false, isWritable: true },

        { pubkey: treasury_accoun, isSigner: false, isWritable: true },

        { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },

        { pubkey: platform_data_account, isSigner: false, isWritable: true },
      ],
      data: Buffer.from(
        Uint8Array.of(5, ...new BN(flore_prise).toArray("le",8))
      ),
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
  }
};
