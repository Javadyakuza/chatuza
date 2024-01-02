#![recursion_limit = "256"]
pub mod api_models;
pub mod db_models;
pub mod schema;
pub mod wallet_lib;

use crate::db_models::{ChatRoomParticipants, ChatRooms, QUsers, UserProfiles, Users};
use crate::schema::{
    chat_room_participants, chat_rooms, solana_wallets, tron_wallets, user_profiles, users,
};
use chrono::Local;
use db_models::{QChatRooms, QSolanaWallet, QTronWallet, UpdatableChatRooms};
pub use diesel;
pub use diesel::pg::PgConnection;
pub use diesel::prelude::*;
pub use diesel::result::Error;
pub use dotenvy::dotenv;
use schema::{
    chat_room_participants::dsl::*, chat_rooms::dsl::*, solana_wallets::dsl::*,
    tron_wallets::dsl::*, user_profiles::dsl::*, users::dsl::*,
};
use wallet_lib::{delete_solana_wallet, delete_tron_wallet};

pub use std::env;
use std::hash::{DefaultHasher, Hash, Hasher};

pub fn establish_connection() -> PgConnection {
    // loading the env vars into the current scope
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

// -- Users / UserProfiles SETTER functions -- //
pub fn add_new_user(
    conn: &mut PgConnection,
    user_credits: &Users,
    user_profile: &mut UserProfiles,
) -> Result<QUsers, Box<dyn std::error::Error>> {
    // inserting user credits
    if let Err(e) = diesel::insert_into(users::table)
        .values(user_credits)
        .returning(Users::as_returning())
        .get_result(conn)
    {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("{:?}", e),
        )));
    }

    // fetching the user info
    let user_info: QUsers;
    match get_user_with_username(conn, &user_credits.username) {
        Ok(res) => user_info = res,
        Err(e) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("{:?}", e),
            )))
        }
    }

    user_profile.user_id = user_info.user_id;
    let u_p: &UserProfiles = user_profile;

    // inserting user profiles
    match diesel::insert_into(user_profiles::table)
        .values(u_p)
        .returning(UserProfiles::as_returning())
        .get_result(conn)
    {
        Ok(_) => Ok(user_info),
        Err(e) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("{:?}", e),
            )))
        }
    }
}

pub fn update_user_credits(
    conn: &mut PgConnection,
    old_username: &String,
    new_user_credits: &Users,
) -> Result<QUsers, Box<dyn std::error::Error>> {
    let user_info: QUsers;
    match get_user_with_username(conn, old_username.as_str()) {
        Ok(res) => user_info = res,
        Err(e) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("{:?}", e),
            )))
        }
    }

    match diesel::update(users.filter(users::user_id.eq(user_info.user_id)))
        .set((
            username.eq(&new_user_credits.username),
            email.eq(&new_user_credits.email),
            password.eq(&new_user_credits.password),
        ))
        .returning(Users::as_returning())
        .get_result(conn)
    {
        Ok(_) => Ok(get_user_with_username(conn, new_user_credits.username.as_str()).unwrap()),
        Err(e) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("{:?}", e),
            )))
        }
    }
}

pub fn update_user_profile(
    conn: &mut PgConnection,
    old_username: &String,
    user_profile: &mut UserProfiles,
) -> Result<UserProfiles, Box<dyn std::error::Error>> {
    //fetching the user id and set it accordingly
    user_profile.user_id = get_user_with_username(conn, &old_username).unwrap().user_id;

    match diesel::update(user_profiles.filter(user_profiles::user_id.eq(user_profile.user_id)))
        .set((
            bio.eq(&user_profile.bio),
            profile_picture.eq(&user_profile.profile_picture),
        ))
        .returning(UserProfiles::as_returning())
        .get_result(conn)
    {
        Ok(_) => Ok(get_user_profile_with_user_id(conn, user_profile.user_id).unwrap()),
        Err(e) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("{:?}", e),
            )))
        }
    }
}

