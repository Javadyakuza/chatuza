use crate::api_models::{CreateTokenAccount, CreateTokenAccountResponse};
use crate::db_models::{QSolanaWallet, SolanaWallet};
use crate::schema::solana_wallets;
use crate::schema::solana_wallets::dsl::*;
use crate::{get_user_with_username, is_valid_user};
pub use diesel;
pub use diesel::pg::PgConnection;
pub use diesel::prelude::*;
pub use diesel::result::Error;
pub use dotenvy::dotenv;
use solana_client::rpc_client::RpcClient;
use solana_sdk::hash::Hash;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::{Keypair, Signature};
use solana_sdk::signer::{EncodableKey, Signer};
use solana_sdk::system_instruction::transfer;
use solana_sdk::transaction::Transaction;
use spl_associated_token_account::get_associated_token_address_with_program_id;
use spl_associated_token_account::instruction::create_associated_token_account;
pub use std::env;
use std::str::FromStr;

pub fn initialize_new_solana_wallet(
    _conn: &mut PgConnection,
    _new_wallet_info: &SolanaWallet,
) -> Result<QSolanaWallet, Box<dyn std::error::Error>> {
    // Checking if user already has a wallet
    if get_user_solana_wallet(_conn, _new_wallet_info.user_id).is_ok() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            "User already has a wallet".to_owned(),
        )));
    }

    // Checking if user ID is valid
    if !is_valid_user(_conn, _new_wallet_info.user_id) {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("user id {} not found !", _new_wallet_info.user_id),
        )));
    }

    match diesel::insert_into(solana_wallets::table)
        .values(_new_wallet_info)
        .returning(QSolanaWallet::as_returning())
        .get_result(_conn)
    {
        Ok(res) => Ok(res),
        Err(e) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "Couldn't initialize a Solana wallet for user ID {} \n
                        Error: {:?}",
                    &_new_wallet_info.user_id, e
                )
                .as_str(),
            )))
        }
    }
}

pub fn delete_solana_wallet(
    _conn: &mut PgConnection,
    _username: &String,
) -> Result<bool, Box<dyn std::error::Error>> {
    let _user_id_removable: i32;
    match get_user_with_username(_conn, _username) {
        Ok(res) => _user_id_removable = res.user_id,
        Err(e) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("{:?}", e),
            )))
        }
    }
    match diesel::delete(solana_wallets.filter(solana_wallets::user_id.eq(_user_id_removable)))
        .execute(_conn)
    {
        Ok(_) => Ok(true),
        Err(e) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "Couldn't delete the Solana wallet for user ID {} \n
                        Error: {:?}",
                    &_user_id_removable, e
                )
                .as_str(),
            )))
        }
    }
}

pub fn get_user_solana_wallet(
    _conn: &mut PgConnection,
    _user_id: i32,
) -> Result<QSolanaWallet, Box<dyn std::error::Error>> {
    let wallets: Vec<QSolanaWallet> = solana_wallets
        .filter(solana_wallets::user_id.eq(_user_id))
        .select(QSolanaWallet::as_select())
        .load(_conn)
        .unwrap_or(vec![]);

    if wallets.len() == 1 {
        Ok(wallets.into_iter().next().unwrap()) // panic impossible
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("user id {} doesn't have a solana wallet ", _user_id),
        )))
    }
}

pub fn create_token_account(
    token_account_info: &CreateTokenAccount,
) -> Result<CreateTokenAccountResponse, Box<dyn std::error::Error>> {
    let kp =
        Keypair::read_from_file("/home/javad/Desktop/chatuza_all/chatuza_db/sec.json").unwrap();
    let pk = kp.try_pubkey().unwrap();
    let rpc = RpcClient::new("https://api.devnet.solana.com".to_string());

    match rpc.send_and_confirm_transaction(&Transaction::new_signed_with_payer(
        &[create_associated_token_account(
            &pk,
            &Pubkey::from_str(token_account_info.wallet_address.as_str()).unwrap(),
            &Pubkey::from_str(token_account_info.token_mint_address.as_str()).unwrap(),
            &Pubkey::from_str(token_account_info.token_program_id.as_str()).unwrap(),
        )],
        Some(&pk),
        &[&kp],
        Hash::from_str(token_account_info.lbh.as_str()).unwrap(),
    )) {
        Ok(sig) => {
            // funding the account
            match rpc.send_and_confirm_transaction(&Transaction::new_signed_with_payer(
                &[transfer(
                    &pk,
                    &get_associated_token_address_with_program_id(
                        &Pubkey::from_str(token_account_info.wallet_address.as_str()).unwrap(),
                        &Pubkey::from_str(token_account_info.token_mint_address.as_str()).unwrap(),
                        &Pubkey::from_str(token_account_info.token_program_id.as_str()).unwrap(),
                    ),
                    1_000_000,
                )],
                Some(&pk),
                &[&kp],
                Hash::from_str(token_account_info.lbh.as_str()).unwrap(),
            )) {
                Ok(sig2) => Ok(CreateTokenAccountResponse {
                    signatures: vec![sig.to_string(), sig2.to_string()],
                }),
                Err(e2) => Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    format!("{}", e2),
                ))),
            }
        }
        Err(e) => Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("{}", e),
        ))),
    }
}

// we activate the the account of the user in exchange of some transferable spl token if not activated before
pub fn activate_wallet_account_for_transfer(
    wallet_pubkey: String,
) -> Result<String, Box<dyn std::error::Error>> {
    let kp =
        Keypair::read_from_file("/home/javad/Desktop/chatuza_all/chatuza_db/sec.json").unwrap();
    let pk: Pubkey = kp.try_pubkey().unwrap();
    let rpc = RpcClient::new("https://api.devnet.solana.com".to_string());
    let recipient_pk: Pubkey = Pubkey::from_str(wallet_pubkey.as_str()).unwrap(); // panic impossible
    let lbh: Hash;
    match rpc.get_latest_blockhash() {
        Ok(_lbh) => lbh = _lbh,
        Err(e) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("couldn't fetch the latest block hash due to \n {}", e),
            )))
        }
    }
    match rpc.send_and_confirm_transaction(&Transaction::new_signed_with_payer(
        &[solana_sdk::system_instruction::transfer(
            &pk,
            &recipient_pk,
            100_000_000,
        )],
        Some(&pk),
        &[&kp],
        lbh,
    )) {
        Ok(sig) => Ok(sig.to_string()),
        Err(e) => Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!(" couldn't fund the recipient due to \n {}", e),
        ))),
    }
}
