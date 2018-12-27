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
    let rows = conn.query("SELECT token, visit FROM add_session3($1, $2, $3, $4, $5, $6)", &[&info.cookie, &ip, &info.referrer, &info.page, &info.when, &info.prev_visit])?;
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

pub(crate) fn reports() -> Result<String, Error> {
    let conn = get_connection()?;
    let table_style = r#""border:1px solid black;border-collapse: collapse;margin-bottom: 10px;""#;
    let header_style = r#""border:1px solid black;font-weight:bold;""#;
    let cell_style = r#""border:1px solid black;""#;
    let refs_head = format!(r#"<table style={}><thead><tr><th style={head_style}>Referrer</th><th style={head_style}>Count</th></tr></thead><tbody>"#, table_style, head_style=header_style);
    let weekly_refs: String = conn.query("SELECT * FROM referrers_this_week()",
                &[])?
        .iter()
        .map(|r|{
            let referrer: String = r.get(0);
            let ct: i64 = r.get(1);
            format!("<tr><td style={cell_style}>{}</td><td style={cell_style}>{}</td></tr>", referrer, ct, cell_style=cell_style)
        })
        .collect();
    let foot = format!("</tbody></table>");
    let visits_head = format!("<table style={}><thead><tr><th style={head_style}>Visit Count</th></tr></thead><tbody>", table_style, head_style=header_style);
    let weekly_visits: String = conn.query("SELECT * FROM unique_visits_this_week()", &[])?
        .iter()
        .map(|r| {
            let visit_count: i64 = r.get(0);
            format!("<tr><td style={cell_style}>{}</td></tr>", visit_count, cell_style=cell_style)
        })
        .collect();
    let views_head = format!("<table style={}><thead><tr><th style={head_style}>Page</th><th style={head_style}>View Count</th></tr></thead><tbody>", table_style, head_style=header_style);
    let weekly_views: String = conn.query("SELECT * FROM unique_page_view_this_week()", &[])?
        .iter()
        .map(|r| {
            let view_count: i64 = r.get(0);
            let page: String = r.get(1);
            format!("<tr><td style={cell_style}>{}</td><td style={cell_style}>{}</td></tr>", page, view_count, cell_style=cell_style)
        })
        .collect();
    Ok(format!("<html><head></head><body>{refs_head}{weekly_refs}{foot}{visits_head}{visits}{foot}{views_head}{views}{foot}</body></html>",
                refs_head=refs_head,
                weekly_refs=weekly_refs,
                foot=foot,
                visits_head=visits_head,
                visits=weekly_visits,
                views_head=views_head,
                views=weekly_views
    ))
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
            prev_visit: None,
        };
        debug!(target: "analytics:test", "initial request: \n-----------\n{:?}\n----------", landing);
        let res = super::add_entry(&landing, "1.1.1.1").unwrap();
        debug!(target: "analytics:test", "result: {:?}", res);
        assert_ne!(unknown_cookie, res.token);
    }
}