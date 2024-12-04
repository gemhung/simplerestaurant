use crate::config::settings;
use crate::controllers;
use actix_web::cookie::SameSite;
use actix_web::dev::ServiceFactory;
use actix_web::dev::ServiceRequest;
use actix_web::dev::ServiceResponse;
use actix_web::web::{self, delete, get, head, patch, post, put, trace, ServiceConfig};
use actix_web::App;
use actix_web::HttpServer;

pub async fn launch() -> Result<(), actix_web::Error> {
    HttpServer::new(move || app())
        .bind(("127.0.0.1", 8080))?
        .run()
        .await?;

    Ok(())
}

fn app() -> App<
    impl ServiceFactory<
        ServiceRequest,
        Response = ServiceResponse<impl actix_web::body::MessageBody>,
        Config = (),
        Error = actix_web::Error,
        InitError = (),
    >,
> {
    let app = App::new()
        // Middlewares
        .wrap(actix_web::middleware::NormalizePath::trim())
        // Routes
        .configure(routes)
        .default_service(web::to(|| actix_web::HttpResponse::NotFound()));

    app
}

fn routes(cfg: &mut ServiceConfig) {
    cfg.route("/items", get().to(controllers::get_all_ordered_items));
    cfg.route(
        "/items/{item_name}",
        get().to(controllers::get_specified_ordered_items),
    );
    cfg.route("/items", post().to(controllers::create_orders));
    cfg.route("/items", delete().to(controllers::delete_orders));
}
