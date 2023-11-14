#[macro_use] extern crate rocket;

use rocket::response::status;
use rocket::serde::json::{Value, json};

#[get("/")]     // Get attribute
fn hello() -> Value{
    json!("Hello, world!")
}

#[get("/exchanges")]
fn get_exchanges() -> Value{
    json!([{"id": 1, "name": "binance"}, {"id": 2, "name": "kucoin"}])
}

#[get("/exchanges/<id>")]
fn view_exchange(id: i32) -> Value{
    json!([{"id": id, "name": "binance", "url": "https://www.binance.com"}])
}

#[post("/exchanges", format = "json")]      // application-json
fn create_exchange() -> Value{
    json!([{"id": 3, "name": "gate", "url": "https://www.gate.io"}])
}

#[put("/exchanges/<id>", format = "json")]
fn update_exchange(id: i32) -> Value{
    json!([{"id": id, "name": "updated", "url": "https://updated_url"}])
}

#[delete("/exchanges/<_id>")]
fn delete_exchange(_id: i32) -> status::NoContent{
    status::NoContent
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
        .launch()
        .await;
}
