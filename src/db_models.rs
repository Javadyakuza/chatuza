use diesel::prelude::*;
// use merge_derivable;
use crate::schema::chat_room_participants;
use rocket::*;
use serde::Serialize;
use struct_iterable::Iterable;

#[derive(Queryable, FromForm, Selectable, Debug, Insertable, Iterable, Serialize, PartialEq)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Users {
    // pub user_id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Queryable, FromForm, Selectable, Debug, Insertable, Iterable, Serialize, PartialEq)]
#[diesel(table_name = crate::schema::user_profiles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserProfiles {
    // pub user_profile_id: i32,
    pub user_id: i32,
    pub bio: Option<String>,
    pub profile_picture: Option<String>,
}

#[derive(Queryable, FromForm, Selectable, Debug, Insertable, Iterable, Serialize)]
#[diesel(table_name = crate::schema::chat_rooms)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ChatRooms {
    // pub chat_room_id: i32,
    pub room_name: String,
    pub room_description: String,
}

#[derive(Queryable, FromForm, Selectable, Debug, Insertable, Iterable, Serialize)]
#[diesel(table_name = crate::schema::chat_room_participants)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ChatRoomParticipants {
    // pub participant_id: i32,
    pub chat_room_id: i32,
    pub user_id: i32,
    pub is_admin: bool,
}

#[derive(Queryable, FromForm, Selectable, Debug, Insertable, Iterable, Serialize)]
#[diesel(table_name = crate::schema::tron_wallets)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TronWallet {
    pub user_id: i32,
    pub wallet_addr: String,
}

#[derive(Queryable, FromForm, Selectable, Debug, Insertable, Iterable, Serialize)]
#[diesel(table_name = crate::schema::solana_wallets)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SolanaWallet {
    pub user_id: i32,
    pub wallet_addr: String,
}
// --  models with queryable primary keys -- //

#[derive(Queryable, FromForm, Selectable, Debug, Insertable, Iterable, Serialize)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct QUsers {
    pub user_id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Queryable, FromForm, Selectable, Debug, Insertable, Iterable, Serialize)]
#[diesel(table_name = crate::schema::chat_rooms)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct QChatRooms {
    pub chat_room_id: i32,
    pub room_name: String,
    pub room_description: String,
}

#[derive(Queryable, FromForm, Selectable, Debug, Insertable, Iterable, Serialize)]
#[diesel(table_name = crate::schema::chat_room_participants)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct QChatRoomParticipants {
    pub participant_id: i32,
    pub chat_room_id: i32,
    pub user_id: i32,
    pub is_admin: bool,
}

macro_rules! allow_group_by {
    ($group_by:ty, $($col_for: ty),+) => {
        $(
            impl
                ::diesel::expression::ValidGrouping<$group_by> for $col_for
            {
                type IsAggregate = ::diesel::expression::is_aggregate::Yes;
            }
        )+
    };
}

allow_group_by!(
    chat_room_participants::columns::chat_room_id,
    chat_room_participants::columns::participant_id,
    chat_room_participants::user_id,
    chat_room_participants::is_admin
);

#[derive(Queryable, FromForm, Selectable, Debug, Insertable, Iterable, Serialize)]
#[diesel(table_name = crate::schema::solana_wallets)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct QSolanaWallet {
    pub wallet_id: i32,
    pub user_id: i32,
    pub wallet_addr: String,
}

#[derive(Queryable, FromForm, Selectable, Debug, Insertable, Iterable, Serialize)]
#[diesel(table_name = crate::schema::tron_wallets)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct QTronWallet {
    pub wallet_id: i32,
    pub user_id: i32,
    pub wallet_addr: String,
}