use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::{rent::Rent, Sysvar},
    token::{self, Mint, TokenAccount, Transfer},
};

entrypoint!(process_instruction);

fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    // Get the account that will receive the released tokens
    let recipient_account = next_account_info(accounts_iter)?;

    // Get the token mint account
    let token_mint_account = next_account_info(accounts_iter)?;

    // Get the token account owned by the program
    let program_token_account = next_account_info(accounts_iter)?;

    // Get the rent sysvar to verify that the recipient account is rent-exempt
    let rent_sysvar_account = next_account_info(accounts_iter)?;
    let rent = &Rent::from_account_info(rent_sysvar_account)?;

    // Verify that the recipient account is rent-exempt
    if !rent.is_exempt(recipient_account.lamports(), recipient_account.data_len()) {
        return Err(ProgramError::AccountNotRentExempt);
    }

    // Initialize token program
    let token_program_id = &token::ID;

    // Transfer tokens from the program's token account to the recipient account
    let transfer_instruction = Transfer {
        source: *program_token_account.key,
        destination: *recipient_account.key,
        amount: 100, // Specify the amount of tokens to transfer
    };
    token::transfer(program_token_account, &mut [recipient_account.clone()], transfer_instruction)?;
    Ok(())
}
