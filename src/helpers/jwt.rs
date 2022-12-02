use chrono::{Duration, Local};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use rocket::{http::Status, request::{FromRequest, Outcome}, Request};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    exp: usize,
    user: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthObject {
    pub authorized: bool,
    pub user: String,
}

pub fn jwt_sign(user: &str) -> String {
    let exp = Local::now() + Duration::days(10);

    let my_claims = Claims {
        user: user.to_string(),
        exp: exp.timestamp() as usize,
    };

    let token = encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret("secret".as_ref()),
    ).unwrap();

    token
}

pub fn jwt_validate(token: &str) -> AuthObject {
    let t = token.replace("Bearer ", "");

    match decode::<Claims>(
        &t,
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::default(),
    ) {
        Ok(t) => {
            // get user with decoded token and store
            return AuthObject {
                authorized: true,
                user: t.claims.user
            };
        }, 
        _ => return AuthObject { authorized: false, user: "".to_string() },
    };
}

// #[derive(Debug)]
// pub struct Auth(AuthObject);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthObject {
    type Error = &'r str;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let token = match req.headers().get("authorization").next() {
            Some(a) => a,
            _ => return Outcome::Failure((Status::BadRequest, "Authorization header not found")),
        };

        let validate = jwt_validate(token);
        if !validate.authorized {
            return Outcome::Failure((Status::Unauthorized, "User is not authorized"));
        }

        Outcome::Success(validate)
    }
}
