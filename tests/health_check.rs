use rs_fullstack::{get_config, run};
use rstest::{fixture, rstest};
use sqlx::{Connection, PgConnection};
use std::future::Future;
use std::net::TcpListener;

/// Used to spawn a backend app and receive its address for each
/// test without having to explicitly use an async context.
#[fixture]
fn backend_address() -> String {
    // port 0 will be resolved by the os which will return a somewhat random port which is not in use.
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");

    let port = listener
        .local_addr()
        .expect("Could not receive local address from listener.")
        .port();

    let server = run(listener).expect("Failed to bind address");

    // future starts immediately after spawning when not applying await
    std::mem::drop(tokio::spawn(server));

    format!("http://127.0.0.1:{}", port)
}

#[fixture]
async fn postgres_connection() -> PgConnection {
    PgConnection::connect(
        &get_config()
            .expect("Failed to read config.")
            .database
            .connection_string(),
    )
    .await
    .expect("Failed to connect to Postgres")
}

#[rstest]
#[tokio::test]
async fn health_check_works(backend_address: String) {
    let response = reqwest::Client::new()
        .get(&format!("{}/health_check", &backend_address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[rstest]
#[awt]
#[tokio::test]
async fn v1_users_register_is_200_for_valid_data<T: Future<Output = PgConnection>>(
    backend_address: String,
    mut postgres_connection: T,
) {
    let response = reqwest::Client::new()
        .post(&format!("{}/users/register", &backend_address))
        .header("Content-Type", "application/json")
        .body(
            "{\"data\": {\"name\": \"test\", \"email\": \"test@test.test\", \"password\": \"test\"}}",
        )
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(200 == response.status().as_u16());

    let saved = sqlx::query!("SELECT name, email FROM users")
        .fetch_one(&mut postgres_connection.await)
        .await
        .expect("Failed to fetch saved subscription");

    assert!(saved.name == "test");
    assert!(saved.email == "test@test.test");
}

#[rstest]
#[case::missing_email_and_password("{\"data\": {\"name\": \"test\"}}")]
#[case::missing_name_and_password("{\"data\": {\"email\": \"test%40test.test\"}}")]
#[case::missing_name_and_email("{\"data\": {\"password\": \"test\"}}")]
#[case::missing_password("{\"data\": {\"name\": \"test_user\", \"email\": \"test%40test.test\"}}")]
#[case::missing_email("{\"data\": {\"name\": \"test_user\", \"password\": \"test\"}}")]
#[case::missing_name("{\"data\": {\"password\": \"test\", \"email\": \"test%40test.test\"}}")]
#[case::missing_all_fields("")]
#[tokio::test]
async fn v1_users_register_is_400_for_missing_data(
    backend_address: String,
    #[case] invalid_body: String,
) {
    let response = reqwest::Client::new()
        .post(&format!("{}/users/register", &backend_address))
        .header("Content-Type", "application/json")
        .body(invalid_body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(400, response.status().as_u16());
}
