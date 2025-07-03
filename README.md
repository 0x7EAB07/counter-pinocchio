# Counter Pinocchio

A Rust monorepo containing a Solana counter program and a CLI tool to interact with it.

The Solana program is ~15kb, impressive size for what it does.

- Same example in anchor would be ~190kb.
- Same example in solana-program is ~100kb.

## Features

- `pinocchio` for program development.
- `mollusk` for testing and benchmarking.
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
