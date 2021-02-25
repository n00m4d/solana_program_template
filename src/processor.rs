//! Program state processor

use crate::{error::ProgramTemplateError, instruction::TemplateInstruction};
use borsh::BorshDeserialize;
use solana_program::{
    account_info::next_account_info, account_info::AccountInfo, entrypoint::ProgramResult, msg,
    pubkey::Pubkey,
};

/// Program state handler.
pub struct Processor {}
impl Processor {
    /// Initialize the pool
    pub fn process_example_instruction(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let _example_account_info = next_account_info(account_info_iter)?;

        Ok(())
    }

    /// Processes an instruction
    pub fn process_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        input: &[u8],
    ) -> ProgramResult {
        let instruction = TemplateInstruction::try_from_slice(input)
            .or(Err(ProgramTemplateError::ExampleError))?;
        match instruction {
            TemplateInstruction::ExampleInstruction => {
                msg!("Instruction: ExampleInstruction");
                Self::process_example_instruction(program_id, accounts)
            }
        }
    }
}
