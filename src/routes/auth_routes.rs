use actix_web::{middleware::from_fn, web};
use crate::handlers::{dashboard::home::home_page,auth::{login,sign_up}};
use crate::middleware::auth_middleware::check_auth_middlewarer;

pub  fn config(cfg:&mut web::ServiceConfig){
    cfg.service(
        web::scope("/api")
            .service(
                web::scope("/auth")
                .route("/login", web::post().to(login))
                .route("/signup", web::post().to(sign_up))
                // .route("/get_info", web::get().to(sign_up))
          
            )

            .service(
                web::scope("/dashboard")
                .wrap(from_fn(check_auth_middlewarer))
                .service(web::resource("/home").route(web::get().to(home_page)))

            )
     
    );
           
}