
use std::env;

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize,Serialize};
use chrono::{self, Duration};


use dotenvy::dotenv;

#[derive(Debug,Serialize, Deserialize, Clone)]
pub struct UserClaims {
    exp:usize,
    iat:usize,
    pub sub: String,
    pub id: i32,
}

#[derive(Serialize)]
pub struct token_response{
    pub access_token: String
}


pub fn encode_jwt(username:String, id:i32)->String{
    let secret_key:String = match env::var("JWT_KEY"){
        Ok(val)=> val,
        Err(e)=> "unable to find secret key".to_string(),
    };

    let now = chrono::Utc::now();
    let expire = Duration::hours(24);

    let user_claim = UserClaims{
        exp:(now + expire).timestamp() as usize,
        iat: now.timestamp() as usize,
        sub:username,
        id
     };

    let token = match encode(&Header::default(), &user_claim, &EncodingKey::from_secret(secret_key.as_ref())){
        Ok(val)=>val,
        Err(e)=>"encoding error {e}".to_string()
    };
    token

}


pub fn decode_jwt(jwt:String)->Result<TokenData<UserClaims>, jsonwebtoken::errors::Error>{
    let secret_key:String = match env::var("JWT_KEY"){
        Ok(val)=> val,
        Err(e)=> "unable to find secret key".to_string(),
    };
    dbg!(&jwt);
    println!("decoding token{jwt}");
    let claim:Result<TokenData<UserClaims>,jsonwebtoken::errors::Error> = decode(&jwt, &DecodingKey::from_secret(secret_key.as_bytes()), &Validation::default());
    println!("{:?} claaim error", claim);
    claim


                    
    
    }

