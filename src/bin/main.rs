#![feature(decl_macro)] // helps us with the routing of our application

#[macro_use]
extern crate rocket; // imports all of the macros from the rocket crate

use rocket::request::Form;
use rocket::response::content::Json;
use rocket::*;
use rocket_contrib::templates::Template;

// getter api's //
#[get("/hello")]
fn hello() -> Json<&'static str> {
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
#[derive(FromForm, Debug)]
struct Book {
    name: String,
    pagesize: u32,
    author: String,
}

#[post("/add_book", data = "<book_form>")]
fn add_book(book_form: Form<Book>) -> String {
    let book: Book = book_form.into_inner();
    let mut dummy_db: Vec<Book> = Vec::new();
    dummy_db.push(book);
    format!("Book added successfully: {:?}", dummy_db)
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Oh no the {} path doesn't exists !!", req.uri())
}
fn main() {
    rocket::ignite()
        .register(catchers![not_found])
        .mount("/api", routes![hello, goodbye, add_book])
        .attach(Template::fairing())
        .launch();
    // needs the "cargo build and then cargo run to be ran oin the fucking serve"
}
