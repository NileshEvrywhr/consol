use borsh::{BorshDeserialize, BorshSerialize};      // tells compiler to make ProgramData serializable
use std::collections::HashMap;
use std::convert::TryInto;

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    // program_error::ProgramError,
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]  
pub struct ProgramData {
    pub campaign_amounts: HashMap<String, u64>,
    pub campaign_descriptions: HashMap<String, String>,
    pub campaign_fulfilled: HashMap<String, u64>,
}

entrypoint!(process_instruction);

pub fn process_instruction (

    _program_id : &Pubkey,
    accounts : &[AccountInfo],
    data : &[u8],
  
  ) -> ProgramResult {

    // instructions on what to do
    let (instruction_byte, all_other_bytes) = data.split_first().unwrap();
    
    // get accounts
    let iterable_accounts = &mut accounts.iter();
    let this_program_account = next_account_info(iterable_accounts)?;    // contract

    // get amounts, initial or contribution
    let amount = all_other_bytes
                    .get(..8)
                    .and_then(|slice| slice.try_into().ok())
                    .map(u64::from_le_bytes)
                    .unwrap();                                          // unwrap's job is to deserialize objects coming from the chain?
    
    // extract description
    let _description = String::from_utf8(all_other_bytes[9..].to_vec()).unwrap();

    
    if *instruction_byte == 0 {
    // create campaign

        let campaign_owner = next_account_info(iterable_accounts)?;     // owner 
        
        // get owner public key; to sign and send back data?
        let key = String::from_utf8(campaign_owner.owner.to_bytes().to_vec()).unwrap(); 
        
        // fill the above firstly created structure
        let mut program_account_data = ProgramData::try_from_slice(&this_program_account.data.borrow())?;

        program_account_data.campaign_amounts.insert(key, amount);
        // program_account_data.campaign_descriptions.insert(key, description);
        // program_account_data.campaign_fulfilled.insert(key, 0);

        program_account_data.serialize(&mut &mut this_program_account.data.borrow_mut()[..])?;

    }
    else if *instruction_byte == 1 {
    // fund a campaign

    }
    else if *instruction_byte == 2 {
    // get how much funds are left to reach the requested amount

    }
    else if *instruction_byte == 3 {
    // withdraw all collected funds and close campaign

    }
    else if *instruction_byte == 4 {
        // getting status of the campaign

        // how is next_account_info working, iterable_accounts me kya store h?
        let campaign_owner = next_account_info(iterable_accounts)?;

        // again? move to global scope?
        let key = String::from_utf8(campaign_owner.owner.to_bytes().to_vec()).unwrap();

        // print amount
        let program_account_data = ProgramData::try_from_slice(&this_program_account.data.borrow())?;
        msg!("{}",program_account_data.campaign_amounts.get(&key).unwrap() - program_account_data.campaign_fulfilled.get(&key).unwrap());

    }

    Ok(())

  }