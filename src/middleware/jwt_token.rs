use jsonwebtoken::{encode, EncodingKey, Header};
use actix_web_httpauth::middleware::HttpAuthentication;
use actix_web_httpauth::extractors::bearer::BearerAuth;

use crate::utils::jwt_module::decode_jwt;


// async fn validator(auth: BearerAuth) -> Result<Claims, Error> {
//     let secret = b"your_secret_key";
//     match decode_jwt(auth.token()) {
//         Ok(claims) => Ok(claims),
//         Err(_) => Err(HttpResponse::Unauthorized().finish().into()),
//     }
// }