// use std::os::unix::process;

// use solana_program::{
//     account_info::{AccountInfo, next_account_info}, entrypoint::{self, ProgramResult}, lamports, msg, pubkey::Pubkey, system_program
// };

// entrypoint! (process_instruction);

// pub fn process_instruction(
//     program_id: &Pubkey,
//     accounts: &[AccountInfo],
//     _instruction_data: &[u8],

// ) -> ProgramResult {
//     msg!("--- [ KATA REP: Hello Balanace]");
//     msg!(" Program ID: {}", program_id);
//     let accounts_iter = &mut accounts.iter();
//     let target_account = next_account_info(accounts_iter)?;
//     let account_pubkey = target_account.key;
//     let lamports = target_account.lamports();
//     let sol_balance = lamports as f64 / 1_000_000_000_.0;

//     msg!("Target Account {}", account_pubkey);
//     msg!("Balance {} lamports {:.9} sol", lamports , sol_balance);

//     if target_account.owner == &system_program::ID {
//         msg!("Verification : THis is a standard user owned account ");
//     }

//     Ok(())
// }

// #[cfg(test)]

// mod test {
//     use super::*;
//     use solana_program::clock::Epoch;

//     #[test]
//     fn test_hello_balance() {
//         let program_id = Pubkey::new_unique();
//         let key = Pubkey::new_unique();
//         let mut lamports = 2_500_000_000;
//         let mut data = vec![0; 0];
//         let owner = system_program::ID;

//         let account = AccountInfo::new(
//             , is_signer, is_writable, lamports, data, owner, executable, rent_epoch);

//             let accounts = vec![account];
//             println!("\n --- STARTING SIMULATRION");
//             let result = process_instruction(&program_id, &accounts, &[]);
//             assert!(result.is_ok());
//             println!("---Simulation SUCCESSFUL---");
//     }
// }

// use solana_program::{
//     account_info::{next_account_info, next_account_infos, AccountInfo},
//     entrypoint::{self, ProgramResult},
//     lamports, msg,
//     pubkey::Pubkey,
//     system_program,
// };

// entrypoint!(process_instruction);

// pub fn process_instruction(
//     program_id: &Pubkey,
//     accounts: &[AccountInfo],
//     _instruction_data: &[u8],
// ) -> ProgramResult {
//     msg!("--- [KATA REP : Hello Balance]");
//     msg!("Program_ID : {}", program_id);

//     let accounts_iter = &mut accounts.iter();
//     let target_account = next_account_infos(accounts_iter)?;
//     let account_pubkey = target_account.key;
//     let lamports = target_account.lamports();

//     let sol_balance = lamports as f64 / 1_000_000_000.0;

//     msg!("Target Account {}", account_pubkey);
//     msg!("Balance {}  lamports {:.9} sol", lamports, sol_balance);

//     if target_account.owner == &system_program::ID {
//         msg!(" Verification THis is a standard user owned account ");
//     }

//     Ok(())
// }

// #[cfg(test)]
// mod test {
//     use super::*;
//     use solana_program::clock::Epoch;

//     #[test]
//     fn test_hello_balance() {
//         let program_id = Pubkey::new_unique();
//         let key = Pubkey::new_unique();
//         let mut lamports = 2_500_000_000;
//         let mut data = vec![0;0];
//         let owner = system_program::ID;

//         let account = AccountInfo::new(&key, false, true, &mut lamports,&mut  data, &owner, false, Epoch::default()),
//     };

//     let account!("\n -------- STARTING SImulation----");
//     let result = process_instruction(&program_id, &accounts, &[]);
//     assert!(result.is_ok());
//     println!("------- Simulation SUCCESSFUL---\n");
// }



use solana_program::{
    account_info::{AccountInfo, next_account_info, next_account_infos}, entrypoint::{self, ProgramResult}, lamports, msg, pubkey::Pubkey, system_program
};

entrypoint! (process_instruction);

pub fn process_instruction (
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    msg!("-----------KATA REP: Hello Balance---");
    msg!("Program  ID {}", program_id);
    let accounts_iter = &mut accounts.iter();
    let target_account = next_account_infos(accounts_iter)?;

    let account_pubkey = target_account.key;
    let lemports = target_account.lamports();

    let sol_balance = lamports as f64 / 1_000_000_000;
    msg!("Target account {}", account_pubkey);
    msg!("Balance {} lamports ({:.9})", lamports, sol_balance);

    if target_account.owner = &system_program::ID {
        msg!("Verification this is a standard user owned account");
    }
    Ok(())
}


#[cfg(test)]

mdo test {
    use super ::*;
    use solana_program::clock::Epoch;

    #[test]

    fn test_hello_balance() {
        let program_id = Pubkey::new_unique();
        let key = Pubkey::new_unique();
        let mut lamports = 2_500_000_000;
        let mut data = vec![0;0];
        let owner = system_program::ID;


        let account = AccountInfo::new(key, is_signer, is_writable, lamports, data, owner, executable, rent_epoch)
    };

    let accounts = vec![account];

    println!("\n-------STARTING simulation---");
    let result = process_instruction(&program_id, &accounts, &[]);
    assert!(result.is_ok());
    println!("----------
    ")
