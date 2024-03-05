use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Type};

use super::{auth::UserCtx, Error};

#[derive(Serialize, Deserialize, Type, Debug)]
#[sqlx(type_name = "job_status", rename_all = "lowercase")]
pub enum JobApplicationStatus {
    Pending,
    Accepted,
    Rejected,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Job {
    pub id: i64,
    pub title: String,
    pub company: String,
    pub application_link: String,
    pub ctime: NaiveDateTime,
    pub user_id: String,
    pub status: JobApplicationStatus,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JobApplicationPatch {
    pub title: Option<String>,
    pub application_link: Option<String>,
    pub company: Option<String>,
    pub status: Option<String>,
}

impl JobApplicationPatch {
    fn is_empty(&self) -> bool {
        self.title.is_none() && self.application_link.is_none() && self.company.is_none()
    }
}

pub struct JobAppManager;

impl JobAppManager {
    async fn create(
        db: &PgPool,
        data: JobApplicationPatch,
        user_ctx: UserCtx,
    ) -> Result<Job, Error> {
        if data.is_empty() {
            return Err(Error::MissingData);
        }

        let new_job = sqlx::query_as!(Job, r#"insert into jobs (title, company, application_link, ctime, user_id, status) values ($1, $2, $3, $4, $5, $6) returning id, title, company, application_link, ctime, user_id, status AS "status!: JobApplicationStatus""#, data.title.unwrap(), data.company.unwrap(), data.application_link.unwrap(), Utc::now().naive_utc(), user_ctx.user_id, JobApplicationStatus::Pending as JobApplicationStatus).fetch_one(db).await?;
        Ok(new_job)
    }

    async fn list(db: &PgPool, user_ctx: UserCtx) -> Result<Vec<Job>, Error> {
        let jobs = sqlx::query_as!(
            Job,
            r#"select id, title, company, application_link, ctime, user_id, status AS "status!: JobApplicationStatus" from jobs where user_id = $1 ORDER BY ctime DESC"#,
            user_ctx.user_id
        )
        .fetch_all(db)
        .await?;
        Ok(jobs)
    }

    async fn update_job_status(
        db: &PgPool,
        job_status: &str,
        user_ctx: UserCtx,
        job_id: i64,
    ) -> Result<Job, Error> {
        let updated_job = sqlx::query_as!(Job, r#"UPDATE jobs SET status = $1 where id = $2 returning id, title, company, application_link, ctime, user_id, status AS "status!: JobApplicationStatus""#, parse_job_status(job_status) as JobApplicationStatus, job_id).fetch_one(db).await?;
        Ok(updated_job)
    }

    async fn update(
        db: &PgPool,
        utx: UserCtx,
        data: JobApplicationPatch,
        job_id: i64,
    ) -> Result<Job, Error> {
        println!("1");
        let job_to_update = sqlx::query_as!(Job, r#"select id, title, company, application_link, ctime, user_id, status AS "status!: JobApplicationStatus" from jobs where id = $1"#, job_id).fetch_one(db).await?;

        if job_to_update.user_id != utx.user_id {
            return Err(Error::NotAuthorized);
        }

        let job_status = match data.status {
            Some(status) => parse_job_status(&status),
            None => JobApplicationStatus::Pending,
        };
        let updated_job = sqlx::query_as!(Job, r#"UPDATE jobs SET title = $1, company = $2, application_link = $3, status = $4 where id = $5 returning id, title, company, application_link, ctime, user_id, status AS "status!: JobApplicationStatus""#, data.title.unwrap_or_else(|| job_to_update.title), data.company.unwrap_or_else(|| job_to_update.company), data.application_link.unwrap_or_else(|| job_to_update.application_link), job_status as JobApplicationStatus, job_id).fetch_one(db).await?;

        Ok(updated_job)
    }

    async fn delete(db: &PgPool, utx: UserCtx, job_id: i64) -> Result<(), Error> {
        let job_to_delete = sqlx::query_as!(Job, r#"select id, title, company, application_link, ctime, user_id, status AS "status!: JobApplicationStatus" from jobs where id = $1"#, job_id).fetch_one(db).await?;

        if job_to_delete.user_id != utx.user_id {
            return Err(Error::NotAuthorized);
        }

        sqlx::query!(r#"DELETE FROM jobs WHERE id = $1"#, job_id)
            .execute(db)
            .await?;
        Ok(())
    }
}
pub fn parse_job_status(status: &str) -> JobApplicationStatus {
    match status {
        "pending" => JobApplicationStatus::Pending,
        "accepted" => JobApplicationStatus::Accepted,
        "rejected" => JobApplicationStatus::Rejected,
        _ => JobApplicationStatus::Pending,
    }
}

#[cfg(test)]
#[path = "../_tests/job_manager.rs"]
mod tests;
