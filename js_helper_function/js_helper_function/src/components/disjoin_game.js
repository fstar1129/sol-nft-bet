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
  game_state_account,
  mint_state,
  collection_state,
  treasury_accoun,
  platform_data_account,
} from "./ids";
import { getOrCreateAssociatedAccount } from "./getOrCreateAssociatedAccount";

const BN = require("bn.js");

export const getTokenAccountFromMint = async (MintPubKey) => {
  const dataFromChain = await connection.getTokenLargestAccounts(
    new PublicKey(MintPubKey)
  );
  const tokenAccount = dataFromChain.value.filter((a) => a.amount === "1")[0]
    .address;
  return tokenAccount.toString();
};

export const disjoin_game = async (user, mint) => {
  const user_token_account = await getOrCreateAssociatedAccount(
    user,
    mint,
    user
  );

  const game_token_account = await getOrCreateAssociatedAccount(
    game_state_account,
    mint,
    user
  );

  console.log("Platform State PDA", platform_data_account.toString());
  console.log("Collection State PDA ", game_state_account.toString());
  console.log("Game State PDA", game_state_account.toString());
  console.log("Mint State PDA", mint_state.toString());

  const floor_price = new BN(1 * 1000000000).toArray("le",8);

  const initEscrowIx = new TransactionInstruction({
    programId: programID,
    keys: [
      { pubkey: user, isSigner: true, isWritable: false },

      { pubkey: user_token_account, isSigner: false, isWritable: true },

      { pubkey: game_state_account, isSigner: false, isWritable: true },

      { pubkey: mint_state, isSigner: false, isWritable: true },

      { pubkey: collection_state, isSigner: false, isWritable: false },

      { pubkey: game_token_account, isSigner: false, isWritable: true },

      { pubkey: treasury_accoun, isSigner: false, isWritable: true },

      { pubkey: TOKEN_PROGRAM_ID, isSigner: false, isWritable: false },

      { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
    ],
    data: Buffer.from(Uint8Array.of(4, ...floor_price)),
  });

  await sendTxUsingExternalSignature(
    [initEscrowIx],
    connection,
    null,
    [],
    new PublicKey(user)
  );
  await new Promise((resolve) => setTimeout(resolve, 2000));
};