pub fn delete_user(
    conn: &mut PgConnection,
    _username: &String,
) -> Result<bool, Box<dyn std::error::Error>> {
    let _user_id: i32;
    match get_user_with_username(conn, _username.as_str()) {
        Ok(res) => _user_id = res.user_id,
        Err(e) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("{:?}", e),
            )))
        }
    }
    // deleting the users p2p chatroom's
    if let Ok(_chat_rooms) = get_user_p2p_chat_rooms_by_user_id(conn, _user_id) {
        for chat_room in _chat_rooms {
            let p2p_chat_room_participants =
                get_chat_room_participants_by_id(conn, chat_room.chat_room_id).unwrap();
            let remover_username =
                get_user_with_user_id(conn, p2p_chat_room_participants[0].user_id)
                    .unwrap()
                    .username;
            let contact_username =
                get_user_with_user_id(conn, p2p_chat_room_participants[1].user_id)
                    .unwrap()
                    .username;
            let _ = delete_p2p_chat_room(conn, &remover_username, &contact_username).unwrap();
        }
    }
    if let Ok(gps) = get_user_group_chat_rooms_by_user_id(conn, _user_id) {
        for gp in gps {
            let group_owner = get_group_owner_by_id(conn, gp.chat_room_id).unwrap();
            if group_owner == _user_id {
                // deleting users owned group chats
                let _ = delete_group_chat_room(conn, &gp.room_name, &_username).unwrap();
            } else {
                // deleting the user from group chats
                let _ = del_participant_from_group_chat_room(
                    conn,
                    &ChatRoomParticipants {
                        user_id: _user_id,
                        chat_room_id: gp.chat_room_id,
                        is_admin: false,
                    },
                    group_owner,
                )
                .unwrap();
            }
        }
    }

    if let Err(e) =
        diesel::delete(user_profiles.filter(user_profiles::user_id.eq(_user_id))).execute(conn)
    {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("{:?}", e),
        )));
    }
    // checking if the user has any wallets created and deleting them
    let _tron_wallets = tron_wallets
        .filter(tron_wallets::user_id.eq(_user_id))
        .select(QTronWallet::as_select())
        .load(conn)
        .unwrap_or(vec![]);

    let _solana_wallets = solana_wallets
        .filter(solana_wallets::user_id.eq(_user_id))
        .select(QSolanaWallet::as_select())
        .load(conn)
        .unwrap_or(vec![]);

    if _tron_wallets.len() as u32 == 1 {
        delete_tron_wallet(conn, _username).unwrap();
    }

    if _solana_wallets.len() as u32 == 1 {
        delete_solana_wallet(conn, _username).unwrap();
    }
    match diesel::delete(users.filter(users::user_id.eq(_user_id))).execute(conn) {
        Ok(_) => Ok(true),
        Err(e) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("{:?}", e),
            )))
        }
    }
}

// -- Users / UserProfiles GETTER functions -- //

