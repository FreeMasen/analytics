
static TEMPLATE: &str = include_str!("report.html");
static TABLE_STYLE: &str = r#"border:1px solid black;border-collapse: collapse;margin-bottom: 10px;"#;
static HEADER_STYLE: &str = r#"border:1px solid black;font-weight:bold;"#;
static CELL_STYLE: &str = r#"border:1px solid black;"#;
#[derive(Serialize)]
pub struct Table {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
}

impl Table {
    pub fn new(headers: Vec<String>) -> Self {
        Self {
            headers,
            rows: Vec::new(),
        }
    }
}


pub fn generate_report(tables: Vec<Table>) -> Result<String, String> {
    use tera::{Tera, Context};
    let mut ctx = Context::new();
    ctx.insert("tables", &tables);
    ctx.insert("table_style", TABLE_STYLE);
    ctx.insert("header_style", HEADER_STYLE);
    ctx.insert("cell_style", CELL_STYLE);
    Tera::one_off(TEMPLATE, &ctx, true).map_err(|e| format!("{:?}", e))
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn tables() {
        let mut table = Table::new(vec!["one".to_string(), "two".to_string()]);
        table.rows.push(vec!["one".to_string(), "two".to_string()]);
        let tables = vec![
            table
        ];
        println!("{}", generate_report(tables).unwrap());
    }
}