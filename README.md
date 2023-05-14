# solana-hex-program
Simple Solana program for safe funds transfer

<h2>Algorithm</h2>

- Backend creates an associated **PDA** Account and writes payment info to an Account data
- Frontend passes this account in the transaction then the program withdraws funds from a signer
- Backend checks the "status" field in the **PDA** data

<h2>Build</h2>

- Build binary **.so** file: `cargo-build-bpf`
- Deploy: `solana program deploy target/deploy/transfer_program.so`

<h2>Examples</h2>

Payment info struct serialization: [Example](https://github.com/Sssilencee/solana-hex-program/blob/main/client/serialization-example.ts)

<h2>Admin account</h2>

Program admin account (Backend): [ADMIN_ACCOUNT_ID](https://github.com/Sssilencee/solana-hex-program/blob/b02c983c3cc7f87ce5060313de07ee6866a3bbf8/program/src/lib.rs#L16)
