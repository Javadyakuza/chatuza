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
) -> Result<QUsers, Error> {
    // inserting user credits
    diesel::insert_into(users::table)
        .values(user_credits)
        .returning(Users::as_returning())
        .get_result(conn)
        .expect("couldn't insert user credits");

    // fetching the user info
    let user_info: QUsers =
        get_user_with_username(conn, &user_credits.username).expect("didn't find the user id !");

    user_profile.user_id = user_info.user_id;
    let u_p: &UserProfiles = user_profile;

    // inserting user profiles
    diesel::insert_into(user_profiles::table)
        .values(u_p)
        .returning(UserProfiles::as_returning())
        .get_result(conn)
        .expect("couldn't insert user credits");

    println!("inserted user id {}", &user_info.user_id);

    Ok(user_info)
}

pub fn update_user_credits(
    conn: &mut PgConnection,
    old_username: &String,
    new_user_credits: &Users,
) -> Result<QUsers, Box<dyn std::error::Error>> {
    let user_info: QUsers =
        get_user_with_username(conn, old_username.as_str()).expect("didn't find the user name !");

    // checking for the duplicated username

    if let Some(_) = get_user_with_username(conn, &new_user_credits.username) {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            "username already exists !",
        )));
    }

    if let Some(_) = get_user_with_email(conn, &new_user_credits.email) {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            "email already exists !",
        )));
    }

    diesel::update(users.filter(users::user_id.eq(user_info.user_id)))
        .set((
            username.eq(&new_user_credits.username),
            email.eq(&new_user_credits.email),
            password.eq(&new_user_credits.password),
        ))
        .returning(Users::as_returning())
        .get_result(conn)
        .expect("couldn't update user credits");

    Ok(get_user_with_username(conn, new_user_credits.username.as_str()).unwrap())
}

pub fn update_user_profile(
    conn: &mut PgConnection,
    old_username: &String,
    user_profile: &mut UserProfiles,
) -> Result<UserProfiles, Box<dyn std::error::Error>> {
    //fetching the user id and set it accordingly
    user_profile.user_id = get_user_with_username(conn, &old_username).unwrap().user_id;

    diesel::update(user_profiles.filter(user_profiles::user_id.eq(user_profile.user_id)))
        .set((
            bio.eq(&user_profile.bio),
            profile_picture.eq(&user_profile.profile_picture),
        ))
        .returning(UserProfiles::as_returning())
        .get_result(conn)
        .expect("couldn't update user profile");

    Ok(get_user_profile_with_user_id(conn, user_profile.user_id).unwrap())
}

