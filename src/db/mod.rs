use sqlx::postgres::PgQueryResult;
use users::Users;

pub mod users;

#[derive(Clone)]
pub struct Db {
    pool: sqlx::PgPool,
    pub users: Users,
}

impl Db {
    pub async fn new(pool: sqlx::PgPool) -> Result<Self, sqlx::Error> {
        let db = Self {
            users: Users::new(pool.clone()),
            pool,
        };

        db.init().await?;
        Ok(db)
    }

    async fn init(&self) -> Result<(), sqlx::Error> {
        self.users.create_table().await?;
        Ok(())
    }
}
