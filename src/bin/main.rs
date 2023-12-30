#![feature(decl_macro)] // helps us with the routing of our application

#[macro_use]
extern crate rocket; // imports all of the macros from the rocket crate

use std::io::Read;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::MutexGuard;
use std::thread;
use std::time::Duration;

use chatuza_db::api_models::*;
use chatuza_db::db_models::*;
use chatuza_db::db_models::*;
use chatuza_db::*;
// use rocket::data::FromDataSimple;
use chatuza_db::wallet_lib::*;
use chatuza_db::*;
// Assuming your Diesel models and functions are here
use diesel::prelude::*;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::request::Form;
use rocket::request::FormItem;
use rocket::request::{FromRequest, Request};
use rocket::State;
use rocket::*;
use rocket_contrib::json;
use rocket_contrib::json::Json;
use serde::Serialize;

#[get("/user-via-username/<username>")]
fn get_user_via_username(username: String) -> Json<QUsers> {
    let mut conn = establish_connection();
    Json(get_user_with_username(&mut conn, username.as_str()).unwrap())
}

#[get("/user-profile-via-username/<username>")]
fn get_user_profile_via_username(username: String) -> Json<Result<UserProfiles, String>> {
    let mut conn = establish_connection();
    Json(get_user_profile_with_username(&mut conn, &username).unwrap_or_else(|| "".to_owned()))
}

#[get("/user-via-email/<email>")]
fn get_user_via_email(email: String) -> Json<QUsers> {
    let mut conn = establish_connection();
    Json(get_user_with_email(&mut conn, email.as_str()).unwrap())
}

#[get("/user-via-userid/<user_id>")]
fn get_user_via_user_id(user_id: i32) -> Json<QUsers> {
    let mut conn = establish_connection();
    Json(get_user_with_user_id(&mut conn, user_id).unwrap())
}

#[get("/chatroom-participants-by-id/<chatroom_id>")]
fn get_chatroom_by_id(chatroom_id: i32) -> Json<Vec<ChatRoomParticipants>> {
    let mut conn = establish_connection();
    Json(get_chat_room_participants_by_id(&mut conn, chatroom_id).unwrap())
}

#[get("/chatroom-participants-by-name/<chatroom_name>")]
fn get_chatroom_by_name(chatroom_name: String) -> Json<Vec<ChatRoomParticipants>> {
    let mut conn = establish_connection();
    Json(get_chat_room_participants_by_name(&mut conn, &chatroom_name).unwrap())
}

#[get("/group-owner/<owner_id>")]
fn get_group_owner_via_id(owner_id: i32) -> Json<i32> {
    let mut conn = establish_connection();
    Json(get_group_owner_by_id(&mut conn, owner_id).unwrap())
}

#[get("/group_by_name/<chatroom_name>")]
fn get_chatroom_id_via_name(chatroom_name: String) -> Json<QChatRooms> {
    let mut conn = establish_connection();
    Json(get_group_chat_by_name(&mut conn, &chatroom_name).unwrap())
}

#[get("/is_valid_gp/<chatroom_id>")]
fn validate_gp(chatroom_id: i32) -> Json<bool> {
    let mut conn = establish_connection();
    Json(is_group_chat(&mut conn, chatroom_id))
}

#[get("/is_valid_chatroom/<chatroom_id>")]
fn validate_cr(chatroom_id: i32) -> Json<bool> {
    let mut conn = establish_connection();
    Json(is_valid_chatroom(&mut conn, chatroom_id))
}

#[get("/is_valid_user/<user_id>")]
fn validate_user(user_id: i32) -> Json<bool> {
    let mut conn = establish_connection();
    Json(is_valid_user(&mut conn, user_id))
}

#[get("/is_user_in_chatroom/<user_id>/<chatroom_id>")]
fn validate_chatroom_user(user_id: i32, chatroom_id: i32) -> Json<bool> {
    let mut conn = establish_connection();
    Json(is_user_in_chat_room(&mut conn, user_id, chatroom_id))
}

#[get("/user_all_gp/<user_id>")]
fn get_all_user_groups(user_id: i32) -> Json<Vec<QChatRooms>> {
    let mut conn = establish_connection();
    Json(get_user_p2p_chat_rooms_by_user_id(&mut conn, user_id).unwrap())
}

#[get("/user_all_p2p/<user_id>")]
fn get_all_user_p2p(user_id: i32) -> Json<Vec<QChatRooms>> {
    let mut conn = establish_connection();
    Json(get_user_group_chat_rooms_by_user_id(&mut conn, user_id).unwrap())
}

#[post("/create-user", data = "<new_user>")]
fn new_user(new_user: Form<NewUserIN>) -> Json<QUsers> {
    let mut conn = establish_connection();
    Json(
        add_new_user(
            &mut conn,
            &Users {
                username: new_user.username_in.clone(),
                email: new_user.email_in.clone(),
                password: new_user.password_in.clone(),
            },
            &mut UserProfiles {
                user_id: 0,
                bio: new_user.bio_in.clone(),
                profile_picture: new_user.profile_picture_in.clone(),
            },
        )
        .unwrap(),
    )
}

