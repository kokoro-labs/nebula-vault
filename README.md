# Nebula Vault

Solana native token vault PoC program. Provides secure token storage with features like time-locked withdrawals and owner-only access control.

## Features

- Secure token storage
- Time-locked withdrawals
- Customizable timelock periods
- Owner-only access control

## Prerequisites

- Rust and Cargo
- Solana CLI tools (v1.16 or later)
- Node.js and npm

## Building

To build the project, run:

```
cargo build-bpf
```

## Testing

To run the tests, use:

```
cargo test-bpf
```

## Deployment

To deploy the program to devnet:

```
solana program deploy target/deploy/nebula_vault.so
```

## Usage

Basic examples of how to use the Nebula Vault:

1. Initialize the vault:
   ```javascript
   const initializeIx = await program.instruction.initialize(
     vault.publicKey,
     owner.publicKey,
     tokenMint.publicKey
   );
   ```

2. Deposit tokens:
   ```javascript
   const depositIx = await program.instruction.deposit(
     new BN(amount),
     vault.publicKey,
     owner.publicKey,
     tokenAccount.publicKey
   );
   ```

3. Withdraw tokens:
   ```javascript
   const withdrawIx = await program.instruction.withdraw(
     new BN(amount),
     vault.publicKey,
     owner.publicKey,
     tokenAccount.publicKey
   );
   ```

4. Set timelock:
   ```javascript
   const setTimelockIx = await program.instruction.setTimelock(
     new BN(newTimelock),
     vault.publicKey,
     owner.publicKey
   );
   ```

## License

This project is open source and available under the [Apache License 2.0](LICENSE).
