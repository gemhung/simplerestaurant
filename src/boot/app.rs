use crate::config::settings;
use actix_web::cookie::SameSite;
use actix_web::dev::ServiceFactory;
use actix_web::dev::ServiceRequest;
use actix_web::dev::ServiceResponse;
use actix_web::App;
use actix_web::HttpServer;

pub async fn launch() -> Result<(), actix_web::Error> {
    HttpServer::new(move || app())
        .bind(("0.0.0.0", 8080))?
        .run()
        .await?;

    Ok(())
}

pub fn app() -> App<
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
        .configure(routes);
    //.default_service(actix_web::web::get().to(crate::utils::response::e404_html));

    app
}

use actix_web::web::{self, delete, get, head, patch, post, put, trace, ServiceConfig};

pub fn routes(cfg: &mut ServiceConfig) {
    //cfg.route("/{tail:.*}", get().to(the_world::the_world));
}
