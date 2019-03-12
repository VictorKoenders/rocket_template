use crate::models::user::User;
use crate::rocket_utils::ResponseResult;
use rocket::Rocket;

mod auth;

#[get("/")]
pub fn index(user: User) -> ResponseResult {
    format!("Hello {}", user.name).into()
}

#[get("/", rank = 2)]
pub fn landing() -> ResponseResult {
    "Not logged in".into()
}

pub fn route(r: Rocket) -> Rocket {
    r.mount(
        "/",
        routes![
            index,
            landing,
            // auth::index,
            // auth::login_submit,
            // auth::register_submit
        ],
    )
}
