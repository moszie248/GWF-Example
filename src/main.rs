use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use std::env;

#[get("/")]
async fn show() -> impl Responder {
	let print_env = env::var("PRINT_ENV").ok().unwrap();
	let port = env::var("PORT").ok().unwrap();
	let endpoint = env::var("ENDPOINT").ok().unwrap();

	format!(
				"PRINT_ENV: {}\nPORT: {}\nENDPOINT: {}",
				print_env, port, endpoint)

      HttpResponse::Ok().body(format!(
        "PRINT_ENV: {}\nPORT: {}\nENDPOINT: {}",
        print_env, port, endpoint
    ))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	dotenv().ok();
	env_logger::init();

	// let port = env::var("PORT")
  //       .unwrap_or_else(|_| "8080".to_string())
  //       .parse::<u16>()
  //       .unwrap();

				HttpServer::new(|| App::new().service(show))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
