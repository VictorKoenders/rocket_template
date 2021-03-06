mod from_request;

use crate::rocket_utils::Connection;
use crate::schema::{user_tokens, users};
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable, Debug)]
pub struct User {
    pub id: Uuid,
    pub register_request_id: Uuid,
    pub name: String,
    pub login_name: String,
    pub password: String,
    pub email: String,
    pub email_confirmed_request_id: Option<Uuid>,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct UserInsert<'a> {
    pub register_request_id: Uuid,
    pub name: &'a str,
    pub login_name: &'a str,
    pub password: &'a str,
    pub email: &'a str,
}

#[cfg(debug_assertions)]
const PASSWORD_ITERATION_COUNT: (u32, u32) = (1_000, 1_100);
#[cfg(not(debug_assertions))]
const PASSWORD_ITERATION_COUNT: (u32, u32) = (10_000, 11_000);

impl User {
    pub fn load_by_id(conn: &Connection, id: Uuid) -> QueryResult<User> {
        users::table.find(id).get_result(conn.get())
    }

    pub fn load_by_login_name(conn: &Connection, name: &str) -> QueryResult<User> {
        users::table
            .filter(users::dsl::login_name.eq(name))
            .get_result(conn.get())
    }

    pub fn load_by_email(conn: &Connection, email: &str) -> QueryResult<User> {
        users::table
            .filter(users::dsl::email.eq(email))
            .get_result(conn.get())
    }

    pub fn verify_password(&self, password: &str) -> bool {
        pbkdf2::pbkdf2_check(password, &self.password).is_ok()
    }

    pub fn register(
        conn: &Connection,
        login_name: &str,
        password: &str,
        email: &str,
        request_id: Uuid,
    ) -> QueryResult<User> {
        use rand::Rng;
        let iteration_count =
            rand::thread_rng().gen_range(PASSWORD_ITERATION_COUNT.0, PASSWORD_ITERATION_COUNT.1);

        let password = pbkdf2::pbkdf2_simple(password, iteration_count).unwrap();
        let insert = UserInsert {
            register_request_id: request_id,
            name: login_name,
            login_name,
            email,
            password: &password,
        };
        diesel::insert_into(users::table)
            .values(&insert)
            .get_result(conn.get())
    }
}

#[derive(Queryable)]
pub struct Token {
    pub id: Uuid,
    pub user_id: Uuid,
    pub created_on: DateTime<Utc>,
    pub created_request_id: Uuid,
    pub ip: String,
}

#[derive(Insertable)]
#[table_name = "user_tokens"]
pub struct TokenInsert<'a> {
    pub user_id: Uuid,
    pub created_on: DateTime<Utc>,
    pub created_request_id: Uuid,
    pub ip: &'a str,
}

impl Token {
    pub fn create_for_user(
        conn: &Connection,
        user_id: Uuid,
        request_id: Uuid,
        ip: &str,
    ) -> QueryResult<Token> {
        diesel::insert_into(user_tokens::table)
            .values(TokenInsert {
                user_id,
                created_on: Utc::now(),
                created_request_id: request_id,
                ip,
            })
            .get_result(conn.get())
    }

    pub fn load_by_user_and_id(conn: &Connection, user: &User, id: Uuid) -> QueryResult<Token> {
        user_tokens::table
            .filter(
                user_tokens::dsl::id
                    .eq(id)
                    .and(user_tokens::dsl::user_id.eq(user.id)),
            )
            .get_result(conn.get())
    }
}
