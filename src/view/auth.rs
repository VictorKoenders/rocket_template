use super::ToResponse;
use crate::rocket_utils::ResponseResult;
use askama::Template;
use rocket::http::Cookies;
use rocket::request::Form;

#[derive(Template, Default)]
#[template(path = "landing_page.html")]
pub struct IndexModel;

#[get("/")]
pub fn login() -> ResponseResult {
    IndexModel::default().to_response()
}

#[derive(Template, Default)]
#[template(path = "login.html")]
pub struct LoginViewModel<'a> {
    pub error: Option<&'a str>,
    pub username: &'a str,
}

#[derive(FromForm)]
pub struct LoginSubmitModel {
    pub loginname: String,
    pub password: String,
}

#[post("/user/login", data = "<form>")]
pub fn login_submit(
    form: Form<LoginSubmitModel>,
    conn: crate::Connection,
    mut cookies: Cookies,
) -> ResponseResult {
    unimplemented!()
}
