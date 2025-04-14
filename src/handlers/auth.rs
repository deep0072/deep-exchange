use actix_web::{body::MessageBody, dev::{ServiceRequest, ServiceResponse},middleware::{from_fn, Next}, web, App,Error, HttpRequest, HttpResponse, Responder};
use jsonwebtoken::encode;
use sqlx::{Pool, Postgres, Row};
use serde_json::{self, Value};
use crate::{models::Users, utils::jwt_module::{encode_jwt, token_response}, AppState};
use bcrypt::{hash, verify, BcryptError, DEFAULT_COST};




// pub async fn login(data: web::Data<AppState>,json: web::Json<Users::Users>) -> impl Responder {
//     match sqlx::query("SELECT username from users where username=($1) and passsword=($2")
//         .bind(&json.username)
//         .fetch_one(&data.db).await {
//             Ok(_)=>HttpResponse::Ok().json("access token is 980--989798089"),
//             Err(e) => HttpResponse::NotFound().json(format!("bad request {}",e))
//         }
   
// }
pub async fn login(data: web::Data<AppState>,json: web::Json<Users::Users>) -> impl Responder {

  
    match sqlx::query!("SELECT username,password,id from users where username=($1)",&json.username)
        .fetch_one(&data.db).await {
            Ok(user)=>{
                println!("{:?}",user.username);
                match verify(&json.password, &user.password){
                    Ok(true)=> {

                        let token  =encode_jwt(user.username, user.id);
                        let response = token_response{
                            access_token:token
                        };
                        HttpResponse::Ok().json(&response)

                    } ,
                    Ok(false) => HttpResponse::Unauthorized().json(" is wrong"),
                    Err(BcryptError) => HttpResponse::Unauthorized().json(format!("password is  wrong {}",BcryptError::InvalidHash(String::from("not"))))
                }   
            
            } 
            Err(e) => HttpResponse::NotFound().json(format!("bad request {}",e))
     
   
    }
}

pub async fn sign_up(data: web::Data<AppState>, json: web::Json<Users::Users>) -> impl Responder {

    
    let hash_pass = match hash(&json.password,DEFAULT_COST){
        Ok(hashed)=>hashed,
        Err(e)=>return  HttpResponse::InternalServerError().json(format!("falided to hash password {}",e))
    };
        
 
    match sqlx::query("INSERT INTO users (username, password) VALUES ($1, $2)")
        .bind(&json.username)
        .bind(hash_pass)
        .execute(&data.db)
        .await
    {
        Ok(_) => HttpResponse::Ok().json("User created successfully"),
        Err(e) => HttpResponse::InternalServerError().json(format!("Failed to create user: {}", e))
    }
}



// Assuming you have a JWT validation library
// and a way to represent user information from the JWT



