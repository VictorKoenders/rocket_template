use crate::models::request::RequestId;
use crate::rocket_utils::Connection;
use chrono::Utc;

#[derive(Debug)]
pub struct Error(failure::Error);

impl Error {
    pub fn from_status_code(status: rocket::http::Status) -> Error {
        Error(failure::format_err!("Server error: {:?}", status))
    }
}

impl<T> From<T> for Error
where
    T: Into<failure::Error>,
{
    fn from(t: T) -> Error {
        Error(t.into())
    }
}

pub trait RenderTemplate {
    fn render(&self) -> ResponseResult;
}

impl<T> RenderTemplate for T
where
    T: askama::Template,
{
    fn render(&self) -> ResponseResult {
        match askama::Template::render(self) {
            Ok(str) => str.into(),
            Err(e) => ResponseResult {
                result: Err(e.into()),
            },
        }
    }
}

pub struct ResponseResult {
    result: Result<rocket::Response<'static>, failure::Error>,
}

impl<'a> rocket::response::Responder<'a> for ResponseResult {
    fn respond_to(self, request: &rocket::Request) -> rocket::response::Result<'a> {
        let conn = request.guard::<Connection>().unwrap();
        let request_id = request.guard::<RequestId>().unwrap();
        let mut finalizer = request_id.finalize();
        let result = match self.result {
            Ok(response) => {
                finalizer.status = response.status().code;
                response.respond_to(request)
            }
            Err(e) => {
                finalizer.status = 500;
                eprintln!("{:?}", e);
                Ok(rocket::Response::build().status(rocket::http::Status::InternalServerError).sized_body(
                std::io::Cursor::new(format!("<html><body><h2>Internal server error</h2>If you see any of our code monkeys, please tell them this: {:?}</body></html>", request_id.0))).finalize())
            }
        };

        finalizer.finished_on = Utc::now();
        if let Err(e) = finalizer.save(&conn) {
            eprintln!("Could not save request finalizer: {:?}", e);
        }

        result
    }
}

impl From<String> for ResponseResult {
    fn from(str: String) -> ResponseResult {
        let response = rocket::Response::build()
            .status(rocket::http::Status::Ok)
            .header(rocket::http::ContentType::HTML)
            .sized_body(std::io::Cursor::new(str))
            .finalize();
        ResponseResult {
            result: Ok(response),
        }
    }
}
