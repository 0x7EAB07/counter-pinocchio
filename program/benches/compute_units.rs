use {
    counter_pinocchio::{
        instructions::{Create, Increase},
        state::Counter,
        ID as PROGRAM_ID,
    },
    mollusk_svm::Mollusk,
    mollusk_svm_bencher::MolluskComputeUnitBencher,
    solana_sdk::{
        account::Account,
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        system_program,
    },
};

fn to_sdk_pubkey(pubkey: pinocchio::pubkey::Pubkey) -> Pubkey {
    solana_sdk::pubkey::Pubkey::new_from_array(pubkey)
}

fn to_pino_pubkey(pubkey: Pubkey) -> pinocchio::pubkey::Pubkey {
    pubkey.to_bytes().into()
}

/// Helper function to create instruction data for increase
fn create_increase_instruction_data(amount: u64) -> Vec<u8> {
    let mut data = vec![*Increase::DISCRIMINATOR];
    data.extend_from_slice(&amount.to_le_bytes());
    data
}

/// Helper function to derive counter PDA
fn derive_counter_pda(authority: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &Counter::seeds(&to_pino_pubkey(*authority)),
        &to_sdk_pubkey(PROGRAM_ID),
    )
}

fn main() {
    let mollusk = Mollusk::new(&to_sdk_pubkey(PROGRAM_ID), "counter-pinocchio");

    // Setup test accounts
    let authority = Pubkey::new_unique();
    let (counter_pda, _bump) = derive_counter_pda(&authority);

    // Create authority account with SOL for rent
    let authority_account = Account {
        lamports: 1_000_000_000, // 1 SOL
        data: vec![],
        owner: system_program::ID,
        executable: false,
        rent_epoch: 0,
    };

    // Prepare accounts for create instruction
    let create_accounts = vec![
        (counter_pda, Account::default()), // Will be created by the instruction
        (authority, authority_account.clone()),
        (system_program::ID, Account::default()),
    ];

    // Create instruction
    let create_instruction = Instruction {
        program_id: to_sdk_pubkey(PROGRAM_ID),
        accounts: vec![
            AccountMeta::new(counter_pda, false),
            AccountMeta::new(authority, true),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: vec![*Create::DISCRIMINATOR],
    };

    // Prepare accounts for increase instructions
    // Note: We need to create a counter account that already exists for increase benchmarks
    let mut counter_account_data = vec![0u8; Counter::LEN];
    let counter = Counter::load_mut(&mut counter_account_data, false).unwrap();
    counter.set_inner(to_pino_pubkey(authority), _bump);

    let counter_account = Account {
        lamports: 1_000_000, // Ensure sufficient lamports
        data: counter_account_data,
        owner: to_sdk_pubkey(PROGRAM_ID),
        executable: false,
        rent_epoch: 0,
    };

    let increase_accounts = vec![
        (counter_pda, counter_account),
        (authority, authority_account),
    ];

    // Different increase instructions to benchmark
    let increase_1_instruction = Instruction {
        program_id: to_sdk_pubkey(PROGRAM_ID),
        accounts: vec![
            AccountMeta::new(counter_pda, false),
            AccountMeta::new_readonly(authority, false),
        ],
        data: create_increase_instruction_data(1),
    };

    let increase_100_instruction = Instruction {
        program_id: to_sdk_pubkey(PROGRAM_ID),
        accounts: vec![
            AccountMeta::new(counter_pda, false),
            AccountMeta::new_readonly(authority, false),
        ],
        data: create_increase_instruction_data(100),
    };

    let increase_max_instruction = Instruction {
        program_id: to_sdk_pubkey(PROGRAM_ID),
        accounts: vec![
            AccountMeta::new(counter_pda, false),
            AccountMeta::new_readonly(authority, false),
        ],
        data: create_increase_instruction_data(u64::MAX / 2), // Large number that won't overflow
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
        .out_dir("../target/benches")
        .execute();
}
