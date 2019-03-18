#![feature(proc_macro_hygiene, decl_macro, never_type, type_alias_enum_variants)]

#[macro_use]
pub extern crate diesel;

mod models;
mod schema;

pub use crate::models::*;

pub type Conn = diesel::pg::PgConnection;