pub fn delete_user(conn: &mut PgConnection, _username: &String) -> Result<bool, std::io::Error> {
    let _user_id = get_user_with_username(conn, _username.as_str())
        .unwrap()
        .user_id;

    // deleting the users p2p chatroom's
    if let Some(_chat_rooms) = get_user_p2p_chat_rooms_by_user_id(conn, _user_id) {
        for chat_room in _chat_rooms {
            let p2p_chat_room_participants =
                get_chat_room_participants_by_id(conn, chat_room.chat_room_id).unwrap();
            let remover_username =
                get_user_with_user_id(conn, p2p_chat_room_participants[0].user_id)
                    .unwrap()
                    .username;
            let contact_username =
                get_user_with_user_id(conn, p2p_chat_room_participants[0].user_id)
                    .unwrap()
                    .username;
            let _ = delete_p2p_chat_room(conn, &remover_username, &contact_username);
        }
    }
    if let Some(gps) = get_user_group_chat_rooms_by_user_id(conn, _user_id) {
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

    diesel::delete(user_profiles.filter(user_profiles::user_id.eq(_user_id)))
        .execute(conn)
        .expect("couldn't delete user profile");

    // checking if the user has any wallets created and deleting them
    let _tron_wallets = tron_wallets
        .filter(tron_wallets::user_id.eq(_user_id))
        .select(QTronWallet::as_select())
        .load(conn)
        .unwrap();

    let _solana_wallets = solana_wallets
        .filter(solana_wallets::user_id.eq(_user_id))
        .select(QSolanaWallet::as_select())
        .load(conn)
        .unwrap();

    if _tron_wallets.len() as u32 == 1 {
        delete_tron_wallet(conn, _username).unwrap();
    }

    if _solana_wallets.len() as u32 == 1 {
        delete_solana_wallet(conn, _username).unwrap();
    }
    diesel::delete(users.filter(users::user_id.eq(_user_id)))
        .execute(conn)
        .expect("couldn't delete user credits");
    Ok(true)
}

// -- Users / UserProfiles GETTER functions -- //

pub fn get_user_with_username(_conn: &mut PgConnection, _username: &str) -> Option<QUsers> {
    let user_row: Vec<QUsers> = users
        .filter(username.eq(&_username))
        .select(QUsers::as_select())
        .load(_conn)
        .unwrap_or(vec![]);
    if user_row.len() == 1 {
        Some(QUsers {
            user_id: user_row[0].user_id,
            username: user_row[0].username.clone(),
            email: user_row[0].email.clone(),
            password: user_row[0].password.clone(),
        })
    } else {
        // nothing was found
        None
    }
}

pub fn get_user_with_email(_conn: &mut PgConnection, _email: &str) -> Option<QUsers> {
    let user_row: Vec<QUsers> = users
        .filter(email.eq(&_email))
        .select(QUsers::as_select())
        .load(_conn)
        .unwrap_or(vec![]);
    if user_row.len() == 1 {
        Some(QUsers {
            user_id: user_row[0].user_id,
            username: user_row[0].username.clone(),
            email: user_row[0].email.clone(),
            password: user_row[0].password.clone(),
        })
    } else {
        // some thing is wrong
        None
    }
}

pub fn get_user_with_user_id(_conn: &mut PgConnection, _user_id: i32) -> Option<QUsers> {
    let user_row: Vec<QUsers> = users
        .filter(users::user_id.eq(&_user_id))
        .select(QUsers::as_select())
        .load(_conn)
        .unwrap_or(vec![]);
    if user_row.len() == 1 {
        Some(QUsers {
            user_id: user_row[0].user_id,
            username: user_row[0].username.clone(),
            email: user_row[0].email.clone(),
            password: user_row[0].password.clone(),
        })
    } else {
        // some thing is wrong
        None
    }
}

pub fn get_user_profile_with_user_id(
    _conn: &mut PgConnection,
    _user_id: i32,
) -> Option<UserProfiles> {
    let user_row: Vec<UserProfiles> = user_profiles
        .filter(user_profiles::user_id.eq(&_user_id))
        .select(UserProfiles::as_select())
        .load(_conn)
        .unwrap_or(vec![]);

    if user_row.len() == 1 {
        Some(UserProfiles {
            user_id: _user_id,
            bio: user_row[0].bio.clone(),
            profile_picture: user_row[0].profile_picture.clone(),
        })
    } else {
        // some thing is wrong
        None
    }
}

pub fn get_user_profile_with_username(
    _conn: &mut PgConnection,
    _username: String,
) -> Option<UserProfiles> {
    let _user_id = get_user_with_username(_conn, _username.as_str())
        .unwrap()
        .user_id;
    let user_row: Vec<UserProfiles> = user_profiles
        .filter(user_profiles::user_id.eq(&_user_id))
        .select(UserProfiles::as_select())
        .load(_conn)
        .unwrap_or(vec![]);

    if user_row.len() == 1 {
        Some(UserProfiles {
            user_id: _user_id,
            bio: user_row[0].bio.clone(),
            profile_picture: user_row[0].profile_picture.clone(),
        })
    } else {
        // some thing is wrong
        None
    }
}

// -- ChatRooms/ Message history SETTER functions -- //

pub fn add_new_p2p_chat_room(
    _conn: &mut PgConnection,
    requestor_user: i32,
    acceptor_user: i32,
    _chat_room_pubkey: String,
) -> Result<QChatRooms, Box<dyn std::error::Error>> {
    // fetching the
    // checking if the two users are having an existing chat room, there may be some grout chats, we check that too.
    let shared_room_res: Vec<_> = chat_room_participants
        .filter(
            chat_room_participants::user_id
                .eq(requestor_user)
                .or(chat_room_participants::user_id.eq(acceptor_user)),
        )
        .group_by(chat_room_participants::chat_room_id)
        .having(diesel::dsl::count(chat_room_participants::user_id).le(2))
        .select(diesel::dsl::min(chat_room_participants::chat_room_id))
        .load::<Option<i32>>(_conn)
        .unwrap();

    if shared_room_res.len() == 0 {
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

        let new_chat_room = diesel::insert_into(chat_rooms)
            .values(&values_of_chat_rooms)
            .returning(QChatRooms::as_returning())
            .get_result(_conn)
            .expect("couldn't insert user credits");

        // adding requester to the participants table
        diesel::insert_into(chat_room_participants)
            .values(ChatRoomParticipants {
                chat_room_id: new_chat_room.chat_room_id,
                user_id: requestor_user,
                is_admin: false,
            })
            .returning(ChatRoomParticipants::as_returning())
            .get_result(_conn)
            .expect("couldn't insert user credits");

        // adding requester to the acceptor table
        diesel::insert_into(chat_room_participants)
            .values(ChatRoomParticipants {
                chat_room_id: new_chat_room.chat_room_id,
                user_id: acceptor_user,
                is_admin: false,
            })
            .returning(ChatRoomParticipants::as_returning())
            .get_result(_conn)
            .expect("couldn't insert user credits");
        println!(
            "new private chat room id {} \n user one id  {} \n user two id {} ",
            new_chat_room.chat_room_id, requestor_user, acceptor_user
        );
        Ok(new_chat_room)
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
    let _remover_id = get_user_with_username(_conn, remover_username)
        .unwrap()
        .user_id;
    let _contact_id = get_user_with_username(_conn, contact_username)
        .unwrap()
        .user_id;

    let _chat_room_id = get_two_users_p2p_chat_room(_conn, _remover_id, _contact_id)
        .unwrap()
        .chat_room_id;

    // checking the authority of the remover
    let participants = get_chat_room_participants_by_id(_conn, _chat_room_id)
        .expect(format!("no chat rooms with id {} found !", _chat_room_id).as_str());

    // deleting the users from the participants table
    for participant in participants.iter() {
        diesel::delete(
            chat_room_participants.filter(chat_room_participants::user_id.eq(participant.user_id)),
        )
        .execute(_conn)
        .expect(
            format!(
                "couldn't delete the user id {} from participants table",
                participant.user_id
            )
            .as_str(),
        );
    }

    // deleting the chat room from the chat rooms table
    diesel::delete(chat_rooms.filter(chat_rooms::chat_room_id.eq(_chat_room_id)))
        .execute(_conn)
        .expect(
            format!(
                "couldn't delete the user id {} from participants table",
                _chat_room_id
            )
            .as_str(),
        );
    Ok(true)
}

pub fn add_new_group_chat_room(
    _conn: &mut PgConnection,
    _chat_room_info: &ChatRooms,
    group_owner_username: &String,
    group_members: Vec<String>,
) -> Result<QChatRooms, Error> {
    let group_owner_id = get_user_with_username(_conn, group_owner_username)
        .expect("group owner provided username didn't found")
        .user_id;

    // creating the chat room
    let new_chat_room = diesel::insert_into(chat_rooms)
        .values(_chat_room_info)
        .returning(QChatRooms::as_returning())
        .get_result(_conn)
        .unwrap();

    println!(
        "new group chat room add, \n  id : {}",
        new_chat_room.chat_room_id
    );

    // adding the owner to the participants
    let owner = ChatRoomParticipants {
        user_id: group_owner_id,
        chat_room_id: new_chat_room.chat_room_id,
        is_admin: true,
    };
    let _ = diesel::insert_into(chat_room_participants)
        .values(&owner)
        .returning(ChatRoomParticipants::as_returning())
        .get_result(_conn)
        .unwrap();

    println!(
        "added the user id {} to the group chat room id {} as the owner",
        group_owner_id, new_chat_room.chat_room_id
    );
    // adding members if any specified
    if group_members.len() > 0 {
        let mut group_members_up: Vec<ChatRoomParticipants> = Vec::new();
        for member in group_members {
            if get_user_with_username(_conn, member.as_str())
                .unwrap()
                .user_id
                != group_owner_id
            {
                group_members_up.push(ChatRoomParticipants {
                    user_id: get_user_with_username(_conn, member.as_str())
                        .unwrap()
                        .user_id,
                    chat_room_id: new_chat_room.chat_room_id,
                    is_admin: false,
                });
            }
        }
        let _ = diesel::insert_into(chat_room_participants::table)
            .values(&group_members_up)
            .returning(ChatRoomParticipants::as_returning())
            .get_result(_conn)
            .unwrap();
    }
    Ok(new_chat_room)
}
pub fn update_group_chat_room_info(
    _conn: &mut PgConnection,
    old_chat_room_name: &String,
    new_chat_room_info: &UpdatableChatRooms,
    editor_username: &String,
) -> Result<QChatRooms, String> {
    // getting the chat room id
    let _chat_room_id = get_group_chat_by_name(_conn, old_chat_room_name)
        .expect("couldn't find the group chat room")
        .chat_room_id;

    let editor_user_id = get_user_with_username(_conn, &editor_username)
        .unwrap()
        .user_id;
    // checking if the editor is the admin
    if editor_user_id != get_group_owner_by_id(_conn, _chat_room_id).unwrap() {
        return Err(format!(
            "user id {} is not allowed to edit the group info",
            editor_user_id
        ));
    }

    // updating the chat room info
    diesel::update(chat_rooms.filter(chat_rooms::chat_room_id.eq(_chat_room_id)))
        .set((
            room_name.eq(&new_chat_room_info.room_name),
            room_description.eq(&new_chat_room_info.room_description),
        ))
        .returning(QChatRooms::as_returning())
        .get_result(_conn)
        .expect("couldn't update the chat room info");

    println!("updated the chat room id {} info", _chat_room_id);

    Ok(get_group_chat_by_name(_conn, &new_chat_room_info.room_name).unwrap())
}

pub fn delete_group_chat_room(
    _conn: &mut PgConnection,
    _chat_room_name: &String,
    remover_username: &String,
) -> Result<bool, Box<dyn std::error::Error>> {
    let remover_user_id = get_user_with_username(_conn, remover_username)
        .unwrap()
        .user_id; // getting the chat room id
    let _chat_room_id = get_group_chat_by_name(_conn, _chat_room_name)
        .expect("couldn't find the group chat room")
        .chat_room_id;

    // checking if the remover is the admin
    // this will also check if the chat room id is a group chat room or not
    if remover_user_id != get_group_owner_by_id(_conn, _chat_room_id).unwrap() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            format!(
                "user id {} is not allowed to remove the group info",
                remover_user_id
            ),
        )));
    }
    // deleting the members associated to the chat room group
    diesel::delete(
        chat_room_participants.filter(chat_room_participants::chat_room_id.eq(_chat_room_id)),
    )
    .execute(_conn)
    .unwrap();

    // deleting the room from the chat rooms table
    diesel::delete(chat_rooms.filter(chat_rooms::chat_room_id.eq(_chat_room_id)))
        .execute(_conn)
        .unwrap();

    Ok(true)
}

