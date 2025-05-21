use actix_web::{App, HttpServer, Responder};

#[actix_web::get("/greet")]
async fn greet() -> impl Responder {
    format!("Hello World")

}

#[actix_web::main]
async fn main() {
    // Server port num must be u16
    let port: u16 = 8080;
    println!("Starting on port {port}");

    HttpServer::new(|| App::new().service(greet))
        .bind(("127.0.0.1", port))
        .expect("Failed to bind to server port")
        .workers(2)
        .run()
        .await
        .expect("Server failed to run");


}
