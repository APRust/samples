use actix_web::{web, App, HttpRequest, HttpServer, Responder};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        println!("http server factory is firing");
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
            .route("/say/hello", web::get().to(|| async { "Hello Again!" }))
    })
    .bind("127.0.0.1:8080")?
    // .workers(3)
    .run()
    .await
}

// The preceding result tells us that the closure was fired three times.
// Altering the number of workers shows us that there is a direct relationship
// between this and the number of times the closure is fired.
// If the workers function is left out, then the closure is fired in relation
// to the number of cores your system has.
// We will explore how these workers fit into the server process in the next section.
