use actix_web::{get, App, HttpResponse, HttpServer};
use tokio::fs::read_to_string;

#[get("/")]
async fn index() -> HttpResponse {
    match read_to_string("./index.html").await {
        Ok(html) => HttpResponse::Ok().body(html),
        Err(_) => HttpResponse::InternalServerError().body("Sorry, out of order"),
    }
}

#[get("/about")]
async fn about() -> HttpResponse {
    match read_to_string("./about.html").await {
        Ok(html) => HttpResponse::Ok().body(html),
        Err(_) => HttpResponse::NotFound().body("page not found"),
    }
}

#[get("/resume")]
async fn resume() -> HttpResponse {
    match read_to_string("./resume.html").await {
        Ok(html) => HttpResponse::Ok().body(html),
        Err(_) => HttpResponse::NotFound().body("page not found"),
    }
}

#[get("/mistakes")]
async fn mistakes() -> HttpResponse {
    match read_to_string("./mistakes.html").await {
        Ok(html) => HttpResponse::Ok().body(html),
        Err(_) => HttpResponse::NotFound().body("page not found"),
    }
}

#[get("/blog")]
async fn blog() -> HttpResponse {
    match read_to_string("./blog.html").await {
        Ok(html) => HttpResponse::Ok().body(html),
        Err(_) => HttpResponse::NotFound().body("page not found"),
    }
}

#[get("/fizzbuzz")]
async fn fizzbuzz() -> HttpResponse {
    match read_to_string("./fizzbuzz.html").await {
        Ok(html) => HttpResponse::Ok().body(html),
        Err(_) => HttpResponse::NotFound().body("page not found"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(about)
            .service(resume)
            .service(mistakes)
            .service(blog)
            .service(fizzbuzz)
    })
    .bind("127.0.0.1:5000")?
    .run()
    .await
}