// EH
pub fn get_user_with_username(
    _conn: &mut PgConnection,
    _username: &str,
) -> Result<QUsers, Box<dyn std::error::Error>> {
    let user_row: Vec<QUsers> = users
        .filter(username.eq(&_username))
        .select(QUsers::as_select())
        .load(_conn)
        .unwrap_or(vec![]);
    if user_row.len() == 1 {
        Ok(QUsers {
            user_id: user_row[0].user_id,
            username: user_row[0].username.clone(),
            email: user_row[0].email.clone(),
            password: user_row[0].password.clone(),
        })
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "username not found !",
        )))
    }
}
// EH
pub fn get_user_with_email(
    _conn: &mut PgConnection,
    _email: &str,
) -> Result<QUsers, Box<dyn std::error::Error>> {
    let user_row: Vec<QUsers> = users
        .filter(email.eq(&_email))
        .select(QUsers::as_select())
        .load(_conn)
        .unwrap_or(vec![]);
    if user_row.len() == 1 {
        Ok(QUsers {
            user_id: user_row[0].user_id,
            username: user_row[0].username.clone(),
            email: user_row[0].email.clone(),
            password: user_row[0].password.clone(),
        })
    } else {
        // some thing is wrong
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "email not found !",
        )))
    }
}
// EH
pub fn get_user_with_user_id(
    _conn: &mut PgConnection,
    _user_id: i32,
) -> Result<QUsers, Box<dyn std::error::Error>> {
    let user_row: Vec<QUsers> = users
        .filter(users::user_id.eq(&_user_id))
        .select(QUsers::as_select())
        .load(_conn)
        .unwrap_or(vec![]);
    if user_row.len() == 1 {
        Ok(QUsers {
            user_id: user_row[0].user_id,
            username: user_row[0].username.clone(),
            email: user_row[0].email.clone(),
            password: user_row[0].password.clone(),
        })
    } else {
        // some thing is wrong
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "user id not found !",
        )))
    }
}
// EH
pub fn get_user_profile_with_user_id(
    _conn: &mut PgConnection,
    _user_id: i32,
) -> Result<UserProfiles, Box<dyn std::error::Error>> {
    let user_row: Vec<UserProfiles> = user_profiles
        .filter(user_profiles::user_id.eq(&_user_id))
        .select(UserProfiles::as_select())
        .load(_conn)
        .unwrap_or(vec![]);

    if user_row.len() == 1 {
        Ok(UserProfiles {
            user_id: _user_id,
            bio: user_row[0].bio.clone(),
            profile_picture: user_row[0].profile_picture.clone(),
        })
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "user id not found !",
        )))
    }
}
// EH
pub fn get_user_profile_with_username(
    _conn: &mut PgConnection,
    _username: &String,
) -> Result<UserProfiles, Box<dyn std::error::Error>> {
    let _user_id;
    match get_user_with_username(_conn, _username.as_str()) {
        Ok(res) => _user_id = res.user_id,
        Err(e) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("{:?}", e),
            )))
        }
    }
    let user_row: Vec<UserProfiles> = user_profiles
        .filter(user_profiles::user_id.eq(&_user_id))
        .select(UserProfiles::as_select())
        .load(_conn)
        .unwrap_or(vec![]);

    if user_row.len() == 1 {
        Ok(UserProfiles {
            user_id: _user_id,
            bio: user_row[0].bio.clone(),
            profile_picture: user_row[0].profile_picture.clone(),
        })
    } else {
        // some thing is wrong
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "username not found !",
        )))
    }
}

// -- ChatRooms/ Message history SETTER functions -- //

pub fn add_new_p2p_chat_room(
    _conn: &mut PgConnection,
    requestor_user: i32,
    acceptor_user: i32,
    _chat_room_pubkey: String,
) -> Result<QChatRooms, Box<dyn std::error::Error>> {
    // checking if the two users are having an existing chat room, there may be some group chats, we check that too.

    if let Err(_) = get_two_users_p2p_chat_room(_conn, requestor_user, acceptor_user) {
        // updating the chat rooms db
        // hashing the chatroom name because it doesn't have a name and it must be using the now time and the literal of the private room
        let mut hasher = DefaultHasher::new();
        Local::now()
            .offset()
            .to_string()
            .push_str("Private Room")
            .hash(&mut hasher);
        let chat_room_name_hashed = hasher.finish().to_string();
        let values_of_chat_rooms = ChatRooms {
            room_name: chat_room_name_hashed,
            room_description: "private room".to_owned(),
            chat_room_pubkey: _chat_room_pubkey.as_bytes().to_vec(),
        };

        let new_chat_room;
        match diesel::insert_into(chat_rooms)
            .values(&values_of_chat_rooms)
            .returning(QChatRooms::as_returning())
            .get_result(_conn)
        {
            Ok(res) => new_chat_room = res,
            Err(e) => {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("{:?}", e),
                )))
            }
        }
        // adding requester to the participants table
        if let Err(e) = diesel::insert_into(chat_room_participants)
            .values(ChatRoomParticipants {
                chat_room_id: new_chat_room.chat_room_id,
                user_id: requestor_user,
                is_admin: false,
            })
            .returning(ChatRoomParticipants::as_returning())
            .get_result(_conn)
        {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("{:?}", e),
            )));
        }
        // adding requester to the acceptor table
        match diesel::insert_into(chat_room_participants)
            .values(ChatRoomParticipants {
                chat_room_id: new_chat_room.chat_room_id,
                user_id: acceptor_user,
                is_admin: false,
            })
            .returning(ChatRoomParticipants::as_returning())
            .get_result(_conn)
        {
            Ok(_) => Ok(new_chat_room),
            Err(e) => {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    format!("{:?}", e),
                )))
            }
        }
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            format!(
                " user id {} already has a p2p chat with user id {}",
                requestor_user, acceptor_user
            ),
        )))
    }
}

