use crate::models::auth::UserCtx;
use crate::models::db::connect_to_dev_db;
use crate::models::users::*;

#[tokio::test]
async fn setup() -> Result<(), Box<dyn std::error::Error>> {
    let pool = connect_to_dev_db().await?;
    sqlx::migrate!("./migrations/dev").run(&pool).await?;

    Ok(())
}

#[tokio::test]
async fn test_create_user() -> Result<(), Box<dyn std::error::Error>> {
    let db = connect_to_dev_db().await?;

    let utx = UserCtx {
        user_id: String::from("test_123"),
    };

    let data = UserPatch {
        first_name: Some(String::from("test_first_name")),
        last_name: Some(String::from("test_last_name")),
        email: Some(String::from("test_create_email")),
    };

    let user = UserManager::create(&db, utx, data).await?;

    assert_eq!("test_first_name", user.first_name);
    assert_eq!("test_last_name", user.last_name);
    assert_eq!("test_create_email", user.email);

    Ok(())
}

#[tokio::test]
async fn test_update_user() -> Result<(), Box<dyn std::error::Error>> {
    let db = connect_to_dev_db().await?;
    let utx = UserCtx {
        user_id: String::from("test_token"),
    };
    let data = UserPatch {
        first_name: Some(String::from("test_first_name_udated")),
        last_name: Some(String::from("test_last_name_updated")),
        email: Some(String::from("test_email_updated")),
    };

    let user = UserManager::update(&db, utx, data).await?;
    assert_eq!("test_first_name_udated", user.first_name);
    assert_eq!("test_last_name_updated", user.last_name);
    assert_eq!("test_email_updated", user.email);

    Ok(())
}

#[tokio::test]
async fn test_delete_user() -> Result<(), Box<dyn std::error::Error>> {
    let db = connect_to_dev_db().await?;
    let utx = UserCtx {
        user_id: String::from("test_delete"),
    };

    UserManager::delete(&db, utx).await?;

    Ok(())
}
