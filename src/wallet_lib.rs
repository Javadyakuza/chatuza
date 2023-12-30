use crate::models::QTronWallet;
use crate::models::TronWallet;
use crate::schema::tron_wallets;
use crate::schema::{solana_wallets::dsl::*, tron_wallets::dsl::*};
pub use diesel;
pub use diesel::pg::PgConnection;
pub use diesel::prelude::*;
pub use diesel::result::Error;
pub use dotenvy::dotenv;
pub use std::env;

pub fn initialize_new_tron_wallet(
    _conn: &mut PgConnection,
    _new_wallet_info: &TronWallet,
) -> Result<i32, String> {
    // checking if user already exists
    if let Some(_) = get_user_wallet(_conn, _new_wallet_info.user_id) {
        Err("user already has a wallet".to_owned())
    } else {

        diesel::insert_into(tron_wallets).values(_new_wallet_info)
        Ok(1)
    }
}

pub fn initialize_new_solana_wallet() -> Result<(), String> {
    Ok(())
}

pub fn get_user_tron_wallet_addr() -> Result<String, String> {
    Ok("".to_owned())
}

pub fn get_user_wallet_wallet_addr() -> Result<String, String> {
    Ok("".to_owned())
}

pub fn get_user_solana_wallet_pub_key() -> Result<String, String> {
    Ok("".to_owned())
}

pub fn get_user_tron_wallet_pub_key() -> Result<String, String> {
    Ok("".to_owned())
}

pub fn get_user_wallet(_conn: &mut PgConnection, _user_id: i32) -> Option<QTronWallet> {
    let wallets: Vec<QTronWallet> = tron_wallets
        .filter(tron_wallets::user_id.eq(_user_id))
        .select(QTronWallet::as_select())
        .load(_conn)
        .unwrap();

    if wallets.len() == 1 {
        Some(QTronWallet {
            wallet_id: wallets[0].wallet_id,
            user_id: wallets[0].user_id,
            pub_key: wallets[0].pub_key.clone(),
            wallet_addr: wallets[0].wallet_addr.clone(),
        })
    } else {
        None
    }
}
