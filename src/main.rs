#[macro_use] extern crate rocket;

#[get("/")]     // Get attribute
fn hello() -> &'static str{
    "Hello, world!"
}

#[rocket::main]     // Rocket main function
async fn main() {
    let _ = rocket::build()             // Build the rocket framework
        .mount("/", routes![hello])     // Mount the routes to the build
        .launch()
        .await;
}
