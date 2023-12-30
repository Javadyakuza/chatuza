#![feature(decl_macro)] // helps us with the routing of our application

#[macro_use]
extern crate rocket; // imports all of the macros from the rocket crate

use chatuza_db::api_models::*;
use chatuza_db::db_models::*;
use chatuza_db::db_models::*;
use chatuza_db::*;
// use rocket::data::FromDataSimple;
use rocket::request::Form;
use rocket::*;
use rocket_contrib::json;
use rocket_contrib::json::Json;
use rocket_contrib::templates::Template;
use serde::Serialize;

// getter api's //
// #[get("/get-user-via-username/<username>")]
// fn get_user_via_username(username: String) -> Users {
//     get_user_with_username(connection, username)
// }

#[get("/get-user-via-email")]
fn get_user_with_email() -> Json<&'static str> {
    Json(
        "{
    'status': 'success',
    'message': 'Hello API!'
  }",
    )
}
#[get("/get-user-via-user-id")]
fn get_user_with_user_id() -> Json<&'static str> {
    Json(
        "{
    'status': 'success',
    'message': 'Hello API!'
  }",
    )
}

#[get("/goodbye")]
fn goodbye() -> Json<&'static str> {
    Json(
        "{
    'status': 'success',
    'message': 'goodbye API!'
  }",
    )
}
#[derive(Debug)]
struct Book {
    name: String,
    pagesize: String,
    author: String,
}

#[post("/add_book", data = "<book_form>")]
fn add_book(book_form: Json<Book>) -> Json<Book> {
    // let book: Book = book_form.into_inner();
    // let mut dummy_db: Vec<Book> = Vec::new();
    // dummy_db.push(book);
    format!("Book fucker successfully: {:?}", &book_form);
    Json(book_form.into_inner())
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Oh no the {} path doesn't exists !!", req.uri())
}
fn main() {
    let connection = &mut establish_connection();
    rocket::ignite()
        .register(catchers![not_found])
        .mount("/api", routes![add_book])
        .attach(Template::fairing())
        .launch();
    // needs the "cargo build and then cargo run to be ran oin the fucking serve"
}
