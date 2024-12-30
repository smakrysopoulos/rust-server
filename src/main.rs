mod models;
mod services;
mod routes;


use actix_web::{get, middleware::Logger, web::Data, App, HttpResponse, HttpServer, Responder};
use routes::build_metadata_route::{create_metadata, get_metadata, get_metadata_list};
use services::db::Database;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let db = Database::init().await;
    let db_data = Data::new(db);
    HttpServer::new(move || App::new().app_data(db_data.clone())
    .wrap(Logger::default())
    .service(create_metadata)
    .service(get_metadata)
    .service(get_metadata_list)
    )
    .workers(num_cpus::get())
    .bind(("0.0.0.0", 5001))?
    .run()
    .await
}
