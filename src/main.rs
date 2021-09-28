use actix_web::{error, middleware::*, web, App, HttpResponse, HttpServer};
use askbox::api::{admin::*, *};
use sqlx::postgres::PgPoolOptions;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();

    let pool = PgPoolOptions::default()
        .connect("postgres://askbox:askbox@localhost/askbox")
        .await?;

    HttpServer::new(move || {
        App::new()
            .app_data(web::JsonConfig::default().error_handler(|err, _req| {
                error::InternalError::from_response(
                    "",
                    HttpResponse::BadRequest()
                        .content_type("application/json")
                        .body(format!(r#"{{"error":"{}"}}"#, err)),
                )
                .into()
            }))
            .wrap(Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .service(
                web::scope("/api")
                    .service(get_askee)
                    .service(get_all_askee)
                    .service(post_ask)
                    .service(
                        web::scope("/admin")
                            .service(reload)
                            .service(get_ask)
                            .service(add_askee),
                    ),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;
    Ok(())
}
