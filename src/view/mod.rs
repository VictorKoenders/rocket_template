use crate::models::user::User;
use crate::rocket_utils::ResponseResult;
use rocket::Rocket;

// mod auth;

#[get("/")]
pub fn index(user: User) -> ResponseResult {
    ResponseResult::from_string(format!("Hello {}", user.name))
}

pub fn route(r: Rocket) -> Rocket {
    r.mount(
        "/",
        routes![
            index,
            // auth::index,
            // auth::login_submit,
            // auth::register_submit
        ],
    )
}

