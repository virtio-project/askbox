use crate::error::Result;
use crate::models::Askee;
use crate::types::DbPool;
use actix_web::{post, web, HttpResponse};

#[post("/reload")]
pub async fn reload() -> Result<HttpResponse> {
    Ok(HttpResponse::NoContent().finish())
}

#[post("/add-askee")]
pub async fn add_askee(pool: web::Data<DbPool>, req: web::Json<Askee>) -> Result<HttpResponse> {
    let req = req.into_inner();
    let id = Askee::create(&**pool, req.display_name).await?;
    let askee = Askee::load(&**pool, id).await?;
    Ok(HttpResponse::Ok().json(askee))
}