pub fn add_participant_to_group_chat_room(
    _conn: &mut PgConnection,
    _adding_user: &ChatRoomParticipants,
    _adder_username: &String,
) -> Result<ChatRoomParticipants, Box<dyn std::error::Error>> {
    let _adder_user_id = get_user_with_username(_conn, _adder_username)
        .unwrap()
        .user_id;
    if get_group_owner_by_id(_conn, _adding_user.chat_room_id).unwrap() != _adder_user_id {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            "only admin can add participants to the group chat at the moment",
        )));
    }

    if !is_group_chat(_conn, _adding_user.chat_room_id) {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
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
        .unwrap();

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
    let new_participant: ChatRoomParticipants = diesel::insert_into(chat_room_participants::table)
        .values(_adding_user)
        .returning(ChatRoomParticipants::as_returning())
        .get_result(_conn)
        .expect("couldn't insert user to the group");

    Ok(new_participant)
}

pub fn del_participant_from_group_chat_room(
    _conn: &mut PgConnection,
    _removing_user: &ChatRoomParticipants,
    remover_user_id: i32,
) -> Result<bool, Box<dyn std::error::Error>> {
    if !is_group_chat(_conn, _removing_user.chat_room_id) {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            "couldn't find the group chat room",
        )));
    }

    // checking if the remover is the admin
    // this will also check if the chat room id is a group chat room or not
    if remover_user_id != get_group_owner_by_id(_conn, _removing_user.chat_room_id).unwrap()
        && remover_user_id != _removing_user.user_id
    {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
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
        .unwrap();

    if chat_room_info.len() != 1 {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            format!(
                "user id {} is not in the group chat room id {}",
                _removing_user.user_id, _removing_user.chat_room_id
            ),
        )));
    }

    // deleting the user from the participants table
    diesel::delete(
        chat_room_participants.filter(chat_room_participants::user_id.eq(_removing_user.user_id)),
    )
    .execute(_conn)
    .unwrap();

    Ok(true)
}

