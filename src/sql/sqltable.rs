use rusqlite::{Connection, Result};

#[allow(unused)]
pub fn print_table_columns() {
    let db_path = "lottoSql.db";
    let conn = init_db(db_path).expect("Failed to initialize database");
    let mut stmt = conn
        .prepare("SELECT name FROM sqlite_master WHERE type='table'")
        .expect("Failed to prepare statement");
    let table_iter = stmt
        .query_map([], |row| row.get::<_, String>(0))
        .expect("Failed to query tables");

    for table_name in table_iter {
        let table_name = table_name.expect("Failed to get table name");
        println!("Table: {}", table_name);
        let pragma_sql = format!("PRAGMA table_info('{}')", table_name);
        let mut col_stmt = conn.prepare(&pragma_sql).expect("Failed to prepare pragma");
        let col_iter = col_stmt
            .query_map([], |row| row.get::<_, String>(1))
            .expect("Failed to query columns");
        print!("  Columns: ");
        for col in col_iter {
            print!("{} ", col.expect("Failed to get column name"));
        }
        println!("");
    }
}
#[allow(unused)]
pub fn print_qihao_all(qihao: u32) {
    let db_path = "lottoSql.db";
    let conn = init_db(db_path).expect("Failed to initialize database");
    let mut stmt = conn
        .prepare("SELECT number FROM numbers WHERE qihao_id = ?1")
        .expect("Failed to prepare statement");
    let rows = stmt
        .query_map([qihao], |row| row.get::<_, String>(0))
        .expect("Failed to query rows");
    println!("大乐透第 {} 期 ", qihao);
    for row in rows {
        let number: String = row.expect("Failed to get row");
        println!(" Number: {}", number);
    }
}
pub fn print_all() {
    //输出数据库表中的所有数据
    let db_path = "lottoSql.db";
    let conn = init_db(db_path).expect("Failed to initialize database");
    let mut stmt = conn
        .prepare("SELECT * FROM numbers")
        .expect("Failed to prepare statement");
    let rows = stmt
        .query_map([], |row| {
            let qihao_id: u32 = row.get(0)?;
            let number: String = row.get(1)?;
            Ok((qihao_id, number))
        })
        .expect("Failed to query rows");

    for row in rows {
        let (qihao_id, number) = row.expect("Failed to get row");
        println!("大乐透第 {} 期: Number: {}", qihao_id, number);
    }
}
pub fn init_db(db_path: &str) -> Result<Connection> {
    let conn = Connection::open(db_path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS numbers (
            qihao_id INTEGER NOT NULL,
            number TEXT NOT NULL
        )",
        [],
    )?;
    Ok(conn)
}

#[cfg(test)]
mod create_sql {
    use crate::sql::sqltable::init_db;

    #[test]
    fn create_sql() {
        let db_path = "lottoSql.db";
        let _ = init_db(db_path).expect("Failed to initialize database");
    }

    #[test]
    fn check_sql_table() {
        let db_path = "lottoSql.db";
        let conn = init_db(db_path).expect("Failed to initialize database");
        let mut stmt = conn
            .prepare("SELECT name FROM sqlite_master WHERE type='table'")
            .expect("Failed to prepare statement");
        let table_iter = stmt
            .query_map([], |row| row.get::<_, String>(0))
            .expect("Failed to query tables");

        for table_name in table_iter {
            println!("Table: {}", table_name.expect("Failed to get table name"));
        }
    }

    #[test]
    fn empty_all() {
        //清空数据库表中的所有数据
        let db_path = "lottoSql.db";
        let conn = init_db(db_path).expect("Failed to initialize database");
        conn.execute("DELETE FROM numbers", [])
            .expect("Failed to delete rows");
    }
}
