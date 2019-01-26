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

use rocket::config::{Config, Environment, Value};
use std::collections::HashMap;

fn main() {
    dotenv::dotenv().unwrap();
    let mut database_config = HashMap::new();
    let mut databases = HashMap::new();

    database_config.insert("url", Value::from(std::env::var("DATABASE_URL").unwrap()));
    databases.insert("DATABASE", Value::from(database_config));

    let config = Config::build(Environment::Development)
        .extra("databases", databases)
        .finalize()
        .unwrap();

    let rocket = rocket::custom(config).attach(rocket_utils::Connection::fairing());
    let rocket = view::route(rocket);

    rocket.launch();
}
