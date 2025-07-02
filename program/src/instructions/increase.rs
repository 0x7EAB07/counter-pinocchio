use pinocchio::{account_info::AccountInfo, program_error::ProgramError};

use crate::{
    helpers::{AccountCheck, PdaAccount, ProgramAccount, WritableAccount},
    state::Counter,
};

pub struct IncreaseAccounts<'a> {
    pub counter: &'a AccountInfo,
    pub authority: &'a AccountInfo,
}

impl<'a> TryFrom<&'a [AccountInfo]> for IncreaseAccounts<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let [counter, authority] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        WritableAccount::check(counter)?;
        ProgramAccount::check(counter)?;

        let counter_data = counter.try_borrow_data()?;
        // Hint: Can be optimized to just load the bump.
        let counter_account = Counter::load(&counter_data)?;

        PdaAccount::check(
            counter,
            &Counter::seeds_with_bump(authority.key(), &[counter_account.bump]),
        )?;

        Ok(Self { counter, authority })
    }
}

pub struct IncreaseInstructionData {
    pub amount: u64,
}

impl TryFrom<&[u8]> for IncreaseInstructionData {
    type Error = ProgramError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let amount = u64::from_le_bytes(
            data.try_into()
                .map_err(|_| ProgramError::InvalidInstructionData)?,
        );
        Ok(Self { amount })
    }
}

pub struct Increase<'a> {
    pub accounts: IncreaseAccounts<'a>,
    pub instruction_data: IncreaseInstructionData,
}

impl<'a> TryFrom<(&'a [u8], &'a [AccountInfo])> for Increase<'a> {
    type Error = ProgramError;

    fn try_from(
        (instruction_data, accounts): (&'a [u8], &'a [AccountInfo]),
    ) -> Result<Self, Self::Error> {
        let accounts = IncreaseAccounts::try_from(accounts)?;
        let instruction_data = IncreaseInstructionData::try_from(instruction_data)?;
        Ok(Self {
            accounts,
            instruction_data,
        })
    }
}

impl<'a> Increase<'a> {
    pub const DISCRIMINATOR: &'a u8 = &1;

    pub fn process(&mut self) -> Result<(), ProgramError> {
        let mut data = self.accounts.counter.try_borrow_mut_data()?;
        let counter = Counter::load_mut(data.as_mut(), true)?;
        counter.increase_by(self.instruction_data.amount)?;
        Ok(())
    }
}
