use actix_web::{body::MessageBody, dev::{ServiceRequest, ServiceResponse}, error::ErrorUnauthorized, http::{header::AUTHORIZATION, Error}, middleware::{self,Next}, HttpMessage};


use crate::utils::jwt_module::decode_jwt;


pub async fn check_auth_middlewarer(req:ServiceRequest, next:Next<impl MessageBody>)->Result<ServiceResponse<impl MessageBody>, actix_web::Error>{
    let auth_header = req.headers().get(AUTHORIZATION).ok_or_else(|| ErrorUnauthorized("Missing auth header"))?;
    let token = auth_header.to_str().map_err(|_| ErrorUnauthorized("invalid header"))?.strip_prefix("Bearer").ok_or(ErrorUnauthorized("INVALID SCHEME"))?;

    

    let claim: jsonwebtoken::TokenData<crate::utils::jwt_module::UserClaims> = decode_jwt(token.trim().to_owned()).map_err(|_| ErrorUnauthorized("invalid token"))?;
    req.extensions_mut().insert(claim.claims);
  
    let res = next.call(req).await?;

    // Return the response
    Ok(res)

    


}