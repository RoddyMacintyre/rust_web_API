use rocket::http::Status;
use rocket::Request;
use rocket::request::{FromRequest, Outcome};

// For auth we will make use of the rocket request guard concept
pub struct BasicAuth{
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
                // Check username & password here
                if auth.username == String::from("foo") && auth.password == String::from("bar") {
                    return Outcome::Success(auth)
                }
            }
        }

        Outcome::Error((Status::Unauthorized, ()))
    }
}
