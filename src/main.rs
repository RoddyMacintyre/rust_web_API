#[macro_use] extern crate rocket;

use rocket::http::Status;
use rocket::Request;
use rocket::request::{FromRequest, Outcome};
use rocket::response::status;
use rocket::serde::json::{Value, json};

// For auth we will make use of the rocket request guard concept
struct BasicAuth{
    pub username: String,   // Public for routes to access them
    pub password: String,
}

// Factory methods
impl BasicAuth{
    fn from_authorization_header(header: &str) -> Option<BasicAuth>{
        let split = header.split_whitespace().collect::<Vec<_>>();
        if split.len() != 2 {
            return None;
        }

        if split[0] != "Basic" {
            return None;
        }

        Self::from_base64_encoded(split[1])
    }

    fn from_base64_encoded(base64_string: &str) -> Option<BasicAuth>{
        let decoded = base64::decode(base64_string).ok()?;  // ok() tries to convert to an Option
        let decoded_string = String::from_utf8(decoded).ok()?;  // ? Means only continue, if the line is ok and returns Some
        let split = decoded_string.split(":").collect::<Vec<_>>();

        if split.len() != 2{
            return None;
        }

        let (username, password) = (split[0].to_string(), split[1].to_string());

        Some(BasicAuth{
            username,
            password,
        })
    }
}

// Implement a trait to use the BasicAuth as a rocket guard
#[rocket::async_trait]
impl<'r> FromRequest<'r> for BasicAuth{
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error>{
        let auth_header = request.headers().get_one("Authorization");

        if let Some(auth_header) = auth_header{
            if let Some(auth) = Self::from_authorization_header(auth_header){
                return Outcome::Success(auth)
            }
        }

        Outcome::Error((Status::Unauthorized, ()))
    }
}


#[get("/")]     // Get attribute
fn hello() -> Value{
    json!("Hello, world!")
}

#[get("/exchanges")]
fn get_exchanges(_auth: BasicAuth) -> Value{     // Implement BasicAuth for this endpoint
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
