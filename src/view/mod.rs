use rocket::Rocket;

mod auth;

pub fn route(r: Rocket) -> Rocket {
    r.mount(
        "/",
        routes![auth::index, auth::login_submit, auth::register_submit],
    )
}

pub trait ToResponse {
    fn to_response(&self) -> crate::rocket_utils::ResponseResult;
}

impl<T> ToResponse for T
where
    T: askama::Template,
{
    fn to_response(&self) -> crate::rocket_utils::ResponseResult {
        let str = self.render()?;
        let response = rocket::Response::build()
            .status(rocket::http::Status::Ok)
            .header(rocket::http::ContentType::HTML)
            .sized_body(std::io::Cursor::new(str))
            .finalize();
        Ok(response)
    }
}