#[post("/update-user-credits", data = "<new_credits>")]
fn update_user_conditionals(new_credits: Form<UpdatedUserCreditsIN>) -> Json<QUsers> {
    let mut conn = establish_connection();
    Json(
        update_user_credits(
            &mut conn,
            &new_credits.username_out.clone(),
            &Users {
                username: new_credits.username_in.clone(),
                email: new_credits.email_in.clone(),
                password: new_credits.password_in.clone(),
            },
        )
        .unwrap(),
    )
}

#[post("/update-user-profile", data = "<new_profile>")]
fn update_user_profile_api(new_profile: Form<UpdatedUserProfileIN>) -> Json<UserProfiles> {
    let mut conn = establish_connection();
    Json(
        update_user_profile(
            &mut conn,
            &new_profile.username_in.clone(),
            &mut UserProfiles {
                user_id: 0,
                bio: Some(new_profile.bio_in.clone()),
                profile_picture: Some(new_profile.profile_picture_in.clone()),
            },
        )
        .unwrap(),
    )
}

#[post("/delete-user", data = "<username>")]
fn delete_user_via_username(username: Form<SinglePostUsername>) -> Json<bool> {
    let mut conn = establish_connection();
    Json(delete_user(&mut conn, &username.username_in.clone()).unwrap())
}

#[post("/create-p2p", data = "<new_p2p_info>")]
fn new_p2p(new_p2p_info: Form<NewP2PChatRoomIN>) -> Json<QChatRooms> {
    let mut conn = establish_connection();
    let req_user = get_user_with_username(
        &mut conn,
        new_p2p_info.requestor_username_in.clone().as_str(),
    )
    .unwrap();
    let acc_user = get_user_with_username(
        &mut conn,
        new_p2p_info.acceptor_username_in.clone().as_str(),
    )
    .unwrap();

    Json(
        add_new_p2p_chat_room(
            &mut conn,
            req_user.user_id,
            acc_user.user_id,
            new_p2p_info.chat_room_pubkey_in.clone(),
        )
        .unwrap(),
    )
}

#[post("/delete-p2p", data = "<new_gp_info>")]
fn delete_p2p(new_gp_info: Form<DeleteP2PChatRoomIN>) -> Json<bool> {
    let mut conn = establish_connection();
    Json(
        delete_p2p_chat_room(
            &mut conn,
            &new_gp_info.remover_username_in.clone(),
            &new_gp_info.contact_username_in.clone(),
        )
        .unwrap(),
    )
}

#[post("/create-gp", data = "<new_gp_info>")]
fn new_gp(new_gp_info: Json<NewGroupChatRoomIN>) -> Json<QChatRooms> {
    let mut conn = establish_connection();

    Json(
        add_new_group_chat_room(
            &mut conn,
            &ChatRooms {
                room_name: new_gp_info.room_name_in.clone(),
                room_description: new_gp_info.room_description_in.clone(),
                chat_room_pubkey: new_gp_info.chat_room_pubkey.as_bytes().to_vec(),
            },
            &(new_gp_info.group_owner_username_in.clone()),
            new_gp_info.group_members_in.to_owned(),
        )
        .unwrap(),
    )
}

#[post("/update-gp-info", data = "<new_gp_info>")]
fn update_gp(new_gp_info: Form<UpdatedGroupChatRoomInfoIN>) -> Json<QChatRooms> {
    let mut conn = establish_connection();
    Json(
        update_group_chat_room_info(
            &mut conn,
            &new_gp_info.old_chat_room_name_in.clone(),
            &UpdatableChatRooms {
                room_name: new_gp_info.room_name_in.clone(),
                room_description: new_gp_info.room_description_in.clone(),
            },
            &new_gp_info.editor_username_in.clone(),
        )
        .unwrap(),
    )
}

#[post("/add-user-to-gp", data = "<new_participant>")]
fn add_user_to_gp(new_participant: Form<NewGroupChatParticipantIN>) -> Json<ChatRoomParticipants> {
    let mut conn = establish_connection();
    let _chat_room_id =
        get_group_chat_by_name(&mut conn, &new_participant.chat_room_name_in.clone())
            .unwrap()
            .chat_room_id;
    let _user_id = get_user_with_username(&mut conn, new_participant.username_in.clone().as_str())
        .unwrap()
        .user_id;
    Json(
        add_participant_to_group_chat_room(
            &mut conn,
            &ChatRoomParticipants {
                chat_room_id: _chat_room_id,
                user_id: _user_id,
                is_admin: false,
            },
            &new_participant.adder_username_in,
        )
        .unwrap(),
    )
}