pub fn get_chat_room_participants_by_id(
    _conn: &mut PgConnection,
    _chat_room_id: i32,
) -> Option<Vec<ChatRoomParticipants>> {
    // getting the participants
    let participants: Vec<ChatRoomParticipants> = chat_room_participants
        .filter(chat_room_participants::chat_room_id.eq(_chat_room_id))
        .select(ChatRoomParticipants::as_select())
        .load(_conn)
        .unwrap();

    if participants.len() == 0 {
        // chat room id doesn't exists
        None
    } else {
        Some(participants)
    }
}

pub fn get_chat_room_participants_by_name(
    _conn: &mut PgConnection,
    _chat_room_name: &String,
) -> Option<Vec<ChatRoomParticipants>> {
    let _chat_room: Vec<QChatRooms> = chat_rooms
        .filter(chat_rooms::room_name.eq(_chat_room_name))
        .select(QChatRooms::as_select())
        .load(_conn)
        .unwrap();

    if _chat_room.len() != 1 {
        return None;
    }
    // getting the participants
    let participants: Vec<ChatRoomParticipants> = chat_room_participants
        .filter(chat_room_participants::chat_room_id.eq(_chat_room[0].chat_room_id))
        .select(ChatRoomParticipants::as_select())
        .load(_conn)
        .unwrap();

    if participants.len() == 0 {
        // chat room id doesn't exists
        None
    } else {
        Some(participants)
    }
}

