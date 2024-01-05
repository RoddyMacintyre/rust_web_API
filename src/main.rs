mod auth;
use auth::BasicAuth;

#[macro_use] extern crate rocket;

use rocket::request::{FromRequest};
use rocket::response::status;
use rocket::serde::json::{Value, json};



#[get("/")]     // Get attribute
fn hello() -> Value{
    json!("Hello, world!")
}

#[get("/exchanges")]
fn get_exchanges(_auth: BasicAuth) -> Value{     // Implement BasicAuth for this endpoint
    json!([{"id": 1, "name": "binance"}, {"id": 2, "name": "kucoin"}])
}

#[get("/exchanges/<id>")]
fn view_exchange(id: i32, _auth: BasicAuth) -> Value{
    json!([{"id": id, "name": "binance", "url": "https://www.binance.com"}])
}

#[post("/exchanges", format = "json")]      // application-json
fn create_exchange(_auth: BasicAuth) -> Value{
    json!([{"id": 3, "name": "gate", "url": "https://www.gate.io"}])
}

#[put("/exchanges/<id>", format = "json")]
fn update_exchange(id: i32, _auth: BasicAuth) -> Value{
    json!([{"id": id, "name": "updated", "url": "https://updated_url"}])
}

#[delete("/exchanges/<_id>")]
fn delete_exchange(_id: i32, _auth: BasicAuth) -> status::NoContent{
    status::NoContent
}

#[catch(404)]   // Rocket error catcher for status code 404
fn not_found() -> Value{
    json!("Not found!")
}

// Error catcher for unauthorized
#[catch(401)]
fn unauthorized() -> Value{
    json!("Invalid/Missing authorization")
}

#[rocket::main]     // Rocket main function
async fn main() {
    let _ = rocket::build()             // Build the rocket framework
        .mount("/", routes![
            hello,
            get_exchanges,
            view_exchange,
            create_exchange,
            update_exchange,
            delete_exchange
        ])     // Mount the routes to the build
        .register("/", catchers![   // Register the catchers to the mounted endpoints
            not_found,
            unauthorized    // Register the 401 unauthorized func
        ])
        .launch()
        .await;
}
