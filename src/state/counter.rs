use core::mem::size_of;
use pinocchio::{instruction::Seed, program_error::ProgramError, pubkey::Pubkey};

#[repr(C)]
pub enum StateKey {
    Uninitialized = 0,
    Counter = 1,
}

#[repr(C)]
pub struct Counter {
    pub key: StateKey,
    pub bump: u8,
    pub authority: Pubkey,
    pub value: u64,
}

impl Counter {
    pub const LEN: usize =
        size_of::<StateKey>() + size_of::<u8>() + size_of::<Pubkey>() + size_of::<u64>();

    #[inline(always)]
    pub fn load_mut(bytes: &mut [u8]) -> Result<&mut Self, ProgramError> {
        if bytes.len() != Counter::LEN || bytes[0] != StateKey::Counter as u8 {
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(unsafe { &mut *core::mem::transmute::<*mut u8, *mut Self>(bytes.as_mut_ptr()) })
    }

    #[inline(always)]
    pub fn load_bump(bytes: &[u8]) -> Result<u8, ProgramError> {
        if bytes.len() != Counter::LEN || bytes[0] != StateKey::Counter as u8 {
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(bytes[1])
    }

    #[inline(always)]
    pub fn load(bytes: &[u8]) -> Result<&Self, ProgramError> {
        if bytes.len() != Counter::LEN || bytes[0] != StateKey::Counter as u8 {
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(unsafe { &*core::mem::transmute::<*const u8, *const Self>(bytes.as_ptr()) })
    }

    #[inline(always)]
    pub fn set_inner(&mut self, authority: Pubkey, bump: u8) {
        self.key = StateKey::Counter;
        self.bump = bump;
        self.authority = authority;
        self.value = 0;
    }

    #[inline(always)]
    pub fn increase_by(&mut self, amount: u64) -> Result<(), ProgramError> {
        self.value = self
            .value
            .checked_add(amount)
            .ok_or(ProgramError::ArithmeticOverflow)?;
        Ok(())
    }

    #[inline(always)]
    pub fn seeds(authority: &Pubkey) -> [&[u8]; 2] {
        [b"counter", authority.as_ref()]
    }

    #[inline(always)]
    pub fn seeds_with_bump<'a>(authority: &'a Pubkey, bump: &'a [u8]) -> [&'a [u8]; 3] {
        let seeds = Self::seeds(authority);
        [seeds[0], seeds[1], bump]
    }

    #[inline(always)]
    pub fn signer_seeds_with_bump<'a>(authority: &'a Pubkey, bump: &'a [u8]) -> [Seed<'a>; 3] {
        let seeds = Self::seeds_with_bump(authority, bump);
        [
            Seed::from(seeds[0]),
            Seed::from(seeds[1]),
            Seed::from(seeds[2]),
        ]
    }
}