pub fn delete_p2p_chat_room(
    _conn: &mut PgConnection,
    remover_username: &String,
    contact_username: &String,
) -> Result<bool, Box<dyn std::error::Error>> {
    let _remover_id;
    match get_user_with_username(_conn, remover_username) {
        Ok(res) => _remover_id = res.user_id,
        Err(e) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("{:?}", e),
            )))
        }
    }
    let _contact_id;
    match get_user_with_username(_conn, contact_username) {
        Ok(res) => _contact_id = res.user_id,
        Err(e) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("{:?}", e),
            )))
        }
    }

    let _chat_room_id;
    match get_two_users_p2p_chat_room(_conn, _remover_id, _contact_id) {
        Ok(res) => _chat_room_id = res.chat_room_id,
        Err(e) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("{:?}", e),
            )))
        }
    }

    // checking the authority of the remover
    let participants;
    match get_chat_room_participants_by_id(_conn, _chat_room_id) {
        Ok(res) => participants = res,
        Err(e) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("{:?}", e),
            )))
        }
    }
    // deleting the users from the participants table
    for participant in participants.iter() {
        if let Err(e) = diesel::delete(
            chat_room_participants.filter(chat_room_participants::user_id.eq(participant.user_id)),
        )
        .execute(_conn)
        {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("{:?}", e),
            )));
        }
    }

    // deleting the chat room from the chat rooms table
    match diesel::delete(chat_rooms.filter(chat_rooms::chat_room_id.eq(_chat_room_id)))
        .execute(_conn)
    {
        Ok(_) => Ok(true),
        Err(e) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("{:?}", e),
            )))
        }
    }
}

pub fn add_new_group_chat_room(
    _conn: &mut PgConnection,
    _chat_room_info: &ChatRooms,
    group_owner_username: &String,
    group_members: Vec<String>,
) -> Result<QChatRooms, Box<dyn std::error::Error>> {
    let group_owner_id: i32;
    match get_user_with_username(_conn, group_owner_username) {
        Ok(res) => group_owner_id = res.user_id,
        Err(e) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("{:?}", e),
            )))
        }
    }

    // creating the chat room
    let new_chat_room;
    match diesel::insert_into(chat_rooms)
        .values(_chat_room_info)
        .returning(QChatRooms::as_returning())
        .get_result(_conn)
    {
        Ok(res) => new_chat_room = res,
        Err(e) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("{:?}", e),
            )))
        }
    }
    // adding the owner to the participants
    let owner = ChatRoomParticipants {
        user_id: group_owner_id,
        chat_room_id: new_chat_room.chat_room_id,
        is_admin: true,
    };
    if let Err(e) = diesel::insert_into(chat_room_participants)
        .values(&owner)
        .returning(ChatRoomParticipants::as_returning())
        .get_result(_conn)
    {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("{:?}", e),
        )));
    }
    // adding members if any specified
    if group_members.len() > 0 {
        let mut group_members_up: Vec<ChatRoomParticipants> = Vec::new();
        for member in group_members {
            let _member_id: i32;
            match get_user_with_username(_conn, member.as_str()) {
                Ok(res) => _member_id = res.user_id,
                Err(e) => {
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::NotFound,
                        format!("{:?}", e),
                    )))
                }
            }
            if _member_id != group_owner_id {
                group_members_up.push(ChatRoomParticipants {
                    user_id: _member_id,
                    chat_room_id: new_chat_room.chat_room_id,
                    is_admin: false,
                });
            }
        }
        if let Err(e) = diesel::insert_into(chat_room_participants::table)
            .values(&group_members_up)
            .returning(ChatRoomParticipants::as_returning())
            .get_result(_conn)
        {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("{:?}", e),
            )));
        }
    }
    Ok(new_chat_room)
}
pub fn update_group_chat_room_info(
    _conn: &mut PgConnection,
    old_chat_room_name: &String,
    new_chat_room_info: &UpdatableChatRooms,
    editor_username: &String,
) -> Result<QChatRooms, Box<dyn std::error::Error>> {
    let _chat_room_id: i32;
    match get_group_chat_by_name(_conn, old_chat_room_name) {
        Ok(res) => _chat_room_id = res.chat_room_id,
        Err(e) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("{:?}", e),
            )))
        }
    }

    let editor_user_id: i32;
    match get_user_with_username(_conn, &editor_username) {
        Ok(res) => editor_user_id = res.user_id,
        Err(e) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("{:?}", e),
            )))
        }
    }

    match get_group_owner_by_id(_conn, _chat_room_id) {
        Ok(res) => {
            if editor_user_id != res {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::PermissionDenied,
                    format!(
                        "user id {} is not allowed to edit the group info",
                        editor_user_id
                    ),
                )));
            }
        }
        Err(e) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("{:?}", e),
            )))
        }
    }

    // updating the chat room info
    match diesel::update(chat_rooms.filter(chat_rooms::chat_room_id.eq(_chat_room_id)))
        .set((
            room_name.eq(&new_chat_room_info.room_name),
            room_description.eq(&new_chat_room_info.room_description),
        ))
        .returning(QChatRooms::as_returning())
        .get_result(_conn)
    {
        Ok(_) => Ok(get_group_chat_by_name(_conn, &new_chat_room_info.room_name).unwrap()),
        Err(e) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("{:?}", e),
            )))
        }
    }
}

