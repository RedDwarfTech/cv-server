extern crate openssl;
#[macro_use]
extern crate diesel;

use entry::server::create_server;

mod entry;
mod common;
mod biz;
mod model;
mod service;

#[rocket::main]
async fn main() {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    let launch_result = create_server().launch().await;
    match launch_result {
        Ok(_) => println!("Rocket shut down gracefully."),
        Err(err) => println!("Rocket had an error: {}", err),
    };
}