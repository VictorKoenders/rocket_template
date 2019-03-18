#![feature(proc_macro_hygiene, decl_macro, never_type, type_alias_enum_variants)]
#![allow(warnings)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

pub mod rocket_utils;
pub mod view;
pub mod models;

fn main() {
    dotenv::dotenv().unwrap();

    let rocket = rocket::ignite().attach(rocket_utils::Connection::fairing());
    let rocket = view::route(rocket);

    rocket.launch();
}
