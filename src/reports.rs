use term_table::{Table as TTable, TableStyle, table_cell::{TableCell, Alignment}, row::Row};
use colored::*;
static TEMPLATE: &str = include_str!("report.html");
static TABLE_STYLE: &str = r#"border:1px solid black;border-collapse: collapse;margin-bottom: 10px;"#;
static HEADER_STYLE: &str = r#"border:1px solid black;font-weight:bold;"#;
static CELL_STYLE: &str = r#"border:1px solid black;"#;

#[derive(Serialize)]
pub struct Table {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
    pub name: String,
}

impl Table {
    pub fn new(name: String, headers: Vec<String>) -> Self {
        Self {
            name,
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

pub fn generate_ascii_report(tables: &Vec<Table>) -> String{
    let mut ret = String::new();
    for table in tables {
        let columns = table.headers.len();
        let mut t = TTable::new();
        debug!("generating {} with width {}", table.name, columns);
        t.style = TableStyle::extended();
        t.add_row(Row::new(vec![
            TableCell::new_with_alignment(&table.name.bold().blue(), columns, Alignment::Center),
        ]));
        t.add_row(Row::new(
            table.headers.iter().map(|h| TableCell::new_with_alignment(&h.bold(), 1, Alignment::Center))
        ));

        for row in table.rows.iter() {
            t.add_row(Row::new(
                row.iter().map(|r| TableCell::new(&r))
            ));
        }
        ret.push_str(&t.render());
        ret.push_str("\n\n");
    }
    ret
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn tables() {
        let mut table = Table::new("TEST ONE".to_string(), vec!["one".to_string(), "two".to_string()]);
        table.rows.push(vec!["one".to_string(), "two".to_string()]);
        let tables = vec![
            table
        ];
        println!("{}", generate_report(tables).unwrap());
    }

    #[test]
    fn ascii() {
        let mut table = Table::new("ASCII TEST".to_string(), vec!["one hundred".to_string(), "two".to_string()]);
        table.rows.push(vec!["one".to_string(), "two thousand".to_string()]);
        let tables = vec![
            table
        ];
        println!("{}", generate_ascii_report(&tables).unwrap())
    }

    #[test]
    fn ascii_uneven() {
        let mut table = Table::new("UNEVEN".to_string(), vec!["one hundred two".to_string(), "two ad".to_string()]);
        table.rows.push(vec!["one".to_string(), "two".to_string()]);
        let tables = vec![
            table
        ];
        println!("{}", generate_ascii_report(&tables).unwrap());
    }
}