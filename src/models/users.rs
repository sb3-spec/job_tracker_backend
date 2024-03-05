use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use super::{auth::UserCtx, Error};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub supabase_id: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserPatch {
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

impl UserPatch {
    fn is_empty(&self) -> bool {
        self.email.is_none() && self.first_name.is_none() && self.last_name.is_none()
    }
}

pub struct UserManager;

impl UserManager {
    async fn create(db: &PgPool, utx: UserCtx, data: UserPatch) -> Result<User, Error> {
        if data.is_empty() {
            return Err(Error::MissingData);
        }
        if let Some(email) = &data.email {
            if let Some(_existing_user) =
                sqlx::query_as!(User, r#"select * from users where email = $1"#, email)
                    .fetch_optional(db)
                    .await?
            {
                return Err(Error::EmailTaken(email.to_string()));
            }
        };

        let new_user = sqlx::query_as!(User, r#"insert into users (supabase_id, email, first_name, last_name) values ($1, $2, $3, $4) returning supabase_id, email, first_name, last_name"#, utx.user_id, data.email.unwrap_or_default(), data.first_name.unwrap_or_default(), data.last_name.unwrap_or_default()).fetch_one(db).await?;
        Ok(new_user)
    }

    async fn update(db: &PgPool, utx: UserCtx, data: UserPatch) -> Result<User, Error> {
        let user_to_update = sqlx::query_as!(
            User,
            r#"select * from users where supabase_id = $1"#,
            utx.user_id
        )
        .fetch_one(db)
        .await?;

        let email = data.email.unwrap_or_else(|| user_to_update.email);
        let first_name = data.first_name.unwrap_or_else(|| user_to_update.first_name);
        let last_name = data.last_name.unwrap_or_else(|| user_to_update.last_name);
        let updated_user = sqlx::query_as!(User, r#"update users set email = $1, first_name = $2, last_name = $3 where supabase_id = $4 returning *"#, email, first_name, last_name, utx.user_id).fetch_one(db).await?;

        Ok(updated_user)
    }

    async fn update_email(db: &PgPool, utx: UserCtx, email: String) -> Result<User, Error> {
        let updated_user = sqlx::query_as!(
            User,
            r#"UPDATE users SET email = $1 where supabase_id = $2 returning *"#,
            email,
            utx.user_id
        )
        .fetch_one(db)
        .await?;
        Ok(updated_user)
    }

    async fn update_first_name(
        db: &PgPool,
        utx: UserCtx,
        first_name: String,
    ) -> Result<User, Error> {
        let updated_user = sqlx::query_as!(
            User,
            r#"UPDATE users SET first_name = $1 where supabase_id = $2 returning *"#,
            first_name,
            utx.user_id
        )
        .fetch_one(db)
        .await?;

        Ok(updated_user)
    }

    async fn update_last_name(db: &PgPool, utx: UserCtx, last_name: String) -> Result<User, Error> {
        let updated_user = sqlx::query_as!(
            User,
            r#"UPDATE users SET last_name = $1 where supabase_id = $2 returning *"#,
            last_name,
            utx.user_id
        )
        .fetch_one(db)
        .await?;

        Ok(updated_user)
    }

    async fn delete(db: &PgPool, utx: UserCtx) -> Result<User, Error> {
        let user_to_delete = sqlx::query_as!(
            User,
            r#"SELECT * from users where supabase_id = $1"#,
            utx.user_id
        )
        .fetch_one(db)
        .await?;

        sqlx::query!(r#"delete from users where supabase_id = $1"#, utx.user_id)
            .execute(db)
            .await?;

        Ok(user_to_delete)
    }
}

#[cfg(test)]
#[path = "../_tests/user_manager.rs"]
mod tests;
