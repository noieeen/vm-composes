use crate::models::user::User;
use sqlx::{PgPool, postgres::PgPoolOptions};
use uuid::Uuid;

#[derive(Clone)]
pub struct Db(pub PgPool);

impl Db {
    pub async fn new() -> Result<Self, sqlx::Error> {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let mut retries = 5;
        let mut delay = std::time::Duration::from_secs(2);

        loop {
            match PgPoolOptions::new()
                .max_connections(10)
                .connect(&database_url)
                .await
            {
                Ok(pool) => return Ok(Db(pool)),
                Err(e) if retries > 0 => {
                    eprintln!(
                        "Failed to connect to Postgres: {}. Retrying in {}s...",
                        e,
                        delay.as_secs()
                    );
                    tokio::time::sleep(delay).await;
                    retries -= 1;
                    delay *= 2;
                }
                Err(e) => return Err(e),
            }
        }
    }

    pub async fn create_user(&self, email: &str, password: &str) -> Result<User, sqlx::Error> {
        let id = Uuid::new_v4();
        let rec = sqlx::query_as::<_, User>(
            "INSERT INTO users (id, email, password) VALUES ($1, $2, $3) RETURNING *"
        )
        .bind(id)
        .bind(email)
        .bind(password)
        .fetch_one(&self.0)
        .await?;

        Ok(rec)
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
            .bind(email)
            .fetch_one(&self.0)
            .await
    }
}