pub fn delete_group_chat_room(
    _conn: &mut PgConnection,
    _chat_room_name: &String,
    remover_username: &String,
) -> Result<bool, Box<dyn std::error::Error>> {
    let remover_user_id: i32;
    match get_user_with_username(_conn, remover_username) {
        Ok(res) => remover_user_id = res.user_id,
        Err(e) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("{:?}", e),
            )))
        }
    }

    let _chat_room_id: i32;
    match get_group_chat_by_name(_conn, _chat_room_name) {
        Ok(res) => _chat_room_id = res.chat_room_id,
        Err(e) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("{:?}", e),
            )))
        }
    }

    match get_group_owner_by_id(_conn, _chat_room_id) {
        Ok(res) => {
            if remover_user_id != res {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::PermissionDenied,
                    format!(
                        "user id {} is not allowed to delete the group",
                        remover_user_id
                    ),
                )));
            }
        }
        Err(e) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("{:?}", e),
            )))
        }
    }

    // deleting the members associated to the chat room group
    if let Err(e) = diesel::delete(
        chat_room_participants.filter(chat_room_participants::chat_room_id.eq(_chat_room_id)),
    )
    .execute(_conn)
    {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("{:?}", e),
        )));
    }
    // deleting the room from the chat rooms table
    match diesel::delete(chat_rooms.filter(chat_rooms::chat_room_id.eq(_chat_room_id)))
        .execute(_conn)
    {
        Ok(_) => Ok(true),
        Err(e) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("{:?}", e),
            )))
        }
    }
}

