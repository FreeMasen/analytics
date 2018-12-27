extern crate chrono;
extern crate env_logger;
#[macro_use]
extern crate lazy_static;
extern crate lettre;
extern crate lettre_email;
#[macro_use]
extern crate log;
extern crate postgres;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate toml;
extern crate uuid;
extern crate warp;
#[cfg(test)]
extern crate reqwest;

use std::{
    error::Error as StdError,
    num::ParseIntError,
};

use chrono::{DateTime, Utc};
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
        .map(landing_handler)
        .with(log);
    let exiting = warp::post2()
        .and(warp::path("exiting"))
        .and(warp::body::json())
        .map(exiting_handler)
        .with(log);
    let reporting = warp::get2()
        .and(warp::path("analytics"))
        .and(warp::path("reports"))
        .map(reports_handler)
        .with(log);
    let catch_all = warp::any().map(catch_all_handler).with(log);
    let analytics = warp::post2().and(warp::path("analytics")).and(landing.or(exiting));
    let routes = warp::any()
                    .and(analytics)
                    .or(reporting)
                    .or(catch_all);
    warp::serve(routes)
        .run(([127, 0, 0, 1], 5555));
}

fn landing_handler(info: LandingInfo, remote: String) -> impl Reply {
    info!(target: "analytics:info", "/analytics/landing {} {}", remote, info);
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
    ::std::thread::spawn(move|| {
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

fn reports_handler() -> impl Reply {
    let msg = match data::reports() {
        Ok(tables) => tables,
        Err(e) => return Response::builder().status(500).body(format!("{}", e)),
    };
    use lettre_email::EmailBuilder;
    use lettre::{EmailTransport, SmtpTransport};
    let email = match EmailBuilder::new()
        .from("r@robertmasen.pizza")
        .to("r.f.masen@gmail.com")
        .subject(format!("Weekly analytics report {}", chrono::Local::today()))
        .html(msg.clone())
        .build() {
        Ok(email) => email,
        Err(e) => return Response::builder().status(500).body(format!("{}", e)),
    };
    let mut mailer = match SmtpTransport::builder_unencrypted_localhost() {
        Ok(m) => m.build(),
        Err(e) => return Response::builder().status(500).body(format!("{}", e)),
    };
    match mailer.send(&email) {
        Ok(_) => Response::builder().body(msg),
        Err(e) => Response::builder().status(500).body(format!("{}", e))
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct LandingInfo {
    referrer: Option<String>,
    page: String,
    cookie: Option<Uuid>,
    when: DateTime<Utc>,
    prev_visit: Option<Uuid>,
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
    visit: Uuid,
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
        write!(f, "{} {} {}", self.visit, self.time, link)
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
    use chrono::Utc;
    use super::{LandingInfo, ExitingInfo, InitialResponse, main};
    #[test]
    fn test_server() -> Result<(), reqwest::Error> {
        debug!(target: "analytics:test", "starting test_server");
        ::std::thread::spawn(|| main());
        ::std::thread::sleep(::std::time::Duration::from_secs(2));
        let c = reqwest::Client::new();
        let addr = "http://localhost:5555/analytics";
        let first_body = LandingInfo {
            referrer: None,
            page: String::from("http://example.com"),
            cookie: None,
            when: Utc::now(),
            prev_visit: None,
        };
        let res: InitialResponse = c.post(&format!("{}/landing", addr))
                                                .header("x-client-address", "0.0.0.0")
                                                .json(&first_body)
                                                .send()?
                                                .json()?;
        debug!(target: "analytics:test", "res: {:?}", res);
        let second_body = ExitingInfo {
            visit: res.visit,
            time: 1000,
            link_clicked: None,
        };
        c.post(&format!("{}/exiting", addr))
                                .json(&second_body)
                                .send()?;
        debug!(target: "analytics:test",  "finishing test_server");
        ::std::thread::sleep(::std::time::Duration::from_secs(2));
        Ok(())
    }
}