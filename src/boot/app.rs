use crate::controllers;
use actix_web::dev::Server;
use actix_web::dev::ServiceFactory;
use actix_web::dev::ServiceRequest;
use actix_web::dev::ServiceResponse;
use actix_web::web::{self, delete, get, post, ServiceConfig};
use actix_web::App;
use actix_web::HttpServer;
use std::net::TcpListener;

pub async fn launch(
    configuration: crate::config::configuration::Settings,
) -> Result<(Server, u16), crate::errors::AppError> {
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(address)?;
    let port = listener.local_addr().unwrap().port();

    Ok((
        HttpServer::new(move || app(&configuration))
            .listen(listener)?
            .run(),
        port,
    ))
}

fn app(
    configuration: &crate::config::configuration::Settings,
) -> App<
    impl ServiceFactory<
        ServiceRequest,
        Response = ServiceResponse<impl actix_web::body::MessageBody>,
        Config = (),
        Error = actix_web::Error,
        InitError = (),
    >,
> {
    let db_pool = crate::boot::database::get_connection_pool(&configuration.database);
    let db_pool = actix_web::web::Data::new(db_pool);
    App::new()
        // Middlewares
        .wrap(actix_web::middleware::NormalizePath::trim())
        // Routes
        .configure(routes)
        .app_data(db_pool.clone())
        .default_service(web::to(actix_web::HttpResponse::NotFound))
}

fn routes(cfg: &mut ServiceConfig) {
    // Requirements
    cfg.route("/items", get().to(controllers::get_all_ordered_items));
    cfg.route(
        "/items/{item_name}",
        get().to(controllers::get_specified_ordered_items),
    );
    cfg.route("/items", post().to(controllers::create_orders));
    cfg.route("/items", delete().to(controllers::delete_orders));

    // Health check
    cfg.route("/health_check", get().to(controllers::health_check));
}
