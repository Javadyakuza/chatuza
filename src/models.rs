use diesel::prelude::*;
// use merge_derivable;
use struct_iterable::Iterable;

use crate::schema::chat_room_participants;

#[derive(Queryable, Selectable, Debug, Insertable, Iterable, PartialEq)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Users {
    // pub user_id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Queryable, Selectable, Debug, Insertable, Iterable, PartialEq)]
#[diesel(table_name = crate::schema::user_profiles)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserProfiles {
    // pub user_profile_id: i32,
    pub user_id: i32,
    pub bio: Option<String>,
    pub profile_picture: Option<String>,
}

#[derive(Queryable, Selectable, Debug, Insertable, Iterable)]
#[diesel(table_name = crate::schema::chat_rooms)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ChatRooms {
    // pub chat_room_id: i32,
    pub room_name: String,
    pub room_description: String,
}

#[derive(Queryable, Selectable, Debug, Insertable, Iterable)]
#[diesel(table_name = crate::schema::chat_room_participants)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ChatRoomParticipants {
    // pub participant_id: i32,
    pub chat_room_id: i32,
    pub user_id: i32,
    pub is_admin: bool,
}

#[derive(Queryable, Selectable, Debug, Insertable, Iterable)]
#[diesel(table_name = crate::schema::messages)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Messages {
    pub message_id: i32,
    pub sender_id: i32,
    pub recipient_id: i32,
    pub timestamp: diesel::data_types::PgTimestamp,
    pub content: String,
    pub is_read: bool,
    pub delivery_status: String,
    pub parent_message_id: Option<i32>,
}

// --  models with queryable primary keys -- //

#[derive(Queryable, Selectable, Debug, Insertable, Iterable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct QUsers {
    pub user_id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Queryable, Selectable, Debug, Insertable, Iterable)]
#[diesel(table_name = crate::schema::chat_rooms)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct QChatRooms {
    pub chat_room_id: i32,
    pub room_name: String,
    pub room_description: String,
}

#[derive(Queryable, Selectable, Debug, Insertable, Iterable)]
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
