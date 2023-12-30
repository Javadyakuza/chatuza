use crate::db_models::{QSolanaWallet, QTronWallet, SolanaWallet, TronWallet};
use crate::schema::{solana_wallets, tron_wallets};
use crate::schema::{solana_wallets::dsl::*, tron_wallets::dsl::*};
use crate::{get_user_with_username, is_valid_user};
pub use diesel;
pub use diesel::pg::PgConnection;
pub use diesel::prelude::*;
pub use diesel::result::Error;
pub use dotenvy::dotenv;
pub use std::env;

pub fn initialize_new_tron_wallet(
    _conn: &mut PgConnection,
    new_wallet_info: &TronWallet,
) -> Result<QTronWallet, Box<dyn std::error::Error>> {
    // Checking if user already has a wallet
    if get_user_tron_wallet(_conn, new_wallet_info.user_id).is_ok() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            "User already has a wallet".to_owned(),
        )));
    }

    // Checking if user ID is valid
    if !is_valid_user(_conn, new_wallet_info.user_id) {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            "Invalid user ID provided".to_owned(),
        )));
    }

    let new_wallet_info: QTronWallet = diesel::insert_into(tron_wallets::table)
        .values(new_wallet_info)
        .returning(QTronWallet::as_returning())
        .get_result(_conn)
        .expect(
            format!(
                "Couldn't initialize a Tron wallet for user ID {}",
                new_wallet_info.user_id
            )
            .as_str(),
        );

    Ok(new_wallet_info)
}

pub fn initialize_new_solana_wallet(
    _conn: &mut PgConnection,
    new_wallet_info: &SolanaWallet,
) -> Result<QSolanaWallet, Box<dyn std::error::Error>> {
    // Checking if user already has a wallet
    if get_user_solana_wallet(_conn, new_wallet_info.user_id).is_ok() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            "User already has a wallet".to_owned(),
        )));
    }

    // Checking if user ID is valid
    if !is_valid_user(_conn, new_wallet_info.user_id) {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            "Invalid user ID provided".to_owned(),
        )));
    }

    let new_wallet_info: QSolanaWallet = diesel::insert_into(solana_wallets::table)
        .values(new_wallet_info)
        .returning(QSolanaWallet::as_returning())
        .get_result(_conn)
        .expect(
            format!(
                "Couldn't initialize a Solana wallet for user ID {}",
                new_wallet_info.user_id
            )
            .as_str(),
        );

    Ok(new_wallet_info)
}

pub fn delete_tron_wallet(
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
    } // checking if it exist before or no.
    diesel::delete(tron_wallets.filter(tron_wallets::user_id.eq(_user_id_removable)))
        .execute(_conn)
        .expect("couldn't delete user tron wallet");

    Ok(true)
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
    diesel::delete(solana_wallets.filter(solana_wallets::user_id.eq(_user_id_removable)))
        .execute(_conn)
        .expect("couldn't delete user solana wallet");
    Ok(true)
}

pub fn get_user_tron_wallet(
    _conn: &mut PgConnection,
    _user_id: i32,
) -> Result<QTronWallet, Box<dyn std::error::Error>> {
    let wallets: Vec<QTronWallet> = tron_wallets
        .filter(tron_wallets::user_id.eq(_user_id))
        .select(QTronWallet::as_select())
        .load(_conn)
        .unwrap_or(vec![]);

    if wallets.len() == 1 {
        Ok(wallets.into_iter().next().unwrap())
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("user id {} doesn't have a tron wallet ", _user_id),
        )))
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
