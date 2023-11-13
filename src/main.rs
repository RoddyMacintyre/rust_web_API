#[macro_use] extern crate rocket;

use rocket::serde::json::{Value, json};

#[get("/")]     // Get attribute
fn hello() -> Value{
    json!("Hello, world!")
}

#[rocket::main]     // Rocket main function
async fn main() {
    let _ = rocket::build()             // Build the rocket framework
        .mount("/", routes![hello])     // Mount the routes to the build
        .launch()
        .await;
}
