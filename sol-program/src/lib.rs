use borsh::{BorshDeserialize, BorshSerialize, de};

use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
enum InstructionType {
    Increment(u32),
    Decrement(u32),
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct Counter {
    count: u32,
}

entrypoint!(counter_contract);

pub fn counter_contract(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let acc = next_account_info(&mut accounts.iter())?;

    let instruction_type = InstructionType::try_from_slice(instruction_data)?;

    let mut counter = Counter::try_from_slice(&acc.data.borrow())?;

    match instruction_type {
        InstructionType::Increment(amount) => {
            msg!("Incrementing counter by {}", amount);
            counter.count += amount;
        }
        InstructionType::Decrement(amount) => {
            if counter.count >= amount {
                msg!("Decrementing counter by {}", amount);
                counter.count -= amount;
            } else {
                msg!("Counter cannot go below zero");
                return Err(solana_program::program_error::ProgramError::InvalidArgument);
            }
        }
    }

    counter.serialize(&mut *acc.data.borrow_mut())?;
    msg!("Contract succeded");

    Ok(())
}
