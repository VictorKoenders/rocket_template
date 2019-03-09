
#[derive(Debug)]
pub struct Error(failure::Error);

impl Error {
    pub fn from_status_code(status: rocket::http::Status) -> Error {
        Error(failure::format_err!("Server error: {:?}", status))
    }
}

impl<T> From<T> for Error where T: Into<failure::Error> {
    fn from(t: T) -> Error {
        Error(t.into())
    }
}


pub struct ResponseResult {
    result: Result<rocket::Response<'static>, failure::Error>,
}

impl<'a> rocket::response::Responder<'a> for ResponseResult {
    fn respond_to(self, request: &rocket::Request) -> rocket::response::Result<'a> {
        unimplemented!()
    }
}

impl ResponseResult {
    pub fn from_string(str: String) -> ResponseResult {
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

impl<T> From<T> for ResponseResult where T : askama::Template {
    fn from(t: T) -> ResponseResult {
        match t.render() {
            Ok(str) => 
                ResponseResult::from_string(str)
            ,
            Err(e) => ResponseResult {
                result: Err(e.into()),
            }
        }
    }
}

