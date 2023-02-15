use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};

const TOKEN_PROGRAM_ID: &str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
const ADMIN_ACCOUNT_ID: &str = "";

#[derive(BorshSerialize, BorshDeserialize, Debug)]
enum Transfer {
    Sol,
    Spl
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct Payment {
    seed: String,
    amount: u64,
    fee: f64,
    status: String,
    shop_wallet: String,
    hex_wallet: String
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
enum InstructionData {
    Transfer(Transfer),
    CreatePayment(Payment)
}

entrypoint!(process_instruction);
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    match InstructionData::try_from_slice(instruction_data)? {
        InstructionData::Transfer(Transfer::Sol) => transfer_sol(accounts),
        InstructionData::Transfer(Transfer::Spl) => transfer_spl(accounts),
        InstructionData::CreatePayment(payment_struct) => create_payment(program_id, accounts, payment_struct)
    }
}

/// Accounts:
/// 
/// 0. `[signer, writable]` Debit lamports from this account
/// 1. `[writable]` PDA account with payment data
/// 2. `[writable]` Hex account(Service fee)
/// 3. `[writable]` Shop account 
/// 4. `[]` System program
fn transfer_sol(
    accounts: &[AccountInfo],
) -> ProgramResult {

    let accounts_iter = &mut accounts.iter();

    let sender = next_account_info(accounts_iter)?;
    let pda = next_account_info(accounts_iter)?;
    let hex_wallet = next_account_info(accounts_iter)?;
    let shop_wallet = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    if !sender.is_signer {
        msg!("Sender isn't a transaction signer");
        return Err(ProgramError::MissingRequiredSignature);
    }
    if pda.data_is_empty() {
        msg!("Pda is empty");
        return Err(ProgramError::InvalidAccountData);
    }
    if !solana_program::system_program::check_id(system_program.key) {
        msg!("Unknown program was passed instead of System program");
        return Err(ProgramError::InvalidArgument);
    }

    let mut payment = Payment::try_from_slice(&pda.data.borrow())?;
    
    if hex_wallet.key.to_string() != payment.hex_wallet {
        msg!("Hex wallet is invalid");
        return Err(ProgramError::InvalidArgument);
    }
    if shop_wallet.key.to_string() != payment.shop_wallet {
        msg!("Shop wallet is invalid");
        return Err(ProgramError::InvalidArgument);
    }

    let fee_amount = payment.amount as f64 * payment.fee;
    let fee_transfer = system_instruction::transfer(
        sender.key, hex_wallet.key, 
        fee_amount as u64
    );
    invoke(&fee_transfer, &[sender.clone(), hex_wallet.clone()])?;
    msg!("Transfered {} sol fee to the hex wallet", fee_amount);

    let shop_amount = payment.amount as f64 * (1_f64 - payment.fee);
    let shop_transfer = system_instruction::transfer(
        sender.key, shop_wallet.key,
        shop_amount as u64
    );
    invoke(&shop_transfer, &[sender.clone(), shop_wallet.clone()])?;
    msg!("Transfered {} sol to the shop wallet", shop_amount);

    payment.status = String::from("paid");
    payment.serialize(&mut &mut pda.data.borrow_mut()[..])?;

    Ok(())

}

/// Accounts:
/// 
/// 0. `[signer, writable]` Sender account
/// 1. `[writable]` PDA account with payment data
/// 2. `[writable]` Sender token account
/// 3. `[writable]` Hex token account(Service fee)
/// 4. `[writable]` Shop token account 
/// 5. `[]` Token program
fn transfer_spl(
    accounts: &[AccountInfo],
) -> ProgramResult {

    let accounts_iter = &mut accounts.iter();

    let sender = next_account_info(accounts_iter)?;
    let pda = next_account_info(accounts_iter)?;
    let sender_token_account = next_account_info(accounts_iter)?;
    let hex_token_account = next_account_info(accounts_iter)?;
    let shop_token_account = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;

    if !sender.is_signer {
        msg!("Sender isn't a transaction signer");
        return Err(ProgramError::MissingRequiredSignature);
    }
    if pda.data_is_empty() {
        msg!("Pda is empty");
        return Err(ProgramError::InvalidAccountData);
    }
    if token_program.key.to_string() != TOKEN_PROGRAM_ID.to_string() {
        msg!("Unknown program was passed instead of Token program");
        return Err(ProgramError::InvalidArgument);
    }

    let mut payment = Payment::try_from_slice(&pda.data.borrow())?;

    if hex_token_account.key.to_string() != payment.hex_wallet {
        msg!("Hex wallet is invalid");
        return Err(ProgramError::InvalidArgument);
    }
    if shop_token_account.key.to_string() != payment.shop_wallet {
        msg!("Shop wallet is invalid");
        return Err(ProgramError::InvalidArgument);
    }

    let fee_amount = payment.amount as f64 * payment.fee;
    let fee_transfer = spl_token::instruction::transfer(
        token_program.key, sender_token_account.key,
        hex_token_account.key, sender.key,
        &[sender.key],
        fee_amount as u64,
    )?;
    invoke(&fee_transfer, &[
        sender_token_account.clone(), 
        hex_token_account.clone(),
        sender.clone(),
        token_program.clone()
    ])?;
    msg!("Transfered {} tokens to the hex wallet", fee_amount);

    let shop_amount = payment.amount as f64 * (1_f64 - payment.fee);
    let shop_transfer = spl_token::instruction::transfer(
        token_program.key, sender_token_account.key,
        shop_token_account.key, sender.key,
        &[sender.key],
        shop_amount as u64,
    )?;
    invoke(&shop_transfer, &[
        sender_token_account.clone(), 
        shop_token_account.clone(),
        sender.clone(),
        token_program.clone()
    ])?;
    msg!("Transfered {} tokens to the shop wallet", shop_amount);

    payment.status = String::from("paid");
    payment.serialize(&mut &mut pda.data.borrow_mut()[..])?;

    Ok(())

}

/// Accounts:
/// 
/// 0. `[signer, writable]` Admin account
/// 1. `[writable]` PDA account to write payment data
/// 2. `[]` System program
/// 3. `[]` Sysvar rent program
fn create_payment(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    payment_struct: Payment,
) -> ProgramResult {

    let accounts_iter = &mut accounts.iter();

    let admin = next_account_info(accounts_iter)?;
    let pda = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;
    let sysvar_rent = next_account_info(accounts_iter)?;

    if admin.key.to_string() != ADMIN_ACCOUNT_ID.to_string() {
        msg!("Access denied. Isn't an admin account");
        return Err(ProgramError::InvalidArgument);
    }
    
    let seed_copy = payment_struct.seed.clone();
    let (_, bump_seed) = Pubkey::find_program_address(&[seed_copy.as_bytes()], &program_id);
    let signer_seeds: &[&[_]] = &[seed_copy.as_bytes(), &[bump_seed]];

    let payment = Payment {
        seed: payment_struct.seed,
        amount: payment_struct.amount,
        fee: payment_struct.fee,
        status: payment_struct.status,
        shop_wallet: payment_struct.shop_wallet,
        hex_wallet: payment_struct.hex_wallet
    };

    let space = payment.try_to_vec()?.len();
    let rent = &Rent::from_account_info(sysvar_rent)?;
    let minimum_balance = rent.minimum_balance(space);

    invoke_signed(
        &system_instruction::create_account(
            admin.key,
            pda.key,
            minimum_balance,
            space as u64,
            program_id,
        ),
        &[admin.clone(), pda.clone(), system_program.clone()],
        &[&signer_seeds],
    )?;

    payment.serialize(&mut &mut pda.data.borrow_mut()[..])?;

    Ok(())
        
}