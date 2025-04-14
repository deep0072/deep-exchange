use actix_web::{web, App, HttpResponse, Responder};
pub async fn home_page()-> impl Responder {
    return HttpResponse::Ok().json("hi welcome to page");

}