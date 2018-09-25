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
#[cfg(test)]
extern crate reqwest;

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
    env_logger::init();
    info!(target: "analytics:info", "Starting up");
    let log = warp::log("analytics:log");
    let landing = warp::post2()
        .and(warp::path("landing"))
        .and(warp::body::json())
        .and(warp::header("x-client-address"))
        .map(landing_handler);
    let exiting = warp::post2()
        .and(warp::path("exiting"))
        .and(warp::body::json())
        .map(exiting_handler);
    let catch_all = warp::any().map(catch_all_handler);
    let analytics = warp::post2().and(warp::path("analytics")).and(
        landing.or(exiting)
    );
    let routes = warp::any()
                    .and(analytics)
                    .or(catch_all)
                    .with(log);
    warp::serve(routes)
        .run(([127, 0, 0, 1], 5555));
}

fn landing_handler(info: LandingInfo, remote: String) -> impl Reply {
    info!(target: "analytics:info", "/analytics {} {}", remote, info);
    let res = match data::add_entry(&info, &remote) {
        Ok(info) => {
            info!(target: "analytics:info", "Successfully added entry to database");
            info
        },
        Err(e) => {
            error!(target: "analytics:error", "Error adding entry to database {}", e);
            return Response::builder()
                            .status(500)
                            .body(format!("error: {}", e))
        }
    };
    match serde_json::to_string(&res) {
        Ok(body) => {
            info!(target: "analytics:info", "Successfully converted info to JSON");
            Response::builder()
                        .body(body)
        },
        Err(e) => {
            error!(target: "analytics:error", "Error converting info to JSON, {}", e);
            Response::builder()
                        .status(500)
                        .body(format!("error: {}", e))
        },
    }
}

fn exiting_handler(info: ExitingInfo) -> impl Reply {
    info!(target: "analytics:info", "/analytics/exiting {:}", info);
    ::std::thread::spawn(move || {
        match data::update_entry(&info) {
            Ok(()) => info!(target: "analytics:info", "Successfully updated entry"),
            Err(e) => error!(target: "analytics:error", "Error updating entry {}", e),
        }
    });
    warp::reply()
}

fn catch_all_handler() -> impl Reply {
    info!(target: "analytics:info", "*");
    Response::builder()
        .body("<html><head></head><body><h1>analytics smoketest</h1></body>")
}

#[derive(Serialize, Deserialize, Debug)]
struct LandingInfo {
    referrer: Option<String>,
    page: String,
    cookie: Option<Uuid>,
    when: NaiveDateTime,
}

impl ::std::fmt::Display for LandingInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let referrer = if let Some(ref referrer) = self.referrer {
            referrer.as_str()
        } else {
            "None"
        };
        let cookie = if let Some(ref cookie) = self.cookie {
            format!("{}", cookie)
        } else {
            String::from("None")
        };
        write!(f, "{} {} {}", referrer, self.page, cookie)
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct InitialResponse {
    token: Uuid,
    visit: Uuid,
}

#[derive(Serialize, Deserialize, Debug)]
struct ExitingInfo {
    cookie: Uuid,
    #[serde(with = "time_parsing")]
    time: i64,
    link_clicked: Option<String>
}

impl ::std::fmt::Display for ExitingInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let link = if let Some(ref link) = self.link_clicked {
            link.as_str()
        } else {
            "None"
        };
        write!(f, "{} {} {}", self.cookie, self.time, link)
    }
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

#[cfg(test)]
mod test {
    use reqwest;
    use chrono::Local;
    #[test]
    fn test_server() -> Result<(), reqwest::Error> {
        ::std::thread::spawn(|| super::main());
        let c = reqwest::Client::new();
        let addr = "http://localhost:5555/analytics";
        let first_body = super::LandingInfo {
            referrer: None,
            page: String::from("http://example.com"),
            cookie: None,
            when: Local::now().naive_local(),
        };
        let res: super::InitialResponse = c.post(&format!("{}/landing", addr))
                                                .header("x-client-address", "0.0.0.0")
                                                .json(&first_body)
                                                .send()?
                                                .json()?;
        debug!(target: "analytics:test", "res: {:?}", res);
        let second_body = super::ExitingInfo {
            cookie: res.token,
            time: 1000,
            link_clicked: None
        };
        c.post(&format!("{}/exiting", addr))
                                .json(&second_body)
                                .send()?;
        Ok(())
    }
}