pub fn add_participant_to_group_chat_room(
    _conn: &mut PgConnection,
    _adding_user: &ChatRoomParticipants,
    _adder_username: &String,
) -> Result<ChatRoomParticipants, Box<dyn std::error::Error>> {
    let _adder_user_id: i32;
    match get_user_with_username(_conn, _adder_username) {
        Ok(res) => _adder_user_id = res.user_id,
        Err(e) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("{:?}", e),
            )))
        }
    }

    match get_group_owner_by_id(_conn, _adding_user.chat_room_id) {
        Ok(res) => {
            if _adder_user_id != res {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::PermissionDenied,
                    format!(
                        "user id {} is not allowed to add members the group",
                        _adder_user_id
                    ),
                )));
            }
        }
        Err(e) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("{:?}", e),
            )))
        }
    }

    if !is_group_chat(_conn, _adding_user.chat_room_id) {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "couldn't find the group chat room",
        )));
    }

    // checking if the user is not already in the group
    let chat_room_info: Vec<ChatRoomParticipants> = chat_room_participants
        .filter(
            chat_room_participants::chat_room_id
                .eq(_adding_user.chat_room_id)
                .and(chat_room_participants::user_id.eq(_adding_user.user_id)),
        )
        .select(ChatRoomParticipants::as_returning())
        .load(_conn)
        .unwrap_or(vec![]);

    if chat_room_info.len() != 0 {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            format!(
                "user id {} is already in the group chat room id {}",
                _adding_user.user_id, _adding_user.chat_room_id
            ),
        )));
    }

    // inserting the user to the participants table
    match diesel::insert_into(chat_room_participants::table)
        .values(_adding_user)
        .returning(ChatRoomParticipants::as_returning())
        .get_result(_conn)
    {
        Ok(res) => Ok(res),
        Err(e) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("{:?}", e),
            )))
        }
    }
}

pub fn del_participant_from_group_chat_room(
    _conn: &mut PgConnection,
    _removing_user: &ChatRoomParticipants,
    remover_user_id: i32,
) -> Result<bool, Box<dyn std::error::Error>> {
    if !is_group_chat(_conn, _removing_user.chat_room_id) {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "couldn't find the group chat room",
        )));
    }
    let owner;
    match get_group_owner_by_id(_conn, _removing_user.chat_room_id) {
        Ok(res) => owner = res,
        Err(e) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("{}", e),
            )))
        }
    }

    if remover_user_id != owner && remover_user_id != _removing_user.user_id {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::PermissionDenied,
            format!(
                "user id {} is not allowed to remove users from group",
                remover_user_id
            ),
        )));
    }

    // checking if the user is in the group
    let chat_room_info: Vec<ChatRoomParticipants> = chat_room_participants
        .filter(
            chat_room_participants::chat_room_id
                .eq(_removing_user.chat_room_id)
                .and(chat_room_participants::user_id.eq(_removing_user.user_id)),
        )
        .select(ChatRoomParticipants::as_returning())
        .load(_conn)
        .unwrap_or(vec![]);

    if chat_room_info.len() != 1 {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!(
                "user id {} is not in the group chat room id {}",
                _removing_user.user_id, _removing_user.chat_room_id
            ),
        )));
    }

    // deleting the user from the participants table
    match diesel::delete(
        chat_room_participants.filter(chat_room_participants::user_id.eq(_removing_user.user_id)),
    )
    .execute(_conn)
    {
        Ok(_) => Ok(true),
        Err(e) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("{:?}", e),
            )))
        }
    }
}

pub fn get_chat_room_participants_by_id(
    _conn: &mut PgConnection,
    _chat_room_id: i32,
) -> Result<Vec<ChatRoomParticipants>, Box<dyn std::error::Error>> {
    // getting the participants
    let participants: Vec<ChatRoomParticipants> = chat_room_participants
        .filter(chat_room_participants::chat_room_id.eq(_chat_room_id))
        .select(ChatRoomParticipants::as_select())
        .load(_conn)
        .unwrap_or(vec![]);

    if participants.len() == 0 {
        // chat room id doesn't exists
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "chat room id not found or it has no members!",
        )))
    } else {
        Ok(participants)
    }
}

pub fn get_chat_room_participants_by_name(
    _conn: &mut PgConnection,
    _chat_room_name: &String,
) -> Result<Vec<ChatRoomParticipants>, Box<dyn std::error::Error>> {
    let _chat_room: Vec<QChatRooms> = chat_rooms
        .filter(chat_rooms::room_name.eq(_chat_room_name))
        .select(QChatRooms::as_select())
        .load(_conn)
        .unwrap_or(vec![]);

    if _chat_room.len() != 1 {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("chat room {} not found !", _chat_room_name),
        )));
    }
    // getting the participants
    let participants: Vec<ChatRoomParticipants> = chat_room_participants
        .filter(chat_room_participants::chat_room_id.eq(_chat_room[0].chat_room_id))
        .select(ChatRoomParticipants::as_select())
        .load(_conn)
        .unwrap_or(vec![]);

    if participants.len() == 0 {
        // chat room id doesn't exists
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "chat room id not found or it has no members",
        )))
    } else {
        Ok(participants)
    }
}

