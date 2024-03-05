use crate::models::auth::UserCtx;
use crate::models::db::connect_to_dev_db;
use crate::models::jobs::*;

#[tokio::test]
async fn setup() -> Result<(), Box<dyn std::error::Error>> {
    let pool = connect_to_dev_db().await?;
    sqlx::query!("DROP TABLE IF EXISTS _sqlx_migrations")
        .execute(&pool)
        .await?;
    sqlx::query!("DROP TABLE IF EXISTS jobs")
        .execute(&pool)
        .await?;
    // sqlx::query!("DROP TABLE IF EXISTS users")
    //     .execute(&pool)
    //     .await?;
    sqlx::migrate!("./migrations/dev").run(&pool).await?;

    Ok(())
}

#[tokio::test]
async fn test_create_job() -> Result<(), Box<dyn std::error::Error>> {
    let db = connect_to_dev_db().await?;

    let utx = UserCtx {
        user_id: String::from("test_token"),
    };

    let data = JobApplicationPatch {
        title: Some(String::from("test_title")),
        company: Some(String::from("test_company")),
        application_link: Some(String::from("test_link")),
        status: None,
    };

    let job = JobAppManager::create(&db, data, utx).await?;
    assert_eq!("test_title", job.title);
    assert_eq!("test_company", job.company);
    assert_eq!("test_link", job.application_link);
    assert_eq!("test_token", job.user_id);

    Ok(())
}

#[tokio::test]
async fn test_update_job() -> Result<(), Box<dyn std::error::Error>> {
    let db = connect_to_dev_db().await?;
    let utx = UserCtx {
        user_id: String::from("test_token"),
    };
    let data = JobApplicationPatch {
        title: Some(String::from("test_title_2")),
        company: Some(String::from("test_company_2")),
        application_link: Some(String::from("test_link_2")),
        status: None,
    };

    let id: i64 = 1;

    let job = JobAppManager::update(&db, utx, data, id).await?;
    assert_eq!("test_title_2", job.title);
    assert_eq!("test_company_2", job.company);
    assert_eq!("test_link_2", job.application_link);

    Ok(())
}

#[tokio::test]
async fn test_list_jobs() -> Result<(), Box<dyn std::error::Error>> {
    let db = connect_to_dev_db().await?;
    let utx = UserCtx {
        user_id: String::from("test_token"),
    };

    let jobs = JobAppManager::list(&db, utx).await?;

    assert!(jobs.len() > 0);

    Ok(())
}

#[tokio::test]
async fn test_delete_job() -> Result<(), Box<dyn std::error::Error>> {
    let db = connect_to_dev_db().await?;
    let utx = UserCtx {
        user_id: String::from("test_token"),
    };
    let id: i64 = 2;
    JobAppManager::delete(&db, utx.clone(), id).await?;

    let remaining_jobs = JobAppManager::list(&db, utx).await?;
    assert_ne!(remaining_jobs[1].id, id);

    Ok(())
}
