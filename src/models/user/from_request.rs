use super::{Token, User};
use crate::rocket_utils::{Connection, Error, PeerAddr};
use rocket::request::{FromRequest, Outcome, Request};
use uuid::Uuid;

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = Error;
    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let mut cookies = request.cookies();
        let (uid_cookie, tid_cookie) =
            match (cookies.get_private("UID"), cookies.get_private("TID")) {
                (Some(uid), Some(tid)) => (uid, tid),
                _ => return Outcome::Forward(()),
            };

        let mut remove_tokens_and_forward = || {
            cookies.remove(uid_cookie.clone());
            cookies.remove(tid_cookie.clone());
            Outcome::<Self, Self::Error>::Forward(())
        };

        let (user_id, token_id) = match (
            Uuid::parse_str(uid_cookie.value()),
            Uuid::parse_str(tid_cookie.value()),
        ) {
            (Ok(user_id), Ok(token_id)) => (user_id, token_id),
            _ => return remove_tokens_and_forward(),
        };

        let conn = request.guard::<Connection>().unwrap();
        let peer_addr = request.guard::<PeerAddr>().unwrap();

        let user = match User::load_by_id(&conn, user_id) {
            Ok(u) => u,
            Err(e) => {
                eprintln!("Could not load user: {:?}", e);
                return remove_tokens_and_forward();
            }
        };

        let token = match Token::load_by_user_and_id(&conn, &user, token_id) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("Could not load user token: {:?}", e);
                return remove_tokens_and_forward();
            }
        };

        if token.ip != peer_addr.0.to_string() {
            remove_tokens_and_forward()
        } else {
            Outcome::Success(user)
        }
    }
}