pub fn get_group_owner_by_id(
    _conn: &mut PgConnection,
    _chat_room_id: i32,
) -> Result<i32, Box<dyn std::error::Error>> {
    // checking if the gp is valid
    if !is_group_chat(_conn, _chat_room_id) {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "invalid chat room id !",
        )));
    }
    let _chat_room_owner: Vec<ChatRoomParticipants> = chat_room_participants
        .filter(
            chat_room_participants::chat_room_id
                .eq(_chat_room_id)
                .and(chat_room_participants::is_admin.eq(true)),
        )
        .select(ChatRoomParticipants::as_select())
        .load(_conn)
        .unwrap_or(vec![]);

    if _chat_room_owner.len() != 1 {
        // write now only one admin is supported
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "chat room owner not found !",
        )))
    } else {
        Ok(_chat_room_owner[0].user_id)
    }
}

pub fn get_group_chat_by_name(
    _conn: &mut PgConnection,
    _chat_room_name: &String,
) -> Result<QChatRooms, Box<dyn std::error::Error>> {
    let _chat_rooms: Vec<QChatRooms> = chat_rooms
        .filter(chat_rooms::room_name.eq(_chat_room_name))
        .select(QChatRooms::as_returning())
        .load(_conn)
        .unwrap_or(vec![]);

    if _chat_rooms.len() != 1 {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("{} chat room not found !", _chat_room_name),
        )))
    } else {
        return Ok(QChatRooms {
            chat_room_id: _chat_rooms[0].chat_room_id,
            room_name: _chat_rooms[0].room_name.clone(),
            room_description: _chat_rooms[0].room_description.clone(),
            chat_room_pubkey: _chat_rooms[0].chat_room_pubkey.to_owned(),
        });
    }
}

pub fn get_group_chat_by_id(
    _conn: &mut PgConnection,
    _chat_room_id: i32,
) -> Result<QChatRooms, Box<dyn std::error::Error>> {
    let _chat_rooms: Vec<QChatRooms> = chat_rooms
        .filter(chat_rooms::chat_room_id.eq(_chat_room_id))
        .select(QChatRooms::as_returning())
        .load(_conn)
        .unwrap_or(vec![]);

    if _chat_rooms.len() != 1 {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("chat room id {} not found !", _chat_room_id),
        )))
    } else {
        return Ok(QChatRooms {
            chat_room_id: _chat_rooms[0].chat_room_id,
            room_name: _chat_rooms[0].room_name.clone(),
            room_description: _chat_rooms[0].room_description.clone(),
            chat_room_pubkey: _chat_rooms[0].chat_room_pubkey.to_owned(),
        });
    }
}
pub fn is_group_chat(_conn: &mut PgConnection, _chat_room_id: i32) -> bool {
    let chat_room_info: Vec<ChatRooms> = chat_rooms
        .filter(chat_rooms::chat_room_id.eq(_chat_room_id))
        .select(ChatRooms::as_returning())
        .load(_conn)
        .unwrap_or(vec![]);

    if chat_room_info.len() != 1 {
        false
    } else if chat_room_info[0].room_description == "private room" {
        false
    } else {
        true
    }
}

pub fn is_valid_chatroom(_conn: &mut PgConnection, _chat_room_id: i32) -> bool {
    let chat_room_info: Vec<ChatRooms> = chat_rooms
        .filter(chat_rooms::chat_room_id.eq(_chat_room_id))
        .select(ChatRooms::as_returning())
        .load(_conn)
        .unwrap_or(vec![]);

    if chat_room_info.len() != 1 {
        false
    } else {
        true
    }
}

