#![feature(proc_macro_hygiene, decl_macro, never_type, type_alias_enum_variants)]
#![allow(proc_macro_derive_resolution_fallback)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;

pub mod models;
pub mod rocket_utils;
pub mod schema;
pub mod view;

fn main() {
    dotenv::dotenv().unwrap();

    let rocket = rocket::ignite().attach(rocket_utils::Connection::fairing());
    let rocket = view::route(rocket);

    rocket.launch();
}
