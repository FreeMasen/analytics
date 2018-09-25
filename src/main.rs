extern crate chrono;
extern crate env_logger;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate postgres;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate toml;
extern crate uuid;
#[macro_use]
extern crate warp;

use std::{
    error::Error as StdError,
    num::ParseIntError,
};

use chrono::{NaiveDateTime};
use uuid::Uuid;
use warp::{
    Filter,
    http::{
        Response,
    },
    reply::Reply,
};
use postgres::Error as PError;

mod data;
mod time_parsing;

fn main() {
    let landing = warp::post2()
        .and(warp::path("analytics"))
        .and(warp::body::json())
        .and(warp::header("x-client-address"))
        .map(landing_handler);
    let exiting = warp::post2()
        .and(path!("analytics" / "exiting"))
        .and(warp::body::json())
        .map(exiting_handler);

    let analytics = warp::post2()
                        .and(landing)
                        .or(exiting);
    warp::serve(analytics)
        .run(([127, 0, 0, 1], 5555));
}

fn landing_handler(info: LandingInfo, remote: String) -> impl Reply {
    println!("/analytics \n{}\n{:#?}", remote, info);
    let res = match data::add_entry(&info, &remote) {
        Ok(info) => info,
        Err(e) => return Response::builder()
                            .status(500)
                            .body(format!("error: {}", e))
    };
    match serde_json::to_string(&res) {
        Ok(body) => Response::builder()
                        .body(body),
        Err(e) => Response::builder()
                        .status(500)
                        .body(format!("error: {}", e)),
    }
}

fn exiting_handler(info: ExitingInfo) -> impl Reply {
    println!("exiting info: {:#?}", info);
    ::std::thread::spawn(move || {
        match data::update_entry(&info) {
            Ok(()) => (),
            Err(e) => println!("Error updating entry {}", e),
        }
    });
    warp::reply()
}

#[derive(Deserialize, Debug)]
struct LandingInfo {
    referrer: Option<String>,
    page: String,
    cookie: Option<Uuid>,
    when: NaiveDateTime,
}

#[derive(Serialize, Debug)]
struct InitialResponse {
    token: Uuid,
    visit: Uuid,
}

#[derive(Deserialize, Debug)]
struct ExitingInfo {
    cookie: Uuid,
    #[serde(with = "time_parsing")]
    time: i64,
    link_clicked: Option<String>
}
#[derive(Debug)]
enum Error {
    Other(String),
    Postgres(PError),
    ParseInt(ParseIntError),
}

impl StdError for Error {
    fn cause(&self) -> Option<&StdError> {
        match self {
            Error::Other(_) => None,
            Error::Postgres(ref e) => Some(e),
            Error::ParseInt(ref e) => Some(e),
        }
    }
}

impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        if let Some(ref inner) = self.cause() {
            inner.fmt(f)
        } else {
            match self {
                Error::Other(s) => s.fmt(f),
                _ => unreachable!()
            }
        }
    }
}

impl From<PError> for Error {
    fn from(other: PError) -> Self {
        Error::Postgres(other)
    }
}

impl From<ParseIntError> for Error {
    fn from(other: ParseIntError) -> Self {
        Error::ParseInt(other)
    }
}