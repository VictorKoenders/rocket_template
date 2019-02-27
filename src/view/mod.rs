use crate::models::user::User;
use crate::rocket_utils::ResponseResult;
use rocket::Rocket;

mod auth;

#[get("/")]
pub fn index(user: User) -> ResponseResult {
    Ok(rocket::Response::build()
        .sized_body(std::io::Cursor::new(format!("Hello {:?}", user)))
        .finalize())
}

pub fn route(r: Rocket) -> Rocket {
    r.mount(
        "/",
        routes![
            index,
            auth::index,
            auth::login_submit,
            auth::register_submit
        ],
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
