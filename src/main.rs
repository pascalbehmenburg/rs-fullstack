use rs_fullstack::run;
use std::net::TcpListener;

// TODO implement user
//  TODO get /v1/users get current user
//  TODO patch /v1/users patch provided user data
//  TODO post /v1/users/login log in user
//  TODO post /v1/users/register creates user

// TODO implement todo

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let address = TcpListener::bind("127.0.0.1:8000")?;
    run(address)?.await
}
