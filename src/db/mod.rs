use difficulties::Difficulties;
use exercises::Exercises;
use users::Users;

pub mod difficulties;
pub mod exercises;
pub mod stats;
pub mod users;

#[derive(Clone)]
pub struct Db {
    pool: sqlx::PgPool,
    pub users: Users,
    pub exercises: Exercises,
    pub stats: stats::Stats,
    pub difficulties: Difficulties,
}

impl Db {
    pub async fn new(pool: sqlx::PgPool) -> Result<Self, sqlx::Error> {
        let db = Self {
            users: Users::new(pool.clone()),
            exercises: Exercises::new(pool.clone()),
            stats: stats::Stats::new(pool.clone()),
            difficulties: Difficulties::new(pool.clone()),
            pool,
        };

        db.init().await?;
        Ok(db)
    }

    async fn init(&self) -> Result<(), sqlx::Error> {
        self.users.create_table().await?;
        self.difficulties.create_table().await?;
        self.exercises.create_table().await?;
        self.stats.create_table().await?;
        Ok(())
    }
}
