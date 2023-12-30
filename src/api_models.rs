// use rocket::data::FromDataSimple;
use rocket::request::Form;
use rocket::*;

use serde::Serialize;

use crate::*;

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
    pub chat_room_id_in: i32,
    pub remover_user_id_in: i32,
}

#[derive(FromForm, Debug, Serialize)]
pub struct NewGroupChatRoomIN {
    pub room_name_in: String,
    pub room_description_in: String,
    pub group_owner_id_in: i32,
    pub group_members_in: String, // then will be converted to the array of the chat room participants instance
}

#[derive(FromForm, Debug, Serialize)]
pub struct DeleteGroupChatRoomIN {
    pub chat_room_name_in: String,
    pub remover_user_id_in: i32,
}

#[derive(FromForm, Debug, Serialize)]
pub struct UpdatedGroupChatRoomInfoIN {
    pub old_chat_room_name_in: String,
    pub room_name_in: String,
    pub room_description_in: String,
    pub editor_user_id_in: i32,
}

#[derive(FromForm, Debug, Serialize)]
pub struct NewGroupChatParticipantIN {
    pub user_id_in: i32,
    pub chat_room_id_in: i32,
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
