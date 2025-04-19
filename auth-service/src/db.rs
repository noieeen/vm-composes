use sqlx::{PgPool, postgres::PgPoolOptions};
use crate::models::user::User;
use uuid::Uuid;

#[derive(Clone)]
pub struct Db(pub PgPool);

impl Db {
    pub async fn new() -> Self {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&std::env::var("DATABASE_URL").unwrap())
            .await
            .unwrap();
        Db(pool)
    }

    pub async fn create_user(&self, email: &str, password: &str) -> Result<User, sqlx::Error> {
        let id = Uuid::new_v4();
        let rec = sqlx::query_as!(
            User,
            "INSERT INTO users (id, email, password) VALUES ($1, $2, $3) RETURNING *",
            id,
            email,
            password
        )
        .fetch_one(&self.0)
        .await?;

        Ok(rec)
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<User, sqlx::Error> {
        sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1", email)
            .fetch_one(&self.0)
            .await
    }
}
