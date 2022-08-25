import * as BufferLayout from '@solana/buffer-layout';
import * as Layout from './layout';
import { AccountInfo, clusterApiUrl, Connection, PublicKey } from '@solana/web3.js';
import { Numberu128, Numberu64 } from './number';
import { programID, game_state_account, platform_data_account, collection_state, mint_state } from './ids';

export const GameState = BufferLayout.struct([
    BufferLayout.u8('is_initialized'),
    Layout.publicKey('collection_mint'),
    Layout.publicKey('player1_pubkey'),
    BufferLayout.u8('player1_joined'),
    Layout.publicKey('player2_pubkey'),
    BufferLayout.u8('player2_joined'),
    Layout.publicKey('player3_pubkey'),
    BufferLayout.u8('player3_joined'),
    Layout.publicKey('player4_pubkey'),
    BufferLayout.u8('player4_joined'),
    Layout.publicKey('player5_pubkey'),
    BufferLayout.u8('player5_joined'),
    Layout.publicKey('player6_pubkey'),
    BufferLayout.u8('player6_joined'),
    Layout.publicKey('winner_pubkey'),
    Layout.publicKey('client_seed'),
    Layout.uint64('counter'),
    Layout.uint64('status'),
    Layout.uint64('random_number'),
    Layout.uint64('nonce'),
]);

export const PlatformState = BufferLayout.struct([
    BufferLayout.u8('is_initialized'),
    Layout.publicKey('server_seed'),
    Layout.uint64('nonce'),
    Layout.publicKey('treasury_pubkey'),
    Layout.uint64('platform_fees'),
])

export const CollectionDataState = BufferLayout.struct([
    BufferLayout.u8('is_initialized'),
    Layout.publicKey('collection_mint'),
    Layout.uint64('nonce'),
])

export const MintsState = BufferLayout.struct([
    BufferLayout.u8('is_initialized'),
    Layout.publicKey('mint1'),
    Layout.publicKey('mint2'),
    Layout.publicKey('mint3'),
    Layout.publicKey('mint4'),
    Layout.publicKey('mint5'),
    Layout.publicKey('mint6'),
])

export const decodeGameState = async () => {
    const connection = new Connection(clusterApiUrl('devnet'), 'confirmed');

    const stateAccount = await connection.getAccountInfo(game_state_account);

    if(stateAccount == null) {
        console.log("State Account not found");
        process.exit(-1);
    }

    if(!stateAccount.owner.equals(programID)) {
        console.log("State Account not associated with program");
        process.exit(-2);
    }

    const decodedData = GameState.decode(stateAccount.data);

    return {
        is_initialized: decodedData.is_initialized,
        collection_mint: new PublicKey(decodedData.collection_mint),
        player1_pubkey: new PublicKey(decodedData.player1_pubkey),
        player1_joined: decodedData.player1_joined,
        player2_pubkey: new PublicKey(decodedData.player2_pubkey),
        player2_joined: decodedData.player2_joined,
        player3_pubkey: new PublicKey(decodedData.player3_pubkey),
        player3_joined: decodedData.player3_joined,
        player4_pubkey: new PublicKey(decodedData.player4_pubkey),
        player4_joined: decodedData.player4_joined,
        player5_pubkey: new PublicKey(decodedData.player5_pubkey),
        player5_joined: decodedData.player5_joined,
        player6_pubkey: new PublicKey(decodedData.player6_pubkey),
        player6_joined: decodedData.player6_joined,
        winner_pubkey: new PublicKey(decodedData.winner_pubkey),
        client_seed: new PublicKey(decodedData.client_seed),
        counter: Numberu64.fromBuffer(decodedData.counter),
        status: Numberu64.fromBuffer(decodedData.status),
        random_number: Numberu64.fromBuffer(decodedData.random_number),
        nonce: Numberu64.fromBuffer(decodedData.nonce),
    }
}


export const decodePlatformState = async () => {
    const connection = new Connection(clusterApiUrl('devnet'), 'confirmed');

    const stateAccount = await connection.getAccountInfo(platform_data_account);

    if(stateAccount == null) {
        console.log("State Account not found");
        process.exit(-1);
    }

    if(!stateAccount.owner.equals(programID)) {
        console.log("State Account not associated with program");
        process.exit(-2);
    }

    const decodedData = PlatformState.decode(stateAccount.data);

    return {
        is_initialized: decodedData.is_initialized,
        server_seed: new PublicKey(decodedData.server_seed),
        nonce: Numberu64.fromBuffer(decodedData.nonce),
        treasury_pubkey: new PublicKey(decodedData.treasury_pubkey),
        platform_fees: Numberu64.fromBuffer(decodedData.platform_fees),
    }
}

