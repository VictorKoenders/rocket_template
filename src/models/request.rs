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

#[derive(Debug, Clone, Copy)]
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

impl RequestId {
    pub fn finalize(&self) -> FinalizeRequest {
        FinalizeRequest {
            id: self.0,
            status: 0,
            finished_on: chrono::MIN_DATE.and_hms(0, 0, 0),
        }
    }
}

pub struct FinalizeRequest {
    id: Uuid,
    pub status: u16,
    pub finished_on: DateTime<Utc>,
}

impl FinalizeRequest {
    pub fn save(self, conn: &Connection) -> Result<(), failure::Error> {
        diesel::update(request_logs::table.find(self.id))
            .set((
                request_logs::dsl::response_code.eq(Some(i32::from(self.status))),
                request_logs::dsl::finished_on.eq(Some(self.finished_on)),
            ))
            .execute(&**conn)?;
        Ok(())
    }
}
