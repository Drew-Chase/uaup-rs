use actix_web::{get, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use log::info;
use serde_json::json;

#[actix::main]
async fn start_server(port: u16) -> std::io::Result<()> {
	std::env::set_var("RUST_LOG", "trace");
	env_logger::init();

	info!("Starting manifest server at http://127.0.0.1:{}",  port);

	HttpServer::new(move || {
		let app = App::new()
//			.wrap(middleware::Logger::default())
			.service(web::scope("api").service(status));
	})
		.workers(4)
		.bind(format!("0.0.0.0:{port}", port = port))?
		.run()
		.await
}

/// Handles requests to check the server status.
///
/// This endpoint responds to GET requests with a JSON object indicating
/// that the server is running correctly. It can be used for health checks
/// or monitoring server status.
///
/// # Returns
///
/// A JSON object with a `status` field set to "ok".
#[get("/")]
async fn status() -> impl Responder {
	HttpResponse::Ok().json(json!({ "status": "ok" }))
}