export const decodeCollectionData = async () => {
    const connection = new Connection(clusterApiUrl('devnet'), 'confirmed');

    const stateAccount = await connection.getAccountInfo(collection_state); 

    if(stateAccount == null) {
        console.log("State Account not found");
        process.exit(-1);
    }

    if(!stateAccount.owner.equals(programID)) {
        console.log("State Account not associated with program");
        process.exit(-2);
    }

    const decodedData = CollectionDataState.decode(stateAccount.data);

    return {
        is_initialized: decodedData.is_initialized,
        collection_mint: new PublicKey(decodedData.collection_mint),
        nonce: Numberu64.fromBuffer(decodedData.nonce)
    }
}


export const decodeMintsState = async () => {
    const connection = new Connection(clusterApiUrl('devnet'), 'confirmed');

    const stateAccount = await connection.getAccountInfo(mint_state); 

    if(stateAccount == null) {
        console.log("State Account not found");
        process.exit(-1);
    }

    if(!stateAccount.owner.equals(programID)) {
        console.log("State Account not associated with program");
        process.exit(-2);
    }

    const decodedData = MintsState.decode(stateAccount.data);

    return {
        is_initialized: decodedData.is_initialized,
        mint1: new PublicKey(decodedData.mint1),
        mint2: new PublicKey(decodedData.mint2),
        mint3: new PublicKey(decodedData.mint3),
        mint4: new PublicKey(decodedData.mint4),
        mint5: new PublicKey(decodedData.mint5),
        mint6: new PublicKey(decodedData.mint6)
    }

}


export const print_states = async () => {
    const platform_data = await decodePlatformState();
    console.log("************************ Platform State")
    console.log("Is Initiaized", platform_data.is_initialized.toString());
    console.log("Server Seed", platform_data.server_seed.toString());
    console.log("Nonce", platform_data.nonce.toString());
    console.log("Treasury Pubkey", platform_data.treasury_pubkey.toString());
    console.log("Platform Fees", platform_data.platform_fees.toString());
    console.log("\n")

    const game_data = await decodeGameState();
    console.log("************************ Game State")
    console.log("Is Initiaized", game_data.is_initialized.toString());
    console.log("Collection Mint", game_data.collection_mint.toString());
    console.log("Player 1 Pubkey", game_data.player1_pubkey.toString());
    console.log("Player 1 Joined", game_data.player1_joined.toString());
    console.log("Player 2 Pubkey", game_data.player2_pubkey.toString());
    console.log("Player 2 Joined", game_data.player2_joined.toString());
    console.log("Player 3 Pubkey", game_data.player3_pubkey.toString());
    console.log("Player 3 Joined", game_data.player3_joined.toString());
    console.log("Player 4 Pubkey", game_data.player4_pubkey.toString());
    console.log("Player 4 Joined", game_data.player4_joined.toString());
    console.log("Player 5 Pubkey", game_data.player5_pubkey.toString());
    console.log("Player 5 Joined", game_data.player5_joined.toString());
    console.log("Player 6 Pubkey", game_data.player6_pubkey.toString());
    console.log("Player 6 Joined", game_data.player6_joined.toString());
    console.log("Winner Pubkey", game_data.winner_pubkey.toString());
    console.log("Client Seed", game_data.client_seed.toString());
    console.log("Counter", game_data.counter.toString());
    console.log("Status", game_data.status.toString());
    console.log("Random Number", game_data.random_number.toString());
    console.log("Nonce", game_data.nonce.toString());
    console.log("\n");

    const collection_data = await decodeCollectionData();
    console.log("************************ Collection Data State")
    console.log("Is Initialized", collection_data.is_initialized.toString());
    console.log("Collection Mint", collection_data.collection_mint.toString());
    console.log("Nonce", collection_data.nonce.toString());
    console.log("\n");

    const mint_data = await decodeMintsState();
    console.log("************************ Mint State")
    console.log("Is Initialized", mint_data.is_initialized.toString());
    console.log("Mint 1", mint_data.mint1.toString());
    console.log("Mint 2", mint_data.mint2.toString());
    console.log("Mint 3", mint_data.mint3.toString());
    console.log("Mint 4", mint_data.mint4.toString());
    console.log("Mint 5", mint_data.mint5.toString());
    console.log("Mint 6", mint_data.mint6.toString());
    console.log("\n");
}