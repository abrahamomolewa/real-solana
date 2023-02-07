use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey
};

entrypoint!(process_intruction);


fn process_intruction(
    _pubkey: &Pubkey,
    _accounts: &[AccountInfo],
    _instruction_data: &[u8]
) -> ProgramResult {
    msg!("hello this is sol ");
    Ok(())
}
