// use rocket::data::FromDataSimple;
use rocket::request::Form;
use rocket::*;

use serde::Serialize;

use crate::*;

#[derive(FromForm, Debug, Serialize)]
pub struct NewUserIN {
    pub credits_in: Users,
    pub profile_in: UserProfiles,
}

#[derive(FromForm, Debug, Serialize)]
pub struct UpdatedUserCreditsIN {
    pub user_id_in: i32,
    pub username_in: String,
    pub email_in: String,
    pub password_in: String,
}

#[derive(FromForm, Debug, Serialize)]
pub struct UpdatedUserProfileIN {
    pub user_id_in: i32,
    pub bio_in: String,
    pub profile_picture_in: String,
}

// delete user only takes one arg //

// get user with username takes only one argument //

// get user with email takes only one argument //

// get user with user id takes only one argument //

#[derive(FromForm, Debug, Serialize)]
pub struct NewP2PChatRoomIN {
    pub requestor_user_in: i32,
    pub acceptor_user_in: i32,
}

#[derive(FromForm, Debug, Serialize)]
pub struct DeleteP2PChatRoomIN {
    pub chat_room_id_in: i32,
    pub remover_user_id_in: i32,
}

#[derive(FromForm, Debug, Serialize)]
pub struct NewGroupChatRoomIN {
    pub chat_room_info_in: ChatRooms,
    pub group_owner_id_in: i32,
    pub group_members_in: Vec<ChatRoomParticipants>, // Note: members should not include the owner
}

#[derive(FromForm, Debug, Serialize)]
pub struct DeleteGroupChatRoomIN {
    pub chat_room_name_in: String,
    pub remover_user_id_in: i32,
}

#[derive(FromForm, Debug, Serialize)]
pub struct UpdatedGroupChatRoomInfoIN {
    pub old_chat_room_name_in: String,
    pub new_chat_room_info_in: ChatRooms,
    pub editor_user_id_in: i32,
}

#[derive(FromForm, Debug, Serialize)]
pub struct NewGroupChatParticipantIN {
    pub user_id_in: i32,
    pub chat_room_id_in: i32,
}

#[derive(FromForm, Debug, Serialize)]
pub struct GroupChatParticipantToRemoveIN {
    pub removing_user_in: ChatRoomParticipants,
    pub remover_user_id_in: i32,
}

// get functions are getting only one argument
