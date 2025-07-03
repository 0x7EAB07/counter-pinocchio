use counter_pinocchio::{
    instructions::{Create, Increase},
    state::{to_bytes, Counter, StateKey},
    ID,
};
use mollusk_svm::{
    program,
    result::{Check, ProgramResult},
    Mollusk,
};
use solana_sdk::pubkey;
use solana_sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction},
    native_token::LAMPORTS_PER_SOL,
    pubkey::Pubkey,
};
extern crate alloc;
use alloc::vec;

pub const PROGRAM: Pubkey = Pubkey::new_from_array(ID);
pub const AUTHORITY: Pubkey = pubkey!("Co11111111111111111111111111111111111111111");

pub fn mollusk() -> Mollusk {
    let mollusk = Mollusk::new(&PROGRAM, "../target/deploy/counter_pinocchio");
    mollusk
}

#[test]
fn test_create_counter() {
    let mollusk = mollusk();

    //system program and system account
    let (system_program, system_account) = program::keyed_account_for_system_program();

    // Create the PDA
    let (counter_pda, _bump) =
        Pubkey::find_program_address(&Counter::seeds(&AUTHORITY.to_bytes()), &PROGRAM);

    //Initialize the accounts
    let authority_account = Account::new(1 * LAMPORTS_PER_SOL, 0, &system_program);
    let counter_account = Account::new(0, 0, &system_program);

    //Push the accounts in to the instruction_accounts vec!
    let ix_accounts = vec![
        AccountMeta::new(counter_pda, false),             // counter
        AccountMeta::new(AUTHORITY, true),                // authority
        AccountMeta::new_readonly(system_program, false), // system program
    ];

    let ix_data = vec![*Create::DISCRIMINATOR];

    // Create instruction
    let instruction = Instruction::new_with_bytes(PROGRAM, &ix_data, ix_accounts);

    // Create tx_accounts vec
    let tx_accounts = &vec![
        (counter_pda, counter_account.clone()),
        (AUTHORITY, authority_account.clone()),
        (system_program, system_account.clone()),
    ];

    let init_res =
        mollusk.process_and_validate_instruction(&instruction, tx_accounts, &[Check::success()]);

    assert!(init_res.program_result == ProgramResult::Success);
}

#[test]
fn test_increase_counter() {
    let mollusk = mollusk();

    //system program and system account
    let (system_program, _system_account) = program::keyed_account_for_system_program();

    // Create the PDA
    // Create the PDA
    let (counter_pda, bump) =
        Pubkey::find_program_address(&Counter::seeds(&AUTHORITY.to_bytes()), &PROGRAM);

    //Initialize the accounts
    let authority_account = Account::new(1 * LAMPORTS_PER_SOL, 0, &system_program);

    let mut counter_account = Account::new(
        mollusk.sysvars.rent.minimum_balance(Counter::LEN),
        Counter::LEN,
        &ID.into(),
    );

    let counter = Counter {
        authority: AUTHORITY.to_bytes(),
        bump,
        key: StateKey::Counter,
        value: 0,
    };

    counter_account.data = unsafe { to_bytes(&counter, Counter::LEN).to_vec() };

    //Push the accounts in to the instruction_accounts vec!
    let ix_accounts = vec![
        AccountMeta::new(counter_pda, false),
        AccountMeta::new(AUTHORITY, true),
    ];

    let mut ix_data = vec![*Increase::DISCRIMINATOR];
    ix_data.extend_from_slice(&1u64.to_le_bytes());

    // Create instruction
    let instruction = Instruction::new_with_bytes(PROGRAM, &ix_data, ix_accounts);
    // Create tx_accounts vec
    let tx_accounts = &vec![
        (counter_pda, counter_account.clone()),
        (AUTHORITY, authority_account.clone()),
    ];

    let update_res =
        mollusk.process_and_validate_instruction(&instruction, tx_accounts, &[Check::success()]);

    assert!(update_res.program_result == ProgramResult::Success);
}
