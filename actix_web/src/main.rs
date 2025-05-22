use actix_web::{App, web, HttpServer, HttpResponse, Responder};
use std::collections::HashMap;
use std::sync::Mutex;

// Application state
struct AppState {
    store: Mutex<HashMap<u32, String>>,
}

#[actix_web::get("/greet/{id}")]
async fn greet(id :  web::Path<u32>) -> impl Responder {
    format!("Hello World {id}")

}

async fn create_item(data: web::Data<AppState>, item: web::Json<(u32, String)>) -> impl Responder {
    let mut db = data.store.lock().unwrap();
    let (id, name) = item.into_inner();
    if db.contains_key(&id) {
        return HttpResponse::BadRequest().body("Item with this ID already exists");
    }
    db.insert(id, name);
    HttpResponse::Created().body("iterm created")
}

async fn get_item(data: web::Data<AppState>, id: web::Path<u32>) -> impl Responder {
    let db = data.store.lock().unwrap();
    if let Some(value) = db.get(&id.into_inner()) {
        HttpResponse::Ok().body(format!("Item: {value}"))
    } else {
        HttpResponse::NotFound().body("Item not found")
    }
}



#[actix_web::main]
async fn main() {
    // Server port num must be u16
    let port: u16 = 8080;
    println!("Starting on port {port}");

    let shared_data = web::Data::new(AppState {
        store: Mutex::new(HashMap::new()),
    });


    HttpServer::new( move ||  {
        App::new()
        .app_data(shared_data.clone())
        .service(greet)
        .route("/item/{id}", web::get().to(get_item))
        .route("/item", web::post().to(create_item))
    })
    .bind(("127.0.0.1", port))
    .expect("Failed to bind to server port")
    .workers(2)
    .run()
    .await
    .expect("Server failed to run");


}
