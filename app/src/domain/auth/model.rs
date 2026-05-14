use serde::{Deserialize, Serialize};
#[derive(Clone, Debug,Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]

pub struct User {
    pub unid: uuid::Uuid,
    pub email: String,
    pub username: String,
    pub password_hash: String,
    pub is_active: bool,
    pub role: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub created_at: time::OffsetDateTime,
    pub updated_at: time::OffsetDateTime,
    pub last_login_at: Option<time::OffsetDateTime>,
    pub last_password_change_at: Option<time::OffsetDateTime>,
    pub failed_login_attempts: i32,
    pub locked_until: Option<time::OffsetDateTime>,
}


#[cfg(feature = "ssr")]
#[async_trait::async_trait]
impl axum_session_auth::Authentication<User, uuid::Uuid, sqlx::PgPool> for User {
    async fn load_user(
        userid: uuid::Uuid,
        pool: Option<&sqlx::PgPool>,
    ) -> Result<User, anyhow::Error> {
        let pool = pool.ok_or_else(|| anyhow::anyhow!("No pool provided"))?;

        let user = sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE unid = $1",
            userid
        )
        .fetch_one(pool)
        .await?;

        Ok(user)
    }

    fn is_authenticated(&self) -> bool {
        true
    }

    fn is_active(&self) -> bool {
        self.is_active
    }

    fn is_anonymous(&self) -> bool {
        false
    }
}
