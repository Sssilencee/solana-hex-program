# solana-hex-program

The `solana-hex-program` is a simple Solana program designed for safe funds transfer.

## Algorithm

The algorithm for the program is as follows:

1. The backend creates an associated Program Derived Address (PDA) account and writes the payment information to the account data.
2. The frontend includes this PDA account in the transaction, and the program withdraws funds from a signer account.
3. The backend checks the "status" field in the PDA data to validate the transaction.

## Build

To build the program, follow these steps:

1. Build the binary `.so` file using the command: `cargo build-bpf`.
2. Deploy the program using the command: `solana program deploy target/deploy/transfer_program.so`.

## Examples

Serialization of the payment info struct can be found in this [example](https://github.com/Sssilencee/solana-hex-program/blob/main/client/serialization-example.ts).

## Admin Account

The program admin account for the backend can be found at [ADMIN_ACCOUNT_ID](https://github.com/Sssilencee/solana-hex-program/blob/b02c983c3cc7f87ce5060313de07ee6866a3bbf8/program/src/lib.rs#L16).