pub fn get_group_owner_by_id(_conn: &mut PgConnection, _chat_room_id: i32) -> Option<i32> {
    let _chat_room_owner = chat_room_participants
        .filter(
            chat_room_participants::chat_room_id
                .eq(_chat_room_id)
                .and(chat_room_participants::is_admin.eq(true)),
        )
        .select(diesel::dsl::min(chat_room_participants::user_id))
        .load::<Option<i32>>(_conn)
        .unwrap();

    println!("{:?}", _chat_room_owner);

    if _chat_room_owner.len() != 1 {
        // write now only one admin is supported
        return None;
    } else {
        _chat_room_owner[0]
    }
}

pub fn get_group_chat_by_name(
    _conn: &mut PgConnection,
    _chat_room_name: &String,
) -> Option<QChatRooms> {
    let _chat_rooms: Vec<QChatRooms> = chat_rooms
        .filter(chat_rooms::room_name.eq(_chat_room_name))
        .select(QChatRooms::as_returning())
        .load(_conn)
        .unwrap();

    if _chat_rooms.len() != 1 {
        return None;
    } else {
        return Some(QChatRooms {
            chat_room_id: _chat_rooms[0].chat_room_id,
            room_name: _chat_rooms[0].room_name.clone(),
            room_description: _chat_rooms[0].room_description.clone(),
            chat_room_pubkey: _chat_rooms[0].chat_room_pubkey.to_owned(),
        });
    }
}

