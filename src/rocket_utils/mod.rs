mod either;
mod error;

pub use either::Either;
pub use error::{Error, ResponseResult};

use rocket::request::{FromRequest, Outcome, Request};

#[database("DATABASE")]
pub struct Connection(diesel::PgConnection);
impl Connection {
    pub fn get(&self) -> &diesel::PgConnection {
        &*self
    }
}

#[derive(Clone, Copy)]
pub struct PeerAddr(pub std::net::IpAddr);

impl<'a, 'r> FromRequest<'a, 'r> for PeerAddr {
    type Error = !;

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        println!("PeerAddr from request");
        let ip = request
            .client_ip()
            .unwrap_or_else(|| std::net::Ipv4Addr::LOCALHOST.into());
        Outcome::Success(PeerAddr(ip))
    }
}
