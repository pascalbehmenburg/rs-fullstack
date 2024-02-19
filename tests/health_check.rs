use rs_fullstack::run;
use std::net::TcpListener;

/// Used to spawn a backend app for each test without having to explicitly use an async context.
fn spawn_backend() -> String {
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

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let address = spawn_backend();
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

//  TODO post /v1/users/register creates user
#[tokio::test]
async fn v1_users_register_is_200_for_valid_data() {
    // Arrange
    let app_address = spawn_backend();
    let client = reqwest::Client::new();

    // Act
    let body =
        "{data: {\"name\": \"test\", \"password\": \"test\", \"email\": \"test%40test.test\"}}";
    let response = client
        .post(&format!("{}/users/register", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn v1_users_register_is_400_for_invalid_data() {
    // Arrange
    let app_address = spawn_backend();
    let client = reqwest::Client::new();
    // TODO think about implementing a json serializer / deserializer
    // which automatically wraps object in data json object
    let test_cases = vec![
        (
            "{data: {\"name\": \"test_user\"}}",
            "missing the email and password",
        ),
        (
            "{data: {\"email\": \"test%40test.test\"}}",
            "missing the name and password",
        ),
        (
            "{data: {\"password\": \"test\"}}",
            "missing the name and email",
        ),
        (
            "{data: {\"name: \"test_user\", \"email\": \"test%40test.test\"}}",
            "missing the password",
        ),
        (
            "{data: {\"name\": \"test_user\", \"password\": \"test\"}}",
            "missing the email",
        ),
        (
            "{data: {\"password\": \"test\", \"email\": \"test%40test.test\"}}",
            "missing the name",
        ),
        ("", "missing the email, name and password"),
    ];

    for (invalid_body, error_message) in test_cases {
        // Act
        let response = client
            .post(&format!("{}/users/register", &app_address))
            .header("Content-Type", "application/json")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            // Additional customised error message on test failure
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}
