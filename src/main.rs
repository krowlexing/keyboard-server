use db::Db;
use handlers::router;

pub mod db;
pub mod dto;
pub mod extractors;
pub mod handlers;
pub mod util;

#[tokio::main]
async fn main() {
    let pool = sqlx::PgPool::connect("postgres://postgres:postgres@localhost:5555/keyboard")
        .await
        .unwrap();

    println!("connected to db");

    let db = Db::new(pool).await.unwrap();

    tracing_subscriber::fmt::init();

    let app = router(db);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}
