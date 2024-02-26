use rs_fullstack::{get_config, run};
use rstest::{fixture, rstest};
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;

struct BackendTestData {
    address: String,
    pg_pool: PgPool,
}

/// Used to spawn a backend app and receive its address for each
/// test without having to explicitly use an async context.
#[fixture]
async fn backend() -> BackendTestData {
    // port 0 will be resolved by the os which will return a somewhat random port which is not in use.
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port.");

    let port = listener
        .local_addr()
        .expect("Could not receive local address from listener.")
        .port();

    let mut config = get_config().expect("Failed to read configuration.");

    // create random database for each test and create a connection pool to it
    let pg_pool = {
        config.database.database_name = Uuid::new_v4().to_string();

        let mut connection = PgConnection::connect(&config.database.connection_string_without_db())
            .await
            .expect("Failed to connect to Postgres");

        connection
            .execute(format!(r#"CREATE DATABASE "{}";"#, config.database.database_name).as_str())
            .await
            .expect("Failed to create database.");

        let pg_pool = PgPool::connect(&config.database.connection_string())
            .await
            .expect("Failed to connect to Postgres");

        sqlx::migrate!("./migrations")
            .run(&pg_pool)
            .await
            .expect("Failed to migrate the database.");

        pg_pool
    };

    let server = run(listener, pg_pool.clone()).expect("Failed to bind address.");

    // mem-dropping a spawned thread will execute it's future without propagating the result
    std::mem::drop(tokio::spawn(server));

    BackendTestData {
        address: format!("http://127.0.0.1:{}", port),
        pg_pool: pg_pool,
    }
}

#[rstest]
#[awt]
#[tokio::test]
async fn health_check_works(#[future] backend: BackendTestData) {
    let response = reqwest::Client::new()
        .get(&format!("{}/health_check", &backend.address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[rstest]
#[awt]
#[tokio::test]
async fn v1_users_register_is_200_for_valid_data(#[future] backend: BackendTestData) {
    let response = reqwest::Client::new()
        .post(&format!("{}/users/register", &backend.address))
        .header("Content-Type", "application/json")
        .body(r#"{"data": {"name": "test", "email": "test@test.test", "password": "test"}}"#)
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(200 == response.status().as_u16());

    let saved = sqlx::query!("SELECT name, email FROM users")
        .fetch_one(&backend.pg_pool)
        .await
        .expect("Failed to fetch saved subscription");

    assert!(saved.name == "test");
    assert!(saved.email == "test@test.test");
}

#[rstest]
#[case::missing_email_and_password(r#"{"data": {"name": "test"}}"#)]
#[case::missing_name_and_password(r#"{"data": {"email": "test@test.test"}}"#)]
#[case::missing_name_and_email(r#"{"data": {"password": "test"}}"#)]
#[case::missing_password(r#"{"data": {"name": "test_user", "email": "test@test.test"}}"#)]
#[case::missing_email(r#"{"data": {"name": "test_user", "password": "test"}}"#)]
#[case::missing_name(r#"{"data": {"email": "test@test.test", "password": "test", }}"#)]
#[case::missing_all_fields("")]
#[awt]
#[tokio::test]
async fn v1_users_register_is_400_for_missing_data(
    #[future] backend: BackendTestData,
    #[case] invalid_body: String,
) {
    let response = reqwest::Client::new()
        .post(&format!("{}/users/register", &backend.address))
        .header("Content-Type", "application/json")
        .body(invalid_body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(400, response.status().as_u16());
}
