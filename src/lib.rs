#![recursion_limit = "256"]
pub mod models;
pub mod schema;

use crate::models::{ChatRoomParticipants, ChatRooms, QMessages, QUsers, UserProfiles, Users};
use crate::schema::{chat_room_participants, chat_rooms, messages, user_profiles, users};
pub use diesel;
pub use diesel::pg::PgConnection;
pub use diesel::prelude::*;
pub use diesel::result::Error;
pub use dotenvy::dotenv;
use models::{Messages, QChatRoomParticipants, QChatRooms};
use schema::{
    chat_room_participants::dsl::*, chat_rooms::dsl::*, user_profiles::dsl::*, users::dsl::*,
};
pub use std::env;

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
) -> Result<(), Error> {
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

    Ok(())
}

pub fn update_user_credits(
    conn: &mut PgConnection,
    user_credits: &QUsers,
) -> Result<String, String> {
    let user_info: QUsers =
        get_user_with_user_id(conn, user_credits.user_id).expect("didn't find the user id !");

    // checking for the password to not be the same as the previous one
    assert!(user_credits.password != user_info.password);

    // checking for the duplicated username
    if let Some(_) = get_user_with_username(conn, &user_credits.username) {
        return Err("username already exists !".to_owned());
    }

    // checking for the duplicated email
    if let Some(_) = get_user_with_username(conn, &user_credits.email) {
        return Err("email already exists !".to_owned());
    }

    diesel::update(users.filter(users::user_id.eq(user_info.user_id)))
        .set((
            username.eq(&user_credits.username),
            email.eq(&user_credits.email),
            password.eq(&user_credits.password),
        ))
        .returning(Users::as_returning())
        .get_result(conn)
        .expect("couldn't update user credits");

    Ok(format!("user id {} updated", &user_info.user_id))
}

pub fn update_user_profile(
    conn: &mut PgConnection,
    user_profile: &UserProfiles,
) -> Result<String, String> {
    diesel::update(user_profiles.filter(user_profiles::user_id.eq(user_profile.user_id)))
        .set((
            bio.eq(&user_profile.bio),
            profile_picture.eq(&user_profile.profile_picture),
        ))
        .returning(UserProfiles::as_returning())
        .get_result(conn)
        .expect("couldn't update user profile");

    Ok(format!("user id {} updated", &user_profile.user_id))
}

