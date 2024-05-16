// region: --- imports
pub mod _utils;

use crate::_utils::endpoints::register_endpoints;
use _utils::log::print_log;

use actix_web::{web, App, HttpServer};
use std::sync::Arc;
use tokio::sync::Mutex;
use voxctl::AppState;
// endregion: --- imports

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let nexus = Arc::new(Mutex::new(AppState { running: None }));

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(nexus.clone()))
            .configure(register_endpoints)
    })
    .bind("127.0.0.1:8080")?;

    print_log("Server starting...");
    let server_handle = server.run();
    print_log("Server started, ready for requests.");

    server_handle.await?;
    print_log("Server stopped.");

    Ok(())
}
