#![feature(decl_macro)] // helps us with the routing of our application

#[macro_use]
extern crate rocket; // imports all of the macros from the rocket crate

use std::sync::Arc;
use std::sync::Mutex;
use std::sync::MutexGuard;

use chatuza_db::api_models::*;
use chatuza_db::db_models::*;
use chatuza_db::db_models::*;
use chatuza_db::*;
// use rocket::data::FromDataSimple;
use chatuza_db::*; // Assuming your Diesel models and functions are here
use diesel::prelude::*;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::request::Form;
use rocket::request::{FromRequest, Request};
use rocket::State;
use rocket::*;
use rocket_contrib::json;
use rocket_contrib::json::Json;
use rocket_contrib::templates::Template;
use serde::Serialize;
// use rocket::
// getter api's //
#[get("/user-via-username/<username>")]
fn get_user_via_username(username: String) -> Json<QUsers> {
    let mut conn = establish_connection();
    Json(get_user_with_username(&mut conn, username.as_str()).unwrap())
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

#[get("/group_id_by_name/<chatroom_name>")]
fn get_chatroom_id_via_name(chatroom_name: String) -> Json<i32> {
    let mut conn = establish_connection();
    Json(get_group_chat_id_by_name(&mut conn, &chatroom_name).unwrap())
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

#[post("/new-user", data = "<new_user>")]
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
                new_user
            ],
        )
        // .attach(DbConn::fairing())
        .launch();
    // needs the "cargo build and then cargo run to be ran oin the fucking serve"
}
