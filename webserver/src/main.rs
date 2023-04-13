use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::io;
use serde::Deserialize;

// Define a struct to deserialize JSON data from POST requests
#[derive(Deserialize)]
struct PostData {
    message: String,
}

#[derive(Deserialize)]
struct GetQuery {
    param: String,
}

async fn index_handler() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/plain; charset=utf-8")
        .body("Server is running.")
}

// Handler for GET requests
async fn get_handler(query: web::Query<GetQuery>) -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/plain; charset=utf-8")
        .body(format!("Received GET request with parameter: {}", query.param))
}


// Handler for POST requests
async fn post_handler(post_data: web::Json<PostData>) -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/plain; charset=utf-8")
        .body(format!("Received POST request with message: {}", post_data.message))
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    // Initialize logging
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    // Start the server
    let server = HttpServer::new(|| {
        App::new()
            .route("/get", web::get().to(get_handler))
            .route("/post", web::post().to(post_handler))
            .route("/", web::get().to(index_handler))
    })
    .bind("127.0.0.1:8080")?;

    // Display the startup message
    println!("Starting server on 127.0.0.1:8080");

    // Run the server
    server.run().await
}
