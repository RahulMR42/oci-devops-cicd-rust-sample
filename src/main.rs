#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

use rocket_contrib::json::{Json, JsonValue};

mod model;
use model::Employee;
use rocket::response::status;


#[get("/")]
fn list_employees() -> Json<JsonValue> {
    Json(json!([
        {
            "eid": 1000,
            "name": "Julia",
            "department": "CXO"
        },
        {
            "eid": 1101,
            "name": "Rinil",
            "department": "HR"
        }
    ]))
}



fn main() {
    rocket::ignite()
        .mount("/employees", routes![list_employees])
        .launch();
}