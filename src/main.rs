#![feature(decl_macro)]

#[macro_use] extern crate rocket;

use rocket::Request;
use rocket::response::content::Json;
use rocket::request::Form;
use rocket_contrib::templates::Template;
use serde::Serialize;



#[get("/hello")]
fn hello() -> Json<&'static str> {
  Json("{
    'status': 'In',
    'message': 'Hello World with RUST & Rocket!'
  }")
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    print!("{}", req);
    format!("We lost with the path '{}'", req.uri())
}


fn main() {
  rocket::ignite()
    .register(catchers![not_found])
    .mount("/api", routes![hello])
    .launch();
}