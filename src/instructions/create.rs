use pinocchio::{
    account_info::AccountInfo, program_error::ProgramError, pubkey::find_program_address,
};

use crate::{
    helpers::{
        AccountCheck, EmptyAccount, ProgramAccount, ProgramAccountInit, SignerAccount,
        SystemProgram, WritableAccount,
    },
    state::Counter,
};

pub struct CreateAccounts<'a> {
    pub counter: &'a AccountInfo,
    pub authority: &'a AccountInfo,
    pub system_program: &'a AccountInfo,
}

impl<'a> TryFrom<&'a [AccountInfo]> for CreateAccounts<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let [counter, authority, system_program, _] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        SignerAccount::check(authority)?;
        WritableAccount::check(authority)?;

        EmptyAccount::check(counter)?;
        WritableAccount::check(counter)?;

        SystemProgram::check(system_program)?;

        Ok(Self {
            counter,
            authority,
            system_program,
        })
    }
}

pub struct Create<'a> {
    pub accounts: CreateAccounts<'a>,
    pub counter_bump: u8,
}

impl<'a> TryFrom<&'a [AccountInfo]> for Create<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let accounts = CreateAccounts::try_from(accounts)?;

        let (_, counter_bump) =
            find_program_address(&Counter::seeds(accounts.authority.key()), &crate::ID);

        ProgramAccount::init::<Counter>(
            accounts.authority,
            accounts.counter,
            &Counter::signer_seeds_with_bump(accounts.authority.key(), &[counter_bump]),
            Counter::LEN,
        )?;

        Ok(Self {
            accounts,
            counter_bump,
        })
    }
}

impl<'a> Create<'a> {
    pub const DISCRIMINATOR: &'a u8 = &0;

    pub fn process(&mut self) -> Result<(), ProgramError> {
        let mut data = self.accounts.counter.try_borrow_mut_data()?;
        let counter = Counter::load_mut(data.as_mut())?;
        counter.set_inner(self.accounts.authority.key().clone(), self.counter_bump);
        Ok(())
    }
}
