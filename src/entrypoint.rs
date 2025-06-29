use pinocchio::{
    account_info::AccountInfo, entrypoint, program_error::ProgramError, pubkey::Pubkey,
    ProgramResult,
};

use crate::instructions::{Create, Increase};

entrypoint!(process_instruction);

fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    match instruction_data.split_first() {
        Some((Create::DISCRIMINATOR, _)) => Create::try_from(accounts)?.process(),
        Some((Increase::DISCRIMINATOR, instruction_data)) => {
            Increase::try_from((instruction_data, accounts))?.process()
        }
        _ => Err(ProgramError::InvalidInstructionData),
    }
}
