// use rocket::data::FromDataSimple;
use rocket::*;

use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;

#[derive(FromForm, Debug, Serialize)]
pub struct NewUserIN {
    pub username_in: String,
    pub email_in: String,
    pub password_in: String,
    pub phone_number_in: String,
    pub bio_in: Option<String>,
    pub profile_picture_in: Option<String>,
}

#[derive(FromForm, Debug, Serialize)]
pub struct UpdatedUserCreditsIN {
    pub username_out: String,
    pub username_in: String,
    pub email_in: String,
    pub password_in: String,
    pub phone_number_in: String,
}

#[derive(FromForm, Debug, Serialize)]
pub struct UpdatedUserProfileIN {
    pub username_in: String,
    pub bio_in: String,
    pub profile_picture_in: String,
}

#[derive(FromForm, Debug, Serialize)]
pub struct SinglePostUsername {
    pub username_in: String,
}
#[derive(FromForm, Debug, Serialize)]
pub struct CreateTokenAccount {
    pub wallet_address: String,
    pub token_mint_address: String,
    pub token_program_id: String,
    pub lbh: String,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct CreateTokenAccountResponse {
    pub signatures: Vec<String>,
}

// delete user only takes one arg //

// get user with username takes only one argument //

// get user with email takes only one argument //

// get user with user id takes only one argument //

#[derive(FromForm, Debug, Serialize)]
pub struct NewP2PChatRoomIN {
    pub requestor_username_in: String,
    pub acceptor_username_in: String,
    pub chat_room_pubkey_in: String,
}
#[derive(FromForm, Debug, Serialize)]
pub struct DeleteP2PChatRoomIN {
    pub remover_username_in: String,
    pub contact_username_in: String,
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct NewGroupChatRoomIN {
    pub room_name_in: String,
    pub room_description_in: String,
    pub group_owner_username_in: String,
    pub group_members_in: Vec<String>,
    pub chat_room_pubkey: String,
}

#[derive(FromForm, Debug, Serialize)]
pub struct DeleteGroupChatRoomIN {
    pub chat_room_name_in: String,
    pub remover_username_in: String,
}

#[derive(FromForm, Debug, Serialize)]
pub struct UpdatedGroupChatRoomInfoIN {
    pub old_chat_room_name_in: String,
    pub room_name_in: String,
    pub room_description_in: String,
    pub editor_username_in: String,
}

#[derive(FromForm, Debug, Serialize)]
pub struct NewGroupChatParticipantIN {
    pub username_in: String,
    pub chat_room_name_in: String,
    pub adder_username_in: String,
}

#[derive(FromForm, Debug, Serialize)]
pub struct GroupChatParticipantToRemoveIN {
    pub chat_room_name_in: String,
    pub username_in: String,
    pub remover_username_in: String,
}

// get functions are getting only one argument

#[derive(FromForm, Debug, Serialize)]
pub struct NewWalletIn {
    pub username_in: String,
    pub wallet_addr_in: String,
    pub wallet_backup_in: String,
}
