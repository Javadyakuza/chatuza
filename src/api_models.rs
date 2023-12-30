// use rocket::data::FromDataSimple;
use rocket::request::Form;
use rocket::*;

use crate::*;
// use rocket_contrib::json;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

#[derive(FromForm, Debug, Serialize)]
pub struct NewUserIN {
    pub username_in: String,
    pub email_in: String,
    pub password_in: String,
    pub bio_in: Option<String>,
    pub profile_picture_in: Option<String>,
}

#[derive(FromForm, Debug, Serialize)]
pub struct UpdatedUserCreditsIN {
    pub username_out: String,
    pub username_in: String,
    pub email_in: String,
    pub password_in: String,
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
}

#[derive(FromForm, Debug, Serialize)]
pub struct GroupChatParticipantToRemoveIN {
    pub chat_room_id_in: i32,
    pub user_id_in: i32,
    pub is_admin_in: bool,
    pub remover_user_id_in: i32,
}

// get functions are getting only one argument

#[derive(FromForm, Debug, Serialize)]
pub struct NewTronWalletIn {
    pub user_id_in: i32,
    pub wallet_addr: String,
}

#[derive(FromForm, Debug, Serialize)]
pub struct NewSolanaWalletIn {
    pub user_id_in: i32,
    pub wallet_addr_in: String,
}
