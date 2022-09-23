//! Program state processor

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
    system_instruction,
    system_instruction::create_account,
    sysvar::rent::Rent,
};

use crate::{
    error::BettingError,
    instruction::BettingInstruction,
    state::{CollectionData, Gamestruct, Mints, PlatformData},
};

use sha2::{Digest, Sha256};
pub struct Processor;

//* Program state handler.
impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = BettingInstruction::unpack(instruction_data)?;
        match instruction {
            BettingInstruction::InitPlatformData { args } => {
                msg!("Instruction: PlatformData initialize");
                Self::process_init_platform(accounts, program_id, args)
            }
            BettingInstruction::CreateCollection {} => {
                msg!("Instruction:Create new collection");
                Self::process_collection_state(accounts, program_id)
            }
            BettingInstruction::CreateGame {} => {
                msg!("Instruction: Create Game instruction");
                Self::process_crate_game(accounts, program_id)
            }
            BettingInstruction::JoinGame {} => {
                msg!("Instruction:Join Game instruction");
                Self::process_join_game(accounts, program_id)
            }

            BettingInstruction::DisJoinGame { amount } => {
                msg!("Instruction:Disjoin Game instruction");
                Self::process_disjoin_game(accounts, program_id, amount)
            }

            BettingInstruction::ClaimReward { amount } => {
                msg!("Instruction:claim");
                Self::process_claim(accounts, program_id, amount)
            }
        }
    }

    //* Process initialized platform
    //* @args: Nonce ,Server seed.
    pub fn process_init_platform(
        accounts: &[AccountInfo],
        program_id: &Pubkey,
        args: (u64, u64),
    ) -> ProgramResult {
        //*Required program accounts info*/
        let account_info_iter = &mut accounts.iter();

        let owner_account = next_account_info(account_info_iter)?;

        let server_seed = next_account_info(account_info_iter)?;

        let treasury_account = next_account_info(account_info_iter)?;

        let platform_data_account = next_account_info(account_info_iter)?;

        let system_program_id = next_account_info(account_info_iter)?;

        let pda_prefix = "betting_contract";

        let platform_seed = &[pda_prefix.as_bytes()];

        let (platform_pda, nonce) = Pubkey::find_program_address(platform_seed, program_id);

        if *platform_data_account.key != platform_pda {
            return Err(ProgramError::InvalidAccountData);
        }
        //* Create a new platform state to store the Server seed and Nonce*/
        if platform_data_account.owner != program_id {
            invoke_signed(
                &create_account(
                    owner_account.key,
                    platform_data_account.key,
                    Rent::default().minimum_balance(PlatformData::LEN),
                    PlatformData::LEN as u64,
                    program_id,
                ),
                &[
                    owner_account.clone(),
                    platform_data_account.clone(),
                    system_program_id.clone(),
                ],
                &[&[pda_prefix.as_bytes(), &[nonce]]],
            )?;
        };

        //* Unpack the platform state and store the data*/
        let mut platform_data =
            PlatformData::unpack_unchecked(&platform_data_account.try_borrow_data()?)?;

        platform_data.is_initialized = true;
        platform_data.server_seed = *server_seed.key;
        platform_data.nonce = args.0;
        platform_data.platform_fees = args.1;
        platform_data.treasury_pubkey = *treasury_account.key;

        PlatformData::pack(
            platform_data,
            &mut platform_data_account.try_borrow_mut_data()?,
        )?;
        msg!("platform_data: {:?}", platform_data);

        Ok(())
    }

    //* crate collection_state_account
    pub fn process_collection_state(
        accounts: &[AccountInfo],
        program_id: &Pubkey,
    ) -> ProgramResult {
        //*Required program accounts info*/
        let account_info_iter = &mut accounts.iter();

        let admin_account = next_account_info(account_info_iter)?;

        let collection_state = next_account_info(account_info_iter)?;

        let collection_mint = next_account_info(account_info_iter)?;

        let system_program_id = next_account_info(account_info_iter)?;

        let collection_prefix = "collection";

        let collection_seed_seed = &[collection_prefix.as_bytes(), (collection_mint.key).as_ref()];

        let (collection_pda, nonce) =
            Pubkey::find_program_address(collection_seed_seed, program_id);

        if *collection_state.key != collection_pda {
            return Err(ProgramError::InvalidAccountData);
        }
        //* Create a new platform state to store the Server seed and Nonce*/
        if collection_state.owner != program_id {
            invoke_signed(
                &create_account(
                    admin_account.key,
                    collection_state.key,
                    Rent::default().minimum_balance(CollectionData::LEN),
                    CollectionData::LEN as u64,
                    program_id,
                ),
                &[
                    admin_account.clone(),
                    collection_state.clone(),
                    system_program_id.clone(),
                ],
                &[&[
                    collection_prefix.as_bytes(),
                    (collection_mint.key).as_ref(),
                    &[nonce],
                ]],
            )?;
        };

        //* Unpack the platform state and store the data*/
        let mut collection_data =
            CollectionData::unpack_unchecked(&collection_state.try_borrow_data()?)?;

        collection_data.is_initialized = true;
        collection_data.collection_mint = *collection_mint.key;
        collection_data.nonce = 0;

        CollectionData::pack(
            collection_data,
            &mut collection_state.try_borrow_mut_data()?,
        )?;

        msg!("collection data: {:?}", collection_data);

        Ok(())
    }
    //* Process crate Game instruction
    //* @param client_seed: client_seed.
    pub fn process_crate_game(accounts: &[AccountInfo], program_id: &Pubkey) -> ProgramResult {
        //* Required program accounts */
        let account_info_iter = &mut accounts.iter();

        let admin_account = next_account_info(account_info_iter)?;

        let client_seed = next_account_info(account_info_iter)?;

        if !admin_account.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let game_state_account = next_account_info(account_info_iter)?;

        let platform_state = next_account_info(account_info_iter)?;

        let mint_state = next_account_info(account_info_iter)?;

        let collection_state = next_account_info(account_info_iter)?;

        let collection_mint = next_account_info(account_info_iter)?;

        let system_program_id = next_account_info(account_info_iter)?;

        let collection_prefix = "collection";

        let collection_seed_seed = &[collection_prefix.as_bytes(), (collection_mint.key).as_ref()];

        let (collection_pda, _nonce) =
            Pubkey::find_program_address(collection_seed_seed, program_id);

        if *collection_state.key != collection_pda {
            return Err(ProgramError::InvalidAccountData);
        }
        let collection_data =
            CollectionData::unpack_unchecked(&collection_state.try_borrow_data()?)?;

        let pda_prefix = "betting_contract";

        let platform_seed = &[pda_prefix.as_bytes()];

        let (platform_pda, _nonce) = Pubkey::find_program_address(platform_seed, program_id);

        if *platform_state.key != platform_pda {
            return Err(ProgramError::InvalidAccountData);
        }

        let game_prefix = collection_data.nonce.to_string();

        let game_seed = &[game_prefix.as_bytes(), (collection_state.key).as_ref()];

        let (game_pda, game_nonce) = Pubkey::find_program_address(game_seed, program_id);

        if *game_state_account.key != game_pda {
            return Err(ProgramError::InvalidAccountData);
        }

        if *game_state_account.owner == *program_id {
            return Err(ProgramError::AccountAlreadyInitialized);
        }

        let mint_prefix = "mints";

        let mint_state_seed = &[mint_prefix.as_bytes(), (game_state_account.key).as_ref()];

        let (mint_state_pda, mint_nonce) =
            Pubkey::find_program_address(mint_state_seed, program_id);

        if *mint_state.key != mint_state_pda {
            return Err(ProgramError::InvalidAccountData);
        }

        if game_state_account.owner != program_id {
            invoke_signed(
                &create_account(
                    admin_account.key,
                    game_state_account.key,
                    Rent::default().minimum_balance(Gamestruct::LEN),
                    Gamestruct::LEN as u64,
                    program_id,
                ),
                &[
                    admin_account.clone(),
                    game_state_account.clone(),
                    system_program_id.clone(),
                ],
                &[&[
                    game_prefix.as_bytes(),
                    (collection_state.key).as_ref(),
                    &[game_nonce],
                ]],
            )?;
        };
        let mut game_data = Gamestruct::unpack_unchecked(&game_state_account.try_borrow_data()?)?;

        if mint_state.owner != program_id {
            invoke_signed(
                &create_account(
                    admin_account.key,
                    mint_state.key,
                    Rent::default().minimum_balance(Mints::LEN),
                    Mints::LEN as u64,
                    program_id,
                ),
                &[
                    admin_account.clone(),
                    mint_state.clone(),
                    system_program_id.clone(),
                ],
                &[&[
                    mint_prefix.as_bytes(),
                    (game_state_account.key).as_ref(),
                    &[mint_nonce],
                ]],
            )?;
        };
        let mut mint_data = Mints::unpack_unchecked(&mint_state.try_borrow_data()?)?;
        mint_data.is_initialized = true;

        game_data.is_initialized = true;
        game_data.counter = 0;
        game_data.client_seed = *client_seed.key;
        game_data.status = 1;
        game_data.collection_mint = *collection_mint.key;

        Gamestruct::pack(game_data, &mut game_state_account.try_borrow_mut_data()?)?;

        Mints::pack(mint_data, &mut mint_state.try_borrow_mut_data()?)?;

        msg!("game data: {:?}", game_data);

        Ok(())
    }

    //* Process JoinGame Instruction
    pub fn process_join_game(accounts: &[AccountInfo], program_id: &Pubkey) -> ProgramResult {
        //* Required program accounts*/
        let account_info_iter = &mut accounts.iter();

        let user_account = next_account_info(account_info_iter)?;

        let user_token_account = next_account_info(account_info_iter)?;

        let nft_mint = next_account_info(account_info_iter)?;

        let collection_mint = next_account_info(account_info_iter)?;

        let game_state_account = next_account_info(account_info_iter)?;

        let mint_state = next_account_info(account_info_iter)?;

        let collection_state = next_account_info(account_info_iter)?;

        let game_token_account = next_account_info(account_info_iter)?;

        let token_program = next_account_info(account_info_iter)?;

        let platform_state_account = next_account_info(account_info_iter)?;

        let pda_prefix = "betting_contract";

        let platform_seed = &[pda_prefix.as_bytes()];

        let (platform_pda, _nonce) = Pubkey::find_program_address(platform_seed, program_id);
      

        if *platform_state_account.key != platform_pda {
            return Err(ProgramError::InvalidAccountData);
        }

        let mut platform_data =
            PlatformData::unpack_unchecked(&platform_state_account.try_borrow_data()?)?;

        let collection_prefix = "collection";

        let collection_seed_seed = &[collection_prefix.as_bytes(), (collection_mint.key).as_ref()];

        let (collection_pda, _nonce) =
            Pubkey::find_program_address(collection_seed_seed, program_id);

      
        if *collection_state.key != collection_pda {
            return Err(ProgramError::InvalidAccountData);
        }

        let mut collection_data =
            CollectionData::unpack_unchecked(&collection_state.try_borrow_data()?)?;

     

        let mut game_data = Gamestruct::unpack_unchecked(&game_state_account.try_borrow_data()?)?;

        let mint_prefix = "mints";

        let mint_state_seed = &[mint_prefix.as_bytes(), (game_state_account.key).as_ref()];

        let (mint_state_pda, _mint_nonce) =
            Pubkey::find_program_address(mint_state_seed, program_id);

        if *mint_state.key != mint_state_pda {
            return Err(ProgramError::InvalidAccountData);
        }
        let mut mint_data = Mints::unpack_unchecked(&mint_state.try_borrow_data()?)?;

        //* Transfer NFT from user token account to PDA(contract)
        let tranfer_instructions = spl_token::instruction::transfer(
            token_program.key,
            user_token_account.key,
            game_token_account.key,
            user_account.key,
            &[],
            1,
        )?;

        invoke(
            &tranfer_instructions,
            &[
                user_token_account.clone(),
                game_token_account.clone(),
                user_account.clone(),
                token_program.clone(),
            ],
        )?;

         //Ristrict same user ....
         if *user_account.key == game_data.player1_pubkey {
            return Err(ProgramError::InvalidAccountData);
        } else if *user_account.key == game_data.player2_pubkey{
            return Err(ProgramError::InvalidAccountData);
        } else if *user_account.key == game_data.player3_pubkey{
            return Err(ProgramError::InvalidAccountData);
        } else if *user_account.key == game_data.player4_pubkey{
            return Err(ProgramError::InvalidAccountData);
        } else if *user_account.key == game_data.player5_pubkey{
            return Err(ProgramError::InvalidAccountData);
        } else if *user_account.key == game_data.player6_pubkey{
            return  Err(ProgramError::InvalidAccountData);
        } 

        if game_data.counter < 6 {
            let vacant_place = get_vacant_place(game_data).ok_or(BettingError::GameFull)?;
            match vacant_place {
                0 => {
                    game_data.player1_pubkey = *user_account.key;
                    mint_data.mint1 = *nft_mint.key;
                    game_data.player1_joined = true;
                    game_data.counter += 1;
                }
                1 => {
                    game_data.player2_pubkey = *user_account.key;
                    mint_data.mint2 = *nft_mint.key;
                    game_data.player2_joined = true;
                    game_data.counter += 1;
                }
                2 => {
                    game_data.player3_pubkey = *user_account.key;
                    mint_data.mint3 = *nft_mint.key;
                    game_data.player3_joined = true;
                    game_data.counter += 1;
                }
                3 => {
                    game_data.player4_pubkey = *user_account.key;
                    mint_data.mint4 = *nft_mint.key;
                    game_data.player4_joined = true;
                    game_data.counter += 1;
                }
                4 => {
                    game_data.player5_pubkey = *user_account.key;
                    mint_data.mint5 = *nft_mint.key;
                    game_data.player5_joined = true;
                    game_data.counter += 1;
                }
                5 => {
                    game_data.player6_pubkey = *user_account.key;
                    mint_data.mint6 = *nft_mint.key;
                    game_data.player6_joined = true;
                    game_data.counter += 1;
                }

                _ => todo!(),
            }
        }
       

      
        // todo Add random number logic here
        if game_data.counter == 6 {
            let client_seed = game_data.client_seed.to_string();
            let server_seed = platform_data.server_seed.to_string();
            let nonce = platform_data.nonce;

            let hash = get_hash(&client_seed, &server_seed, nonce);

            let random_no = get_random_no(&hash);
            game_data.random_no = random_no;
            game_data.nonce = nonce;

            let new_nonce = get_new_nonce(&hash);

            platform_data.nonce = new_nonce;

            // game_data.hash = hash.as_bytes().try_into().unwrap();

            match random_no {
                0 => {
                    game_data.winner_pubkey = game_data.player1_pubkey;
                }
                1 => {
                    game_data.winner_pubkey = game_data.player2_pubkey;
                }
                2 => {
                    game_data.winner_pubkey = game_data.player3_pubkey;
                }
                3 => {
                    game_data.winner_pubkey = game_data.player4_pubkey;
                }
                4 => {
                    game_data.winner_pubkey = game_data.player5_pubkey;
                }
                5 => {
                    game_data.winner_pubkey = game_data.player6_pubkey;
                }
                _ => todo!(),
            }

            game_data.status = 2;
            game_data.previous_game_nonce=collection_data.nonce;
            collection_data.nonce += 1;
        }

        Gamestruct::pack(game_data, &mut game_state_account.try_borrow_mut_data()?)?;
        CollectionData::pack(collection_data, &mut collection_state.try_borrow_mut_data()?)?;

        PlatformData::pack(
            platform_data,
            &mut platform_state_account.try_borrow_mut_data()?,
        )?;
        Mints::pack(mint_data, &mut mint_state.try_borrow_mut_data()?)?;

        Ok(())
    }

    pub fn process_disjoin_game(
        accounts: &[AccountInfo],
        program_id: &Pubkey,
        floor_price: u64,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        let user_account = next_account_info(account_info_iter)?;

        let user_token_account = next_account_info(account_info_iter)?;

        let game_state_account = next_account_info(account_info_iter)?;

        let mint_state = next_account_info(account_info_iter)?;

        let collection_state = next_account_info(account_info_iter)?;

        let collection_mint = next_account_info(account_info_iter)?;


        let game_token_account = next_account_info(account_info_iter)?;

        let treasury_account = next_account_info(account_info_iter)?;

        let token_program = next_account_info(account_info_iter)?;

        let system_program_account = next_account_info(account_info_iter)?;

        let mut game_data = Gamestruct::unpack_unchecked(&game_state_account.try_borrow_data()?)?;

        let mint_prefix = "mints";

        let mint_state_seed = &[mint_prefix.as_bytes(), (game_state_account.key).as_ref()];

        let (mint_state_pda, _mint_nonce) =
            Pubkey::find_program_address(mint_state_seed, program_id);

        if *mint_state.key != mint_state_pda {
            return Err(ProgramError::InvalidAccountData);
        }

        let collection_prefix = "collection";

        let collection_seed_seed = &[collection_prefix.as_bytes(), (collection_mint.key).as_ref()];

        let (collection_pda, _nonce) =
            Pubkey::find_program_address(collection_seed_seed, program_id);

        if *collection_state.key != collection_pda {
            return Err(ProgramError::InvalidAccountData);
        }
        let collection_data =
            CollectionData::unpack_unchecked(&collection_state.try_borrow_data()?)?;

        let game_pda_seeds_prefix = collection_data.nonce.to_string();

        let game_pda_seeds = &[
            game_pda_seeds_prefix.as_bytes(),
            collection_state.key.as_ref(),
        ];

        let (_game_pda, game_seeds) = Pubkey::find_program_address(game_pda_seeds, program_id);

        let mut mint_data = Mints::unpack_unchecked(&mint_state.try_borrow_data()?)?;

        let penalty_fee = floor_price
            .checked_mul(3)
            .ok_or(BettingError::MathError)?
            .checked_div(100)
            .ok_or(BettingError::MathError)?;

        let disjoining_user = get_disjoining_player(game_data, *user_account.key)
            .ok_or(BettingError::UnknownPlayer)?;

        let transfer_sol_to_treasury_ix =
            system_instruction::transfer(user_account.key, treasury_account.key, penalty_fee);

        invoke(
            &transfer_sol_to_treasury_ix,
            &[
                user_account.clone(),
                treasury_account.clone(),
                user_account.clone(),
                system_program_account.clone(),
            ],
        )?;

        let transfer_nft_back_to_user_ix = spl_token::instruction::transfer(
            token_program.key,
            game_token_account.key,
            user_token_account.key,
            game_state_account.key,
            &[],
            1,
        )?;

        invoke_signed(
            &transfer_nft_back_to_user_ix,
            &[
                game_token_account.clone(),
                user_token_account.clone(),
                game_state_account.clone()
            ],
            &[&[
                game_pda_seeds_prefix.as_bytes(),
                (collection_state.key.as_ref()),
                &[game_seeds],
            ]],
        )?;

        if game_data.counter > 0 {
            if game_data.status == 1 {
                match disjoining_user {
                    0 => {
                        game_data.player1_pubkey = Pubkey::default();
                        mint_data.mint1 = Pubkey::default();
                        game_data.player1_joined = false;
                        game_data.counter -= 1;
                    }
                    1 => {
                        game_data.player2_pubkey = Pubkey::default();
                        mint_data.mint2 = Pubkey::default();
                        game_data.player2_joined = false;
                        game_data.counter -= 1;
                    }
                    2 => {
                        game_data.player3_pubkey = Pubkey::default();
                        mint_data.mint3 = Pubkey::default();
                        game_data.player3_joined = false;
                        game_data.counter -= 1;
                    }
                    3 => {
                        game_data.player4_pubkey = Pubkey::default();
                        mint_data.mint4 = Pubkey::default();
                        game_data.player4_joined = false;
                        game_data.counter -= 1;
                    }
                    4 => {
                        game_data.player5_pubkey = Pubkey::default();
                        mint_data.mint5 = Pubkey::default();
                        game_data.player5_joined = false;
                        game_data.counter -= 1;
                    }
                    5 => {
                        game_data.player6_pubkey = Pubkey::default();
                        mint_data.mint6 = Pubkey::default();
                        game_data.player6_joined = false;
                        game_data.counter -= 1;
                    }
                    _ => todo!(),
                }
            } else {
                return Err(BettingError::GameEnded.into());
            }
        }

        Gamestruct::pack(game_data, &mut game_state_account.try_borrow_mut_data()?)?;
        Mints::pack(mint_data, &mut mint_state.try_borrow_mut_data()?)?;

        msg!("Game State: {:?}", game_data);
        msg!("Mints State {:?}", mint_data);

        Ok(())
    }

    //* Process claimReward Instruction.
    //* @param client_seed: client_seed.
    pub fn process_claim(
        accounts: &[AccountInfo],
        program_id: &Pubkey,
        flore_prise: u64,
    ) -> ProgramResult {
        //* Required program accounts*/
        let account_info_iter = &mut accounts.iter();

        let user_account = next_account_info(account_info_iter)?;

        if !user_account.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
        let user_token_account = next_account_info(account_info_iter)?;

        let game_state_account = next_account_info(account_info_iter)?;

        let collection_state_account = next_account_info(account_info_iter)?;

        let game_token_account = next_account_info(account_info_iter)?;

        let collection_mint = next_account_info(account_info_iter)?;

        let treasury_account = next_account_info(account_info_iter)?;

        let token_program = next_account_info(account_info_iter)?;

        let platform_state_account = next_account_info(account_info_iter)?;
        let system_program = next_account_info(account_info_iter)?;


        let pda_prefix = "betting_contract";

        let platform_seed = &[pda_prefix.as_bytes()];

        let (platform_pda, _nonce) = Pubkey::find_program_address(platform_seed, program_id);

        if *platform_state_account.key != platform_pda {
            return Err(ProgramError::InvalidAccountData);
        }

        let platform_data =
            PlatformData::unpack_unchecked(&platform_state_account.try_borrow_data()?)?;

        if *treasury_account.key != platform_data.treasury_pubkey {
            return Err(ProgramError::InvalidAccountData);
        }

        let collection_prefix = "collection";

        let collection_seed_seed = &[collection_prefix.as_bytes(), (collection_mint.key).as_ref()];

        let (collection_pda, _nonce) =
            Pubkey::find_program_address(collection_seed_seed, program_id);

        if *collection_state_account.key != collection_pda {
            return Err(ProgramError::InvalidAccountData);
        }

        let collection_data =
            CollectionData::unpack_unchecked(&collection_state_account.try_borrow_data()?)?;

        let game_data = Gamestruct::unpack_unchecked(&game_state_account.try_borrow_data()?)?;


        let game_prefix = game_data.previous_game_nonce.to_string();

        let game_seed = &[
            game_prefix.as_bytes(),
            (collection_state_account.key).as_ref(),
        ];

        let (game_pda, game_nonce) = Pubkey::find_program_address(game_seed, program_id);

        if *game_state_account.key != game_pda {
            return Err(ProgramError::InvalidAccountData);
        }


        //* tarnsfer 3% of flore prise to admin */
        //* Transfer sol from user  account to admin()
      
    
        let transfer_sol_to_treasury_ix =
        system_instruction::transfer(user_account.key, treasury_account.key, flore_prise * platform_data.platform_fees / 100);

        invoke(
            &transfer_sol_to_treasury_ix,
            &[
                user_account.clone(),
                treasury_account.clone(),
                user_account.clone(),
                system_program.clone(),
            ],
        )?;
      
       

        if game_data.winner_pubkey == *user_account.key {
            //* Transfer NFT from owner token account to PDA(contract)
            let tranfer_instructions = spl_token::instruction::transfer(
                token_program.key,
                game_token_account.key,
                user_token_account.key,
                game_state_account.key,
                &[],
                1,
            )?;

            invoke_signed(
                &tranfer_instructions,
                &[
                    game_token_account.clone(),
                    user_token_account.clone(),
                    game_state_account.clone(),
                    token_program.clone(),
                ],
                &[&[
                    game_prefix.as_bytes(),
                    (collection_state_account.key).as_ref(),
                    &[game_nonce],
                ]],
            )?;
        } else {
            return Err(BettingError::InvalidWinner.into());
        }
      

        Gamestruct::pack(game_data, &mut game_state_account.try_borrow_mut_data()?)?;
        CollectionData::pack(
            collection_data,
            &mut collection_state_account.try_borrow_mut_data()?,
        )?;
     

        Ok(())
    }
}

