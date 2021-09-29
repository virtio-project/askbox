use crate::error::Result;
use crate::models::{Ask, Askee};
use crate::types::DbPool;
use crate::hcaptcha::Hcaptcha;
use actix_web::{get, post, web, HttpResponse};

pub mod admin;

#[post("/ask")]
pub async fn post_ask(_captcha: Hcaptcha, pool: web::Data<DbPool>, req: web::Json<Ask>) -> Result<HttpResponse> {
    let req = req.into_inner();
    let id = Ask::create(&**pool, req.askee, req.content, req.dedup).await?;
    let ask = Ask::load(&**pool, id).await?;
    Ok(HttpResponse::Ok().json(ask))
}

#[get("/askee")]
pub async fn get_all_askee(pool: web::Data<DbPool>) -> Result<HttpResponse> {
    let askees = Askee::fetch_all(&**pool).await?;
    Ok(HttpResponse::Ok().json(askees))
}

#[get("/askee/{id}")]
pub async fn get_askee(pool: web::Data<DbPool>, id: web::Path<i32>) -> Result<HttpResponse> {
    let askee = Askee::load(&**pool, *id).await?;
    Ok(HttpResponse::Ok().json(askee))
}