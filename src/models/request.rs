use crate::rocket_utils::{Connection, Error};
use crate::schema::request_logs;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use uuid::Uuid;

#[derive(Insertable)]
#[table_name = "request_logs"]
pub struct RequestInsert<'a> {
    url: &'a str,
    headers: String,
    created_on: DateTime<Utc>,
}

#[derive(Clone, Copy)]
pub struct RequestId(pub Uuid);

impl<'a, 'r> FromRequest<'a, 'r> for RequestId {
    type Error = Error;
    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let conn = match request.guard::<Connection>() {
            Outcome::Success(c) => c,
            Outcome::Forward(()) => return Outcome::Forward(()),
            Outcome::Failure((e, ())) => {
                return Outcome::Failure((e, Error::from_status_code(e)));
            }
        };

        let request_insert = RequestInsert {
            url: request.uri().path(),
            headers: request.headers().iter().fold(String::new(), |acc, header| {
                format!("{}{}: {}\n", acc, header.name(), header.value())
            }),
            created_on: Utc::now(),
        };

        let id = match diesel::insert_into(request_logs::table)
            .values(request_insert)
            .returning(request_logs::dsl::id)
            .get_result(&*conn)
        {
            Ok(id) => id,
            Err(e) => return Outcome::Failure((Status::InternalServerError, e.into())),
        };

        Outcome::Success(*request.local_cache(|| RequestId(id)))
    }
}
