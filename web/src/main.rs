#![feature(proc_macro_hygiene, decl_macro, never_type, type_alias_enum_variants)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

pub mod models;
pub mod rocket_utils;
pub mod view;

fn main() {
    dotenv::dotenv().unwrap();

    let rocket = rocket::ignite().attach(rocket_utils::Connection::fairing());
    let rocket = view::route(rocket);

    rocket.launch();
}
