pub fn report_sql_error(err: sqlx::Error, msg: &str) {
    println!("{msg}:{err}");
}
