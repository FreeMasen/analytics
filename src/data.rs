use super::{
    LandingInfo,
    ExitingInfo,
    Error,
    InitialResponse,
};
use postgres::{
    Connection,
    TlsMode,
};
use toml::from_str;
use uuid::Uuid;

lazy_static !{
    static ref CONFIG: DbConfig = from_str(include_str!("../dbinfo.toml")).expect("Unable to parse dbinfo.toml");
}

pub(crate) fn add_entry(info: &LandingInfo, ip: &str) -> Result<InitialResponse, Error> {
    let conn = get_connection()?;
    let rows = conn.query("SELECT token, visit FROM add_session2($1, $2, $3, $4, $5, $6)", &[&info.cookie, &ip, &info.referrer, &info.page, &info.when, &info.prev_visit])?;
    let only = rows.get(0);
    let token: Uuid = only.get(0);
    let visit: Uuid = only.get(1);
    Ok(InitialResponse {
        token,
        visit,
    })
}

pub(crate) fn update_entry(info: &ExitingInfo) -> Result<(), Error> {
    let conn = get_connection()?;
    conn.execute("SELECT update_session($1, $2, $3)", &[&info.visit, &info.time, &info.link_clicked])?;
    Ok(())
}

fn get_connection() -> Result<Connection, Error> {
    let conn_str = CONFIG.to_string();
    let ret = Connection::connect(conn_str.as_str(), TlsMode::None)?;
    Ok(ret)
}

#[derive(Deserialize)]
struct DbConfig {
    user: String,
    domain: String,
    port: usize,
    password: String,
}

impl ToString for DbConfig {
    fn to_string(&self) -> String {
        format!("postgres://{}:{}@{}:{}/analytics", self.user, self.password, self.domain, self.port)
    }
}

#[cfg(test)]
mod test {
    use uuid::Uuid;
    #[test]
    fn simple() {
        let initial = super::LandingInfo {
            referrer: Some("http://reddit.com/r/rust".into()),
            page: "http://wiredforge.com/blog/getpid/index.html".into(),
            cookie: None,
            when: super::super::chrono::Utc::now(),
        };
        debug!(target: "analytics:test", "initial request: \n-----------\n{:?}\n----------", initial);
        let res = super::add_entry(&initial, "0.0.0.0").unwrap();
        debug!(target: "analytics:test", "initial response: \n----------\n{:#?}\n-----------", res);
        let exit = super::ExitingInfo {
            visit: res.visit,
            time: 10000,
            link_clicked: None,
        };
        super::update_entry(&exit).unwrap();
    }

    #[test]
    fn unknown_cookie() {
        let unknown_cookie = Uuid::new_v4();
        let landing = super::LandingInfo {
            referrer: Some("http://reddit.com/r/rust".into()),
            page: "http://wiredforge.com/blog/getpid/index.html".into(),
            cookie: Some(unknown_cookie),
            when: super::super::chrono::Utc::now(),
        };
        debug!(target: "analytics:test", "initial request: \n-----------\n{:?}\n----------", landing);
        let res = super::add_entry(&landing, "1.1.1.1").unwrap();
        debug!(target: "analytics:test", "result: {:?}", res);
        assert_ne!(unknown_cookie, res.token);
    }
}