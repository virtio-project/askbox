use actix_web::{error, middleware::*, web, App, HttpResponse, HttpServer};
use askbox::api::{admin::*, *};
use sqlx::postgres::PgPoolOptions;
use actix_cors::Cors;
use askbox::config::Config;
use std::borrow::Borrow;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();

    let config = Config::default();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect_with(config.database.borrow().into())
        .await?;

    let config_cloned = config.clone();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST"])
            .allow_any_header()
            .max_age(10);

        App::new()
            .wrap(cors)
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
            .app_data(web::Data::new(config_cloned.hcaptcha.clone()))
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
    .bind(("::", 8080))?
    .run()
    .await?;
    Ok(())
}