#[post("/delete-user-from-gp", data = "<removing_participant>")]
fn delete_user_from_gp(removing_participant: Form<GroupChatParticipantToRemoveIN>) -> Json<bool> {
    let mut conn = establish_connection();
    let _chat_room_id =
        get_group_chat_by_name(&mut conn, &removing_participant.chat_room_name_in.clone())
            .unwrap()
            .chat_room_id;
    let _removing_user_id =
        get_user_with_username(&mut conn, removing_participant.username_in.clone().as_str())
            .unwrap()
            .user_id;
    let _remover_user_id = get_user_with_username(
        &mut conn,
        removing_participant.remover_username_in.clone().as_str(),
    )
    .unwrap()
    .user_id;

    let _admin = get_group_owner_by_id(&mut conn, _chat_room_id).unwrap();

    Json(
        del_participant_from_group_chat_room(
            &mut conn,
            &ChatRoomParticipants {
                chat_room_id: _chat_room_id,
                user_id: _removing_user_id,
                is_admin: _remover_user_id == _admin,
            },
            _remover_user_id,
        )
        .unwrap(),
    )
}

#[post("/delete-gp", data = "<new_gp_info>")]
fn delete_gp(new_gp_info: Form<DeleteGroupChatRoomIN>) -> Json<bool> {
    let mut conn = establish_connection();
    Json(
        delete_group_chat_room(
            &mut conn,
            &new_gp_info.chat_room_name_in.clone(),
            &new_gp_info.remover_username_in.clone(),
        )
        .unwrap(),
    )
}

#[post("/add-solana-wallet", data = "<new_wallet_info>")]
fn add_solana_wallet(new_wallet_info: Form<NewWalletIn>) -> Json<QSolanaWallet> {
    let mut conn = establish_connection();
    let _user_id = get_user_with_username(&mut conn, &new_wallet_info.username_in)
        .unwrap()
        .user_id;
    Json(
        initialize_new_solana_wallet(
            &mut conn,
            &SolanaWallet {
                user_id: _user_id,
                wallet_addr: new_wallet_info.wallet_addr_in.as_bytes().to_vec(),
                wallet_backup: new_wallet_info.wallet_backup_in.as_bytes().to_vec(),
            },
        )
        .unwrap(),
    )
}
#[post("/add-tron-wallet", data = "<new_wallet_info>")]
fn add_tron_wallet(new_wallet_info: Form<NewWalletIn>) -> Json<QTronWallet> {
    let mut conn = establish_connection();
    let _user_id = get_user_with_username(&mut conn, &new_wallet_info.username_in)
        .unwrap()
        .user_id;
    Json(
        initialize_new_tron_wallet(
            &mut conn,
            &TronWallet {
                user_id: _user_id,
                wallet_addr: new_wallet_info.wallet_addr_in.as_bytes().to_vec(),
                wallet_backup: new_wallet_info.wallet_backup_in.as_bytes().to_vec(),
            },
        )
        .unwrap(),
    )
}
#[get("/get-tron-addr-by-username/<username>")]
fn get_tron_addr(username: String) -> Json<String> {
    let mut conn = establish_connection();
    let _user_id = get_user_with_username(&mut conn, &username)
        .unwrap()
        .user_id;

    Json(
        String::from_utf8_lossy(
            get_user_tron_wallet(&mut conn, _user_id)
                .unwrap()
                .wallet_addr
                .as_slice(),
        )
        .as_ref()
        .to_string(),
    )
}

#[get("/get-solana-addr-by-username/<username>")]
fn get_solana_addr(username: String) -> Json<String> {
    let mut conn = establish_connection();
    let _user_id = get_user_with_username(&mut conn, &username)
        .unwrap()
        .user_id;

    Json(
        String::from_utf8_lossy(
            get_user_solana_wallet(&mut conn, _user_id)
                .unwrap()
                .wallet_addr
                .as_slice(),
        )
        .as_ref()
        .to_string(),
    )
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Oh no the {} path doesn't exists !!", req.uri())
}

fn main() {
    let mut connection = &mut establish_connection();
    rocket::ignite()
        .register(catchers![not_found])
        .mount(
            "/api",
            routes![
                get_user_via_email,
                get_user_via_username,
                get_user_via_user_id,
                get_user_profile_via_username,
                get_chatroom_by_id,
                get_chatroom_by_name,
                get_group_owner_via_id,
                get_chatroom_id_via_name,
                validate_gp,
                validate_cr,
                validate_user,
                validate_chatroom_user,
                get_all_user_groups,
                get_all_user_p2p,
                // setters
                new_user,
                update_user_conditionals,
                update_user_profile_api,
                delete_user_via_username,
                new_p2p,
                delete_p2p,
                new_gp,
                update_gp,
                delete_gp,
                add_user_to_gp,
                delete_user_from_gp,
                get_tron_addr,
                get_solana_addr,
                add_solana_wallet,
                add_tron_wallet
            ],
        )
        // .attach(DbConn::fairing())
        .launch();
    // needs the "cargo build and then cargo run to be ran oin the fucking serve"
}