pub fn delete_user(conn: &mut PgConnection, _user_id: i32) -> Result<String, String> {
    diesel::delete(user_profiles.filter(user_profiles::user_id.eq(_user_id)))
        .execute(conn)
        .expect("couldn't update user profile");

    diesel::delete(users.filter(users::user_id.eq(_user_id)))
        .execute(conn)
        .expect("couldn't update user profile");

    Ok(format!("user id {} updated", &_user_id))
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

// -- ChatRooms/ Message history SETTER functions -- //

pub fn add_new_p2p_chat_room(
    _conn: &mut PgConnection,
    requestor_user: &mut ChatRoomParticipants,
    acceptor_user: &mut ChatRoomParticipants,
) -> Result<(), String> {
    // fetching the
    // checking if the two users are having an existing chat room, there may be some grout chats, we check that too.
    let shared_room_res: Vec<_> = chat_room_participants
        .filter(
            chat_room_participants::user_id
                .eq(requestor_user.user_id)
                .or(chat_room_participants::user_id.eq(acceptor_user.user_id)),
        )
        .group_by(chat_room_participants::chat_room_id)
        .having(diesel::dsl::count(chat_room_participants::user_id).le(2))
        .select(diesel::dsl::min(chat_room_participants::chat_room_id))
        .load::<Option<i32>>(_conn)
        .unwrap();

    if shared_room_res.len() == 0 {
        // updating the chat rooms db
        let values_of_chat_rooms = ChatRooms {
            room_name: "private room".to_owned(),
            room_description: "private room".to_owned(),
        };

        let new_chat_room = diesel::insert_into(chat_rooms)
            .values(&values_of_chat_rooms)
            .returning(QChatRooms::as_returning())
            .get_result(_conn)
            .expect("couldn't insert user credits");

        requestor_user.chat_room_id = new_chat_room.chat_room_id;
        acceptor_user.chat_room_id = new_chat_room.chat_room_id;

        // adding requester to the participants table
        diesel::insert_into(chat_room_participants)
            .values(&*requestor_user)
            .returning(ChatRoomParticipants::as_returning())
            .get_result(_conn)
            .expect("couldn't insert user credits");

        // adding requester to the acceptor table
        diesel::insert_into(chat_room_participants)
            .values(&*acceptor_user)
            .returning(ChatRoomParticipants::as_returning())
            .get_result(_conn)
            .expect("couldn't insert user credits");
        println!(
            "new private chat room id {} \n user one id  {} \n user two id {} ",
            new_chat_room.chat_room_id, requestor_user.user_id, acceptor_user.user_id
        );
        Ok(())
    } else {
        Err(format!(
            " user id {} already has a p2p chat with user id {}",
            requestor_user.user_id, acceptor_user.user_id
        ))
    }
}

pub fn delete_private_chat_room(
    _conn: &mut PgConnection,
    _chat_room_id: i32,
    _remover_id: i32,
) -> Result<(), String> {
    // checking the authority of the remover

    let participants = get_chat_room_participants_by_id(_conn, _chat_room_id)
        .expect(format!("no chat rooms with id {} found !", _chat_room_id).as_str());
    if participants.len() != 2 {
        return Err("didn't find a p2p chat with that chat room id".to_owned());
    } else if participants[0].user_id != _remover_id && participants[1].user_id != _remover_id {
        return Err("remover id doesn't have the authority to delete the chat room".to_owned());
    }

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
    Ok(())
}

pub fn add_new_group_chat_room(
    _conn: &mut PgConnection,
    _chat_room_info: &ChatRooms,
    group_owner_id: i32,
    group_members: Vec<&mut ChatRoomParticipants>,
) -> Result<i32, String> {
    // checking the owner existence
    if let None = get_user_with_user_id(_conn, group_owner_id) {
        return Err(format!("user id {} doesnt exits", group_owner_id));
    }

    // creating the chat room
    let new_chat_room_id = diesel::insert_into(chat_rooms)
        .values(_chat_room_info)
        .returning(QChatRooms::as_returning())
        .get_result(_conn)
        .unwrap()
        .chat_room_id;

    println!("new group chat room add, \n  id : {}", new_chat_room_id);

    // adding the owner to the participants
    let owner = ChatRoomParticipants {
        user_id: group_owner_id,
        chat_room_id: new_chat_room_id,
        is_admin: true,
    };
    let _ = diesel::insert_into(chat_room_participants)
        .values(&owner)
        .returning(ChatRoomParticipants::as_returning())
        .get_result(_conn)
        .unwrap();

    println!(
        "added the user id {} to the group chat room id {} as the owner",
        group_owner_id, new_chat_room_id
    );

    // adding members if any specified
    if group_members.len() > 0 {
        let group_members: Vec<ChatRoomParticipants> = group_members
            .iter()
            .filter(|member| member.user_id != group_owner_id)
            .map(|member| ChatRoomParticipants {
                user_id: member.user_id,
                chat_room_id: new_chat_room_id,
                is_admin: false,
            })
            .collect();
        let _ = diesel::insert_into(chat_room_participants::table)
            .values(&group_members)
            .returning(ChatRoomParticipants::as_returning())
            .get_result(_conn)
            .unwrap();
    }

    Ok(new_chat_room_id)
}

pub fn update_group_chat_room_info(
    _conn: &mut PgConnection,
    old_chat_room_name: &String,
    new_chat_room_info: &ChatRooms,
    editor_user_id: i32,
) -> Result<(), String> {
    // getting the chat room id
    let _chat_room_id = get_group_chat_id_by_name(_conn, old_chat_room_name)
        .expect("couldn't find the group chat room");

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

    Ok(())
}

pub fn delete_group_chat_room(
    _conn: &mut PgConnection,
    _chat_room_name: &String,
    remover_user_id: i32,
) -> Result<(), String> {
    // getting the chat room id
    let _chat_room_id = get_group_chat_id_by_name(_conn, _chat_room_name)
        .expect("couldn't find the group chat room");

    // checking if the remover is the admin
    // this will also check if the chat room id is a group chat room or not
    if remover_user_id != get_group_owner_by_id(_conn, _chat_room_id).unwrap() {
        return Err(format!(
            "user id {} is not allowed to remove the group info",
            remover_user_id
        ));
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

    Ok(())
}

pub fn add_participant_to_group_chat_room(
    _conn: &mut PgConnection,
    _adding_user: &ChatRoomParticipants,
) -> Result<(), String> {
    if !is_group_chat(_conn, _adding_user.chat_room_id) {
        return Err(format!("couldn't find the group chat room"));
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
        return Err(format!(
            "user id {} is already in the group chat room id {}",
            _adding_user.user_id, _adding_user.chat_room_id
        ));
    }

    // inserting the user to the participants table
    diesel::insert_into(chat_room_participants::table)
        .values(_adding_user)
        .returning(ChatRoomParticipants::as_returning())
        .get_result(_conn)
        .expect("couldn't insert user to the group");

    Ok(())
}

pub fn del_participant_to_group_chat_room(
    _conn: &mut PgConnection,
    _removing_user: &ChatRoomParticipants,
    remover_user_id: i32,
) -> Result<(), String> {
    if !is_group_chat(_conn, _removing_user.chat_room_id) {
        return Err(format!("couldn't find the group chat room"));
    }

    // checking if the remover is the admin
    // this will also check if the chat room id is a group chat room or not
    if remover_user_id != get_group_owner_by_id(_conn, _removing_user.chat_room_id).unwrap() {
        return Err(format!(
            "user id {} is not allowed to remove users from group",
            remover_user_id
        ));
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
        return Err(format!(
            "user id {} is not in the group chat room id {}",
            _removing_user.user_id, _removing_user.chat_room_id
        ));
    }

    // deleting the user from the participants table
    diesel::delete(
        chat_room_participants.filter(chat_room_participants::user_id.eq(_removing_user.user_id)),
    )
    .execute(_conn)
    .unwrap();

    Ok(())
}

pub fn add_new_message(_conn: &mut PgConnection, _message: &Messages) -> Result<i32, String> {
    // checking the existence of the chat room
    assert!(is_valid_chatroom(_conn, _message.chat_room_id));

    // checking the existence of the users in the chat room
    assert!(is_user_in_chat_room(
        _conn,
        _message.chat_room_id,
        _message.sender_id
    ));
    assert!(is_user_in_chat_room(
        _conn,
        _message.chat_room_id,
        _message.recipient_id
    ));

    // adding the message to the messages table

    let new_message_info: QMessages = diesel::insert_into(messages::table)
        .values(_message)
        .returning(QMessages::as_select())
        .get_result(_conn)
        .unwrap();

    Ok(new_message_info.message_id)
}

pub fn del_message() {}

// -- ChatRooms/ Message history GETTER functions -- //

pub fn get_p2p_chat_history() {}

pub fn get_group_chat_history() {}

pub fn get_user_group_chat_room_ids() {}

pub fn get_user_p2p_chat_room_ids() {}

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

pub fn get_group_chat_id_by_name(
    _conn: &mut PgConnection,
    _chat_room_name: &String,
) -> Option<i32> {
    let _chat_room_id: Vec<QChatRooms> = chat_rooms
        .filter(chat_rooms::room_name.eq(_chat_room_name))
        .select(QChatRooms::as_returning())
        .load(_conn)
        .unwrap();

    if _chat_room_id.len() != 1 {
        return None;
    } else {
        return Some(_chat_room_id[0].chat_room_id);
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
    } else if chat_room_info[0].room_name == "private room" {
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
