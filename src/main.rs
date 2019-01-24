#![feature(proc_macro_hygiene, decl_macro)]
#![allow(proc_macro_derive_resolution_fallback)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;

pub mod rocket_utils;
pub mod schema;
pub mod view;

#[database("DATABASE_URL")]
pub struct Connection(diesel::PgConnection);

fn main() {
    dotenv::dotenv().unwrap();
    let rocket = rocket::ignite();
    let rocket = view::route(rocket);

    rocket.launch();
}
