use crate::is_valid_user;
use crate::models::{QSolanaWallet, QTronWallet, SolanaWallet, TronWallet};
use crate::schema::{solana_wallets, tron_wallets};
use crate::schema::{solana_wallets::dsl::*, tron_wallets::dsl::*};
pub use diesel;
pub use diesel::pg::PgConnection;
pub use diesel::prelude::*;
pub use diesel::result::Error;
pub use dotenvy::dotenv;
pub use std::env;

pub fn initialize_new_tron_wallet(
    _conn: &mut PgConnection,
    new_wallet_info: &TronWallet,
) -> Result<i32, String> {
    // Checking if user already has a wallet
    if get_user_tron_wallet(_conn, new_wallet_info.user_id).is_some() {
        return Err("User already has a wallet".to_owned());
    }

    // Checking if user ID is valid
    if !is_valid_user(_conn, new_wallet_info.user_id) {
        return Err("Invalid user ID provided".to_owned());
    }

    let new_wallet_info: QTronWallet = diesel::insert_into(tron_wallets::table)
        .values(new_wallet_info)
        .returning(QTronWallet::as_returning())
        .get_result(_conn)
        .map_err(|_| {
            format!(
                "Couldn't initialize a Solana wallet for user ID {}",
                new_wallet_info.user_id
            )
        })?;

    Ok(new_wallet_info.user_id)
}

pub fn initialize_new_solana_wallet(
    _conn: &mut PgConnection,
    new_wallet_info: &SolanaWallet,
) -> Result<i32, String> {
    // Checking if user already has a wallet
    if get_user_solana_wallet(_conn, new_wallet_info.user_id).is_some() {
        return Err("User already has a wallet".to_owned());
    }

    // Checking if user ID is valid
    if !is_valid_user(_conn, new_wallet_info.user_id) {
        return Err("Invalid user ID provided".to_owned());
    }

    let new_wallet_info: QSolanaWallet = diesel::insert_into(solana_wallets::table)
        .values(new_wallet_info)
        .returning(QSolanaWallet::as_returning())
        .get_result(_conn)
        .map_err(|_| {
            format!(
                "Couldn't initialize a Solana wallet for user ID {}",
                new_wallet_info.user_id
            )
        })?;

    Ok(new_wallet_info.user_id)
}

pub fn get_user_tron_wallet(_conn: &mut PgConnection, _user_id: i32) -> Option<QTronWallet> {
    let wallets = tron_wallets
        .filter(tron_wallets::user_id.eq(_user_id))
        .select(QTronWallet::as_select())
        .load::<QTronWallet>(_conn)
        .unwrap();

    assert!(wallets.len() == 1);

    wallets.into_iter().next()
}

pub fn get_user_solana_wallet(_conn: &mut PgConnection, _user_id: i32) -> Option<QSolanaWallet> {
    let wallets = solana_wallets
        .filter(solana_wallets::user_id.eq(_user_id))
        .select(QSolanaWallet::as_select())
        .load::<QSolanaWallet>(_conn)
        .unwrap();

    assert!(wallets.len() != 1);

    wallets.into_iter().next()
}