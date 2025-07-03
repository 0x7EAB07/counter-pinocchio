use counter_pinocchio::ID as PROGRAM_ID;
use solana_sdk::pubkey::Pubkey;

pub fn get_program_id() -> Pubkey {
    Pubkey::new_from_array(PROGRAM_ID)
}

pub fn get_counter_address(authority: &Pubkey, program_id: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"counter", authority.as_ref()], program_id)
}
