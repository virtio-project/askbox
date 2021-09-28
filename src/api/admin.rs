use crate::error::Result;
use crate::models::{Ask, Askee, TimeRange};
use crate::types::DbPool;
use actix_web::{get, post, web, HttpResponse};
use chrono::{Utc, Duration};
use std::ops::Sub;

#[post("/reload")]
pub async fn reload() -> Result<HttpResponse> {
    Ok(HttpResponse::NoContent().finish())
}

#[get("/ask/{id}")]
pub async fn get_ask(pool: web::Data<DbPool>, id: web::Path<i32>) -> Result<HttpResponse> {
    let ask = Ask::load(&**pool, *id).await?;
    Ok(HttpResponse::Ok().json(ask))
}

#[get("/askee/{id}/asks")]
pub async fn get_all_asks(pool: web::Data<DbPool>, id: web::Path<i32>, time_range: web::Query<TimeRange>) -> Result<HttpResponse> {
    let asks = Ask::fetch_all(
        &**pool,
        *id,
        time_range.before.unwrap_or_else(|| Utc::now().naive_utc()),
        time_range.after.unwrap_or_else(|| Utc::now().sub(Duration::days(1)).naive_utc()),
    )
        .await?;
    Ok(HttpResponse::Ok().json(asks))
}

#[post("/add-askee")]
pub async fn add_askee(pool: web::Data<DbPool>, req: web::Json<Askee>) -> Result<HttpResponse> {
    let req = req.into_inner();
    let id = Askee::create(&**pool, req.display_name).await?;
    let askee = Askee::load(&**pool, id).await?;
    Ok(HttpResponse::Ok().json(askee))
}
