use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn get_handler() -> impl Responder {
    HttpResponse::Ok().body("GET request handled")
}

async fn post_handler() -> impl Responder {
    HttpResponse::Ok().body("POST request handled")
}

async fn delete_handler() -> impl Responder {
    HttpResponse::Ok().body("DELETE request handled")
}

async fn put_handler() -> impl Responder {
    HttpResponse::Ok().body("PUT request handled")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/get", web::get().to(get_handler))
            .route("/post", web::post().to(post_handler))
            .route("/delete", web::delete().to(delete_handler))
            .route("/put", web::put().to(put_handler))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