pub fn get_group_chat_by_id(_conn: &mut PgConnection, _chat_room_id: i32) -> Option<QChatRooms> {
    let _chat_rooms: Vec<QChatRooms> = chat_rooms
        .filter(chat_rooms::chat_room_id.eq(_chat_room_id))
        .select(QChatRooms::as_returning())
        .load(_conn)
        .unwrap();

    if _chat_rooms.len() != 1 {
        return None;
    } else {
        return Some(QChatRooms {
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
        .unwrap();

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
        .unwrap();

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
        .unwrap();

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
        .unwrap();

    if chat_room_info.len() != 1 {
        false
    } else {
        true
    }
}

pub fn get_user_p2p_chat_rooms_by_user_id(
    _conn: &mut PgConnection,
    _user_id: i32,
) -> Option<Vec<QChatRooms>> {
    let mut _chat_rooms: Vec<QChatRooms> = chat_rooms
        .inner_join(
            chat_room_participants
                .on(chat_room_participants::chat_room_id.eq(chat_rooms::chat_room_id)),
        )
        .filter(chat_room_participants::user_id.eq(chat_room_participants::user_id))
        .filter(chat_rooms::room_description.eq("private room")) // Add this filter for room name
        .select(QChatRooms::as_select())
        .load(_conn)
        .unwrap();

    if _chat_rooms.len() == 0 {
        // chat room id doesn't exists
        None
    } else {
        Some(_chat_rooms)
    }
}

pub fn get_user_group_chat_rooms_by_user_id(
    _conn: &mut PgConnection,
    _user_id: i32,
) -> Option<Vec<QChatRooms>> {
    let mut _chat_rooms: Vec<QChatRooms> = chat_rooms
        .inner_join(
            chat_room_participants
                .on(chat_room_participants::chat_room_id.eq(chat_rooms::chat_room_id)),
        )
        .filter(chat_room_participants::user_id.eq(_user_id))
        .filter(chat_rooms::room_description.ne("private room")) // Add this filter for room name
        .select(QChatRooms::as_select())
        .load(_conn)
        .unwrap();

    if _chat_rooms.len() == 0 {
        // chat room id doesn't exists
        None
    } else {
        Some(_chat_rooms)
    }
}

pub fn get_two_users_p2p_chat_room(
    _conn: &mut PgConnection,
    _user_id_1: i32,
    _user_id_2: i32,
) -> Option<QChatRooms> {
    assert!(get_user_with_user_id(_conn, _user_id_1).is_some());
    assert!(get_user_with_user_id(_conn, _user_id_2).is_some());
    let _chat_room_id;
    {
        _chat_room_id = chat_room_participants
            .filter(
                chat_room_participants::user_id
                    .eq(_user_id_1)
                    .or(chat_room_participants::user_id.eq(_user_id_2)),
            )
            .group_by(chat_room_participants::chat_room_id)
            .having(diesel::dsl::count(chat_room_participants::user_id).le(2))
            .select(diesel::dsl::min(chat_room_participants::chat_room_id))
            .load::<Option<i32>>(_conn)
            .unwrap()[0]
            .unwrap();
    }
    Some(get_group_chat_by_id(_conn, _chat_room_id).unwrap())
}
