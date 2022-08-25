#![allow(clippy::too_many_arguments)]

use solana_program::program_error::ProgramError;
use std::mem;

#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BettingInstruction {
    //* PlatformData
    InitPlatformData { args: (u64, u64) },

    CreateCollection,

    //* Create a new Game
    CreateGame,

    //* Join a new Game
    JoinGame,

    DisJoinGame { amount: u64 },

    //* winner will Claim Reward
    ClaimReward { amount: u64 },
}

//* Packing and unpacking for the instructions data.
//* References: https://github.com/solana-labs/solana-program-library/blob/7caf27cca6a9f58055f93517774318eb2b2f97bf/token-swap/program/src/instruction.rs#L190

impl BettingInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidAccountData)?;

        Ok(match tag {
            0 => Self::InitPlatformData {
                args: Self::unpack_data(rest)?,
            },

            1 => Self::CreateCollection,

            2 => Self::CreateGame,
            3 => Self::JoinGame,
            4 => Self::DisJoinGame {
                amount: Self::unpack_amount(rest)?,
            },
            5 => Self::ClaimReward {
                amount: Self::unpack_amount(rest)?,
            },
            _ => return Err(ProgramError::InvalidAccountData),
        })
    }

    pub fn pack(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(mem::size_of::<Self>());
        match self {
            Self::InitPlatformData { args } => {
                buf.push(0);
                buf.extend_from_slice(&args.0.to_le_bytes());
                buf.extend_from_slice(&args.1.to_le_bytes());
            }

            Self::ClaimReward { amount } => {
                buf.push(4);
                buf.extend_from_slice(&amount.to_le_bytes());
            }

            _ => todo!(),
        }
        buf
    }

    fn unpack_amount(input: &[u8]) -> Result<u64, ProgramError> {
        let amount = input
            .get(0..8)
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes)
            .ok_or(ProgramError::InvalidInstructionData)?;

        Ok(amount)
    }
    fn unpack_data(input: &[u8]) -> Result<(u64, u64), ProgramError> {
        let nonce = input
            .get(0..8)
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes)
            .ok_or(ProgramError::InvalidInstructionData)?;
        let platform_fees = input
            .get(8..16)
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes)
            .ok_or(ProgramError::InvalidInstructionData)?;

        Ok((nonce, platform_fees))
    }
}