fn convert_hex_to_str(bytes: Vec<u8>) -> String {
    let hash_string: Vec<String> = bytes.iter().map(|b| format!("{:02x}", b)).collect();
    hash_string.join("")
}

fn get_hash(client_seed: &str, server_seed: &str, nonce: u64) -> String {
    let mut hasher = Sha256::new();
    hasher.update(&format!("{}{}{}", client_seed.trim(), server_seed, nonce).into_bytes());
    let hash = hasher.finalize();
    convert_hex_to_str(hash.to_vec())
}

fn get_random_no(hash: &str) -> u64 {
    let mut index = 0;
    let mut result;

    loop {
        result = u64::from_str_radix(&hash[(index * 5)..((index * 5) + 5)], 16).unwrap();
        index += 1;

        if (index * 5 + 5) > 64 {
            result = 9999;
            break;
        }

        if result < 1000000 {
            break;
        };
    }

    result % 6
}

fn get_new_nonce(hash: &str) -> u64 {
    let mut index = 0;
    let mut result;

    loop {
        result = u64::from_str_radix(&hash[(index * 5)..((index * 5) + 5)], 16).unwrap();
        index += 1;

        if (index * 5 + 5) > 64 {
            result = 9999;
            break;
        }

        if result < 1000000 {
            break;
        };
    }

    result % 55555
}

fn get_vacant_place(game_state: Gamestruct) -> Option<u64> {
    if !game_state.player1_joined {
        return Some(0);
    } else if !game_state.player2_joined {
        return Some(1);
    } else if !game_state.player3_joined {
        return Some(2);
    } else if !game_state.player4_joined {
        return Some(3);
    } else if !game_state.player5_joined {
        return Some(4);
    } else if !game_state.player6_joined {
        return Some(5);
    }

    None
}

fn get_disjoining_player(game_state: Gamestruct, user_account_key: Pubkey) -> Option<u64> {
    if game_state.player1_pubkey == user_account_key {
        return Some(0);
    } else if game_state.player2_pubkey == user_account_key {
        return Some(1);
    } else if game_state.player3_pubkey == user_account_key {
        return Some(2);
    } else if game_state.player4_pubkey == user_account_key {
        return Some(3);
    } else if game_state.player5_pubkey == user_account_key {
        return Some(4);
    } else if game_state.player6_pubkey == user_account_key {
        return Some(5);
    }

    None
}
