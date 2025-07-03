# Pinocchio Counter

[![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Pinocchio](https://img.shields.io/badge/Pinocchio-FF6B6B?style=for-the-badge&logo=rust&logoColor=white)](https://github.com/anza-xyz/pinocchio)
[![Mollusk](https://img.shields.io/badge/Mollusk-2A7A7A?style=for-the-badge&logo=rust&logoColor=white)](https://github.com/anza-xyz/mollusk)
[![Size](https://img.shields.io/badge/Size-13.85kb-AEFF23?style=for-the-badge&logoColor=white)](https://github.com/anza-xyz/pinocchio)
[![Twitter](https://img.shields.io/badge/Twitter-1DA1F2?style=for-the-badge&logo=twitter&logoColor=white)](https://twitter.com/0x7eab07)

The Solana program is ~13.85kb, impressive size for what it does.

- Same example in anchor would be ~190kb.
- Same example in solana-program is ~100kb.

## Features

- `pinocchio` for program development.
- `mollusk` for testing and benchmarking.
- GH Action for building, testing and benchmarking.
- `cli` rust-based cli using `solana-client` and `solana-sdk`.

## How To

### Build contract

```sh
$ cargo build-sbf
```

### Deploy contract

```sh
$ solana program deploy -u d --program-id ./program-id.json \
  -k ./deployer.json \
  --upgrade-authority ./deployer.json \
  ./target/deploy/counter_pinocchio.so
```

### Test

```sh
$ cargo test --features test-default
```

### Benchmark

```sh
$ cargo bench --features bench-default
```

## Attributions

Thanks @nagaprasadvr for his work on http://github.com/Nagaprasadvr/solana-pinocchio-starter

## Benches

#### 2025-07-03 05:05:34.478002 UTC

Solana CLI Version: solana-cli 2.1.14 (src:3ad46824; feat:3271415109, client:Agave)

| Name                     | CUs  | Delta   |
| ------------------------ | ---- | ------- |
| create_counter           | 3220 | - new - |
| increase_by_1            | 1698 | - new - |
| increase_by_100          | 1698 | - new - |
| increase_by_large_number | 1698 | - new - |
