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
use reports::Table;

lazy_static !{
    static ref CONFIG: DbConfig = from_str(include_str!("../dbinfo.toml")).expect("Unable to parse dbinfo.toml");
}

pub(crate) fn add_entry(info: &LandingInfo, ip: &str, user_agent: &str) -> Result<InitialResponse, Error> {
    debug!("add_entry {:#?},\n{}, {}", info, ip, user_agent);
    let conn = get_connection()?;
    let rows = conn.query("SELECT token, visit FROM add_session($1, $2, $3, $4, $5, $6, $7, $8)", &[&info.cookie, &ip, &info.referrer, &info.page, &info.when, &info.prev_visit, &info.site, &user_agent])?;
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

pub(crate) fn reports() -> Result<Vec<Table>, Error> {
    let conn = get_connection()?;
    let mut ref_table = Table::new(vec![
        "Referrer".to_string(),
        "Count".to_string(),
    ]);
    conn.query("SELECT * FROM referrers_this_week()",
                &[])?
        .iter()
        .for_each(|r|{
            let referrer: String = r.get(0);
            let ct: i64 = r.get(1);
            ref_table.rows.push(vec![referrer, ct.to_string()]);
        });
    let mut visits = Table::new(vec![
        "Visit Count".to_string(),
    ]);
    
    conn.query("SELECT * FROM unique_visits_this_week()", &[])?
        .iter()
        .for_each(|r| {
            let visit_count: i64 = r.get(0);
            visits.rows.push(vec![
                visit_count.to_string(),
            ]);
        });
    let mut views = Table::new(vec![
        "Page".to_string(),
        "View Count".to_string(),    
    ]);
    
    conn.query("SELECT * FROM unique_page_view_this_week()", &[])?
        .iter()
        .for_each(|r| {
            let view_count: i64 = r.get(0);
            let page: String = r.get(1);
            views.rows.push(vec![
                view_count.to_string(),
                page,
            ]);
        });
    Ok(vec![ref_table, visits, views])
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
            prev_visit: None,
            site: Some("wiredforge.com".into())
        };
        debug!(target: "analytics:test", "initial request: \n-----------\n{:?}\n----------", initial);
        let res = super::add_entry(&initial, "0.0.0.0", "I'm a teapot").unwrap();
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
            prev_visit: None,
            site: Some("http://wiredforge.com".into()),
        };
        debug!(target: "analytics:test", "initial request: \n-----------\n{:?}\n----------", landing);
        let res = super::add_entry(&landing, "1.1.1.1", "I'm a teapot").unwrap();
        debug!(target: "analytics:test", "result: {:?}", res);
        assert_ne!(unknown_cookie, res.token);
    }
}