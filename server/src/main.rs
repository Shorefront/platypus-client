use actix_files::{Files, NamedFile};
use actix_web::{web, App, HttpServer, Result, HttpResponse};
use std::path::PathBuf;

// Fallback handler to serve index.html for client-side routing
async fn index() -> Result<NamedFile> {
    Ok(NamedFile::open(PathBuf::from("./client/dist/index.html"))?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let addr = "0.0.0.0:8001";
    println!("Server running at http://{}", addr);

    HttpServer::new(|| {
        App::new()
            // 1. Optional: Put your API backend routes here
            .service(web::scope("/api").route("/hello", web::get().to(|| async { 
                HttpResponse::Ok().json("Hello from Actix!") 
            })))
            
            // 2. Serve the static assets compiled by Trunk
            .service(Files::new("/", "./client/dist").index_file("index.html"))
            
            // 3. Fallback to index.html for any unmatched routes (handles browser refreshes)
            .default_service(web::to(index))
    })
    .bind(addr)?
    .run()
    .await
}