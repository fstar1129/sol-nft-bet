//* Lending protocol state
use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};
use solana_program::{
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
};
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Gamestruct {
    pub is_initialized: bool,
    pub collection_mint: Pubkey,
    pub player1_pubkey: Pubkey,
    pub player1_joined: bool,
    pub player2_pubkey: Pubkey,
    pub player2_joined: bool,
    pub player3_pubkey: Pubkey,
    pub player3_joined: bool,
    pub player4_pubkey: Pubkey,
    pub player4_joined: bool,
    pub player5_pubkey: Pubkey,
    pub player5_joined: bool,
    pub player6_pubkey: Pubkey,
    pub player6_joined: bool,
    pub winner_pubkey: Pubkey,
    pub client_seed: Pubkey,
    pub counter: u64,
    pub status: u64,
    // pub hash:[u8 ;64],
    pub random_no: u64,
    pub nonce: u64,
    pub previous_game_nonce: u64,

}
impl Sealed for Gamestruct {}
impl IsInitialized for Gamestruct {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}
impl Pack for Gamestruct {
    const LEN: usize = 335;
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, Gamestruct::LEN];
        let (
            is_initialized,
            collection_mint,
            player1_pubkey,
            player1_joined,
            player2_pubkey,
            player2_joined,
            player3_pubkey,
            player3_joined,
            player4_pubkey,
            player4_joined,
            player5_pubkey,
            player5_joined,
            player6_pubkey,
            player6_joined,
            winner_pubkey,
            client_seed,
            counter,
            status,
            // hash,
            random_no,
            nonce,
            previous_game_nonce
        ) = array_refs![
            src, 1, 32, 32, 1, 32, 1, 32, 1, 32, 1, 32, 1, 32, 1, 32, 32, 8, 8, /*64,*/ 8, 8,8
        ];
        let is_initialized = match is_initialized {
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };
        Ok(Gamestruct {
            is_initialized,

            collection_mint: Pubkey::new_from_array(*collection_mint),

            player1_pubkey: Pubkey::new_from_array(*player1_pubkey),

            player1_joined: match player1_joined {
                [0] => false,
                [1] => true,
                _ => return Err(ProgramError::InvalidAccountData),
            },

            player2_pubkey: Pubkey::new_from_array(*player2_pubkey),

            player2_joined: match player2_joined {
                [0] => false,
                [1] => true,
                _ => return Err(ProgramError::InvalidAccountData),
            },

            player3_pubkey: Pubkey::new_from_array(*player3_pubkey),

            player3_joined: match player3_joined {
                [0] => false,
                [1] => true,
                _ => return Err(ProgramError::InvalidAccountData),
            },

            player4_pubkey: Pubkey::new_from_array(*player4_pubkey),

            player4_joined: match player4_joined {
                [0] => false,
                [1] => true,
                _ => return Err(ProgramError::InvalidAccountData),
            },

            player5_pubkey: Pubkey::new_from_array(*player5_pubkey),

            player5_joined: match player5_joined {
                [0] => false,
                [1] => true,
                _ => return Err(ProgramError::InvalidAccountData),
            },

            player6_pubkey: Pubkey::new_from_array(*player6_pubkey),

            player6_joined: match player6_joined {
                [0] => false,
                [1] => true,
                _ => return Err(ProgramError::InvalidAccountData),
            },

            winner_pubkey: Pubkey::new_from_array(*winner_pubkey),

            client_seed: Pubkey::new_from_array(*client_seed),

            counter: u64::from_le_bytes(*counter),

            status: u64::from_le_bytes(*status),

            // hash: *hash,
            random_no: u64::from_le_bytes(*random_no),

            nonce: u64::from_le_bytes(*nonce),
            previous_game_nonce: u64::from_le_bytes(*previous_game_nonce),

        })
    }
    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, Gamestruct::LEN];
        let (
            is_initialized_dst,
            collection_mint_dst,
            player1_pubkey_dst,
            player1_joined_dst,
            player2_pubkey_dst,
            player2_joined_dst,
            player3_pubkey_dst,
            player3_joined_dst,
            player4_pubkey_dst,
            player4_joined_dst,
            player5_pubkey_dst,
            player5_joined_dst,
            player6_pubkey_dst,
            player6_joined_dst,
            winner_pubkey_dst,
            client_seed_dst,
            counter_dst,
            status_dst,
            // hash_dst,
            random_no_dst,
            nonce_dst,
            previous_game_nonce_dst
        ) = mut_array_refs![
            dst, 1, 32, 32, 1, 32, 1, 32, 1, 32, 1, 32, 1, 32, 1, 32, 32, 8, 8, /*64,*/ 8, 8,8
        ];
        let Gamestruct {
            is_initialized,
            collection_mint,
            player1_pubkey,
            player1_joined,
            player2_pubkey,
            player2_joined,
            player3_pubkey,
            player3_joined,
            player4_pubkey,
            player4_joined,
            player5_pubkey,
            player5_joined,
            player6_pubkey,
            player6_joined,
            winner_pubkey,
            client_seed,
            counter,
            status,
            // hash,
            random_no,
            nonce,
            previous_game_nonce
        } = self;
        is_initialized_dst[0] = *is_initialized as u8;

        collection_mint_dst.copy_from_slice(collection_mint.as_ref());

        player1_pubkey_dst.copy_from_slice(player1_pubkey.as_ref());

        player1_joined_dst[0] = *player1_joined as u8;

        player2_pubkey_dst.copy_from_slice(player2_pubkey.as_ref());

        player2_joined_dst[0] = *player2_joined as u8;

        player3_pubkey_dst.copy_from_slice(player3_pubkey.as_ref());

        player3_joined_dst[0] = *player3_joined as u8;

        player4_pubkey_dst.copy_from_slice(player4_pubkey.as_ref());

        player4_joined_dst[0] = *player4_joined as u8;

        player5_pubkey_dst.copy_from_slice(player5_pubkey.as_ref());

        player5_joined_dst[0] = *player5_joined as u8;

        player6_pubkey_dst.copy_from_slice(player6_pubkey.as_ref());

        player6_joined_dst[0] = *player6_joined as u8;

        winner_pubkey_dst.copy_from_slice(winner_pubkey.as_ref());
        client_seed_dst.copy_from_slice(client_seed.as_ref());

        *counter_dst = counter.to_le_bytes();
        *status_dst = status.to_le_bytes();
        // *hash_dst = *hash;

        *random_no_dst = random_no.to_le_bytes();
        *nonce_dst = nonce.to_le_bytes();
        *previous_game_nonce_dst=previous_game_nonce.to_le_bytes();
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct PlatformData {
    pub is_initialized: bool,
    pub server_seed: Pubkey,
    pub nonce: u64,
    pub treasury_pubkey: Pubkey,
    pub platform_fees: u64,
}
impl Sealed for PlatformData {}
impl IsInitialized for PlatformData {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for PlatformData {
    const LEN: usize = 81;
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, PlatformData::LEN];
        let (is_initialized, server_seed, nonce, treasury_pubkey, platform_fees) =
            array_refs![src, 1, 32, 8, 32, 8];
        let is_initialized = match is_initialized {
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };
        Ok(PlatformData {
            is_initialized,
            server_seed: Pubkey::new_from_array(*server_seed),
            nonce: u64::from_le_bytes(*nonce),
            treasury_pubkey: Pubkey::new_from_array(*treasury_pubkey),
            platform_fees: u64::from_le_bytes(*platform_fees),
        })
    }
    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, PlatformData::LEN];
        let (
            is_initialized_dst,
            server_seed_dst,
            nonce_dst,
            treasury_pubkey_dst,
            platform_fees_dst,
        ) = mut_array_refs![dst, 1, 32, 8, 32, 8];
        let PlatformData {
            is_initialized,
            server_seed,
            nonce,
            treasury_pubkey,
            platform_fees,
        } = self;
        is_initialized_dst[0] = *is_initialized as u8;
        server_seed_dst.copy_from_slice(server_seed.as_ref());

        *nonce_dst = nonce.to_le_bytes();
        treasury_pubkey_dst.copy_from_slice(treasury_pubkey.as_ref());
        *platform_fees_dst = platform_fees.to_le_bytes();
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct CollectionData {
    pub is_initialized: bool,
    pub collection_mint: Pubkey,
    pub nonce: u64,
}
impl Sealed for CollectionData {}
impl IsInitialized for CollectionData {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for CollectionData {
    const LEN: usize = 41;
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, CollectionData::LEN];
        let (is_initialized, collection_mint, nonce) = array_refs![src, 1, 32, 8];
        let is_initialized = match is_initialized {
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };
        Ok(CollectionData {
            is_initialized,
            collection_mint: Pubkey::new_from_array(*collection_mint),
            nonce: u64::from_le_bytes(*nonce),
        })
    }
    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, CollectionData::LEN];
        let (is_initialized_dst, server_seed_dst, nonce_dst) = mut_array_refs![dst, 1, 32, 8];
        let CollectionData {
            is_initialized,
            collection_mint,
            nonce,
        } = self;
        is_initialized_dst[0] = *is_initialized as u8;
        server_seed_dst.copy_from_slice(collection_mint.as_ref());

        *nonce_dst = nonce.to_le_bytes();
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Mints {
    pub is_initialized: bool,
    pub mint1: Pubkey,
    pub mint2: Pubkey,
    pub mint3: Pubkey,
    pub mint4: Pubkey,
    pub mint5: Pubkey,
    pub mint6: Pubkey,
}
impl Sealed for Mints {}
impl IsInitialized for Mints {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for Mints {
    const LEN: usize = 193;
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, Mints::LEN];
        let (is_initialized, mint1, mint2, mint3, mint4, mint5, mint6) =
            array_refs![src, 1, 32, 32, 32, 32, 32, 32];
        let is_initialized = match is_initialized {
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };
        Ok(Mints {
            is_initialized,
            mint1: Pubkey::new_from_array(*mint1),
            mint2: Pubkey::new_from_array(*mint2),
            mint3: Pubkey::new_from_array(*mint3),
            mint4: Pubkey::new_from_array(*mint4),
            mint5: Pubkey::new_from_array(*mint5),
            mint6: Pubkey::new_from_array(*mint6),
        })
    }
    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, Mints::LEN];
        let (is_initialized_dst, mint1_dst, mint2_dst, mint3_dst, mint4_dst, mint5_dst, mint6_dst) =
            mut_array_refs![dst, 1, 32, 32, 32, 32, 32, 32];
        let Mints {
            is_initialized,
            mint1,
            mint2,
            mint3,
            mint4,
            mint5,
            mint6,
        } = self;
        is_initialized_dst[0] = *is_initialized as u8;
        mint1_dst.copy_from_slice(mint1.as_ref());
        mint2_dst.copy_from_slice(mint2.as_ref());
        mint3_dst.copy_from_slice(mint3.as_ref());
        mint4_dst.copy_from_slice(mint4.as_ref());
        mint5_dst.copy_from_slice(mint5.as_ref());
        mint6_dst.copy_from_slice(mint6.as_ref());
    }
}
