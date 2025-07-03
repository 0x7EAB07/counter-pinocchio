use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    pubkey::create_program_address,
    sysvars::{rent::Rent, Sysvar},
};
extern crate alloc;
use pinocchio_system::instructions::CreateAccount;

pub trait AccountCheck {
    fn check(account: &AccountInfo) -> Result<(), ProgramError>;
}

// Define a Type
pub struct SignerAccount;

// Implement the trait for different Types
impl AccountCheck for SignerAccount {
    fn check(account: &AccountInfo) -> Result<(), ProgramError> {
        if !account.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }
        Ok(())
    }
}

pub struct WritableAccount;

impl AccountCheck for WritableAccount {
    fn check(account: &AccountInfo) -> Result<(), ProgramError> {
        if !account.is_writable() {
            return Err(ProgramError::Immutable);
        }
        Ok(())
    }
}

pub struct EmptyAccount;

impl AccountCheck for EmptyAccount {
    fn check(account: &AccountInfo) -> Result<(), ProgramError> {
        if account.data_len() != 0 {
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(())
    }
}

pub struct SystemProgram;

impl AccountCheck for SystemProgram {
    fn check(account: &AccountInfo) -> Result<(), ProgramError> {
        if account.key() != &pinocchio_system::ID {
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(())
    }
}

pub struct ProgramAccount;

impl AccountCheck for ProgramAccount {
    fn check(account: &AccountInfo) -> Result<(), ProgramError> {
        if !account.is_owned_by(&crate::ID) {
            return Err(ProgramError::InvalidAccountOwner);
        }

        Ok(())
    }
}

pub struct PdaAccount;

impl PdaAccount {
    pub fn check(account: &AccountInfo, seeds_with_bump: &[&[u8]]) -> Result<(), ProgramError> {
        let pda_address = create_program_address(seeds_with_bump, &crate::ID)?;
        if account.key() != &pda_address {
            return Err(ProgramError::InvalidAccountOwner);
        }
        Ok(())
    }
}

pub trait ProgramAccountInit {
    fn init<T: Sized>(
        payer: &AccountInfo,
        account: &AccountInfo,
        seeds: &[Seed],
        space: usize,
    ) -> Result<(), ProgramError>;
}

impl ProgramAccountInit for ProgramAccount {
    fn init<T: Sized>(
        payer: &AccountInfo,
        account: &AccountInfo,
        seeds: &[Seed],
        space: usize,
    ) -> Result<(), ProgramError> {
        CreateAccount {
            from: payer,
            to: account,
            lamports: Rent::get()?.minimum_balance(space),
            space: space as u64,
            owner: &crate::ID,
        }
        .invoke_signed(&[Signer::from(seeds)])?;
        Ok(())
    }
}
