use mollusk_svm::program;
use solana_sdk::pubkey;
use solana_sdk::pubkey::Pubkey;
use {
    counter_pinocchio::{
        instructions::{Create, Increase},
        state::{to_bytes, Counter, StateKey},
        ID,
    },
    mollusk_svm::Mollusk,
    mollusk_svm_bencher::MolluskComputeUnitBencher,
    solana_sdk::{
        account::Account,
        instruction::{AccountMeta, Instruction},
        system_program,
    },
};

pub const PROGRAM: Pubkey = Pubkey::new_from_array(ID);
pub const AUTHORITY: Pubkey = pubkey!("Co11111111111111111111111111111111111111111");

pub fn mollusk() -> Mollusk {
    let mollusk = Mollusk::new(&PROGRAM, "../target/deploy/counter_pinocchio");
    mollusk
}

/// Helper function to create instruction data for increase
fn create_increase_instruction_data(amount: u64) -> Vec<u8> {
    let mut data = vec![*Increase::DISCRIMINATOR];
    data.extend_from_slice(&amount.to_le_bytes());
    data
}

/// Helper function to derive counter PDA
fn derive_counter_pda(authority: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&Counter::seeds(&authority.to_bytes()), &PROGRAM)
}

fn main() {
    let mollusk = mollusk();

    // Setup test accounts
    let (counter_pda, bump) = derive_counter_pda(&AUTHORITY);
    let authority_account = Account::new(1_000_000_000, 0, &system_program::ID);
    let (system_program, system_account) = program::keyed_account_for_system_program();

    // Prepare accounts for create instruction
    let create_accounts = vec![
        (counter_pda, Account::new(0, 0, &system_program::ID)),
        (AUTHORITY, authority_account.clone()),
        (system_program, system_account.clone()),
    ];

    // Create instruction
    let create_instruction = Instruction {
        program_id: PROGRAM,
        accounts: vec![
            AccountMeta::new(counter_pda, false),
            AccountMeta::new(AUTHORITY, true),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: vec![*Create::DISCRIMINATOR],
    };

    // Prepare accounts for increase instructions
    let counter = Counter {
        authority: AUTHORITY.to_bytes(),
        bump,
        key: StateKey::Counter,
        value: 0,
    };

    let counter_account = Account {
        lamports: 1_000_000,
        data: unsafe { to_bytes(&counter, Counter::LEN).to_vec() },
        owner: PROGRAM,
        executable: false,
        rent_epoch: 0,
    };

    let increase_accounts = vec![
        (counter_pda, counter_account),
        (AUTHORITY, authority_account),
    ];

    // Different increase instructions to benchmark
    let increase_1_instruction = Instruction {
        program_id: PROGRAM,
        accounts: vec![
            AccountMeta::new(counter_pda, false),
            AccountMeta::new_readonly(AUTHORITY, false),
        ],
        data: create_increase_instruction_data(1),
    };

    let increase_100_instruction = Instruction {
        program_id: PROGRAM,
        accounts: vec![
            AccountMeta::new(counter_pda, false),
            AccountMeta::new_readonly(AUTHORITY, false),
        ],
        data: create_increase_instruction_data(100),
    };

    let increase_max_instruction = Instruction {
        program_id: PROGRAM,
        accounts: vec![
            AccountMeta::new(counter_pda, false),
            AccountMeta::new_readonly(AUTHORITY, false),
        ],
        data: create_increase_instruction_data(u64::MAX / 2),
    };

    // Run benchmarks
    MolluskComputeUnitBencher::new(mollusk)
        .bench(("create_counter", &create_instruction, &create_accounts))
        .bench(("increase_by_1", &increase_1_instruction, &increase_accounts))
        .bench((
            "increase_by_100",
            &increase_100_instruction,
            &increase_accounts,
        ))
        .bench((
            "increase_by_large_number",
            &increase_max_instruction,
            &increase_accounts,
        ))
        .must_pass(true)
        .out_dir("../benches")
        .execute();
}