pub fn is_valid_user(_conn: &mut PgConnection, _user_id: i32) -> bool {
    let chat_room_info: Vec<Users> = users
        .filter(users::user_id.eq(_user_id))
        .select(Users::as_returning())
        .load(_conn)
        .unwrap_or(vec![]);

    if chat_room_info.len() != 1 {
        false
    } else {
        true
    }
}

pub fn is_user_in_chat_room(_conn: &mut PgConnection, _chat_room_id: i32, _user_id: i32) -> bool {
    let chat_room_info: Vec<ChatRoomParticipants> = chat_room_participants
        .filter(
            chat_room_participants::chat_room_id
                .eq(_chat_room_id)
                .and(chat_room_participants::user_id.eq(_user_id)),
        )
        .select(ChatRoomParticipants::as_returning())
        .load(_conn)
        .unwrap_or(vec![]);

    if chat_room_info.len() != 1 {
        false
    } else {
        true
    }
}

pub fn get_user_p2p_chat_rooms_by_user_id(
    _conn: &mut PgConnection,
    _user_id: i32,
) -> Result<Vec<QChatRooms>, Box<dyn std::error::Error>> {
    let mut _chat_rooms: Vec<QChatRooms> = chat_rooms
        .inner_join(
            chat_room_participants
                .on(chat_room_participants::chat_room_id.eq(chat_rooms::chat_room_id)),
        )
        .filter(chat_room_participants::user_id.eq(chat_room_participants::user_id))
        .filter(chat_rooms::room_description.eq("private room")) // Add this filter for room name
        .select(QChatRooms::as_select())
        .load(_conn)
        .unwrap_or(vec![]);

    if _chat_rooms.len() == 0 {
        // chat room id doesn't exists
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("no p2p chat rooms for user id {} ", _user_id),
        )))
    } else {
        Ok(_chat_rooms)
    }
}

pub fn get_user_group_chat_rooms_by_user_id(
    _conn: &mut PgConnection,
    _user_id: i32,
) -> Result<Vec<QChatRooms>, Box<dyn std::error::Error>> {
    let mut _chat_rooms: Vec<QChatRooms> = chat_rooms
        .inner_join(
            chat_room_participants
                .on(chat_room_participants::chat_room_id.eq(chat_rooms::chat_room_id)),
        )
        .filter(chat_room_participants::user_id.eq(_user_id))
        .filter(chat_rooms::room_description.ne("private room")) // Add this filter for room name
        .select(QChatRooms::as_select())
        .load(_conn)
        .unwrap_or(vec![]); // this function never fails, if there is none we will get a empty vector

    if _chat_rooms.len() == 0 {
        // chat room id doesn't exists
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("no group chat rooms for user id {} ", _user_id),
        )))
    } else {
        Ok(_chat_rooms)
    }
}

pub fn get_two_users_p2p_chat_room(
    _conn: &mut PgConnection,
    _user_id_1: i32,
    _user_id_2: i32,
) -> Result<QChatRooms, Box<dyn std::error::Error>> {
    if !get_user_with_user_id(_conn, _user_id_1).is_ok() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("user id  {} doesn't exist's", _user_id_1),
        )));
    }
    if !get_user_with_user_id(_conn, _user_id_2).is_ok() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("user id  {} doesn't exist's", _user_id_2),
        )));
    }

    let _chat_room_participants: Vec<ChatRoomParticipants>;
    {
        _chat_room_participants = chat_room_participants
            .filter(
                chat_room_participants::user_id
                    .eq(_user_id_1)
                    .or(chat_room_participants::user_id.eq(_user_id_2)),
            )
            .group_by(chat_room_participants::chat_room_id)
            .having(diesel::dsl::count(chat_room_participants::user_id).eq(2))
            .select(ChatRoomParticipants::as_select())
            .load(_conn)
            .unwrap_or(vec![]);
    }
    if _chat_room_participants.len() != 1 {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!(
                "user id  {} and user id {} are not charing any p2p chat rooms !",
                _user_id_1, _user_id_2
            ),
        )));
    }
    match get_group_chat_by_id(_conn, _chat_room_participants[0].chat_room_id) {
        Ok(res) => Ok(res),
        Err(e) => Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("{}", e),
        ))),
    }
}
