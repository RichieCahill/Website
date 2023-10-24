use actix_files::Files;
use actix_web::{get, App, HttpResponse, HttpServer};

#[get("/")]
async fn index() -> HttpResponse {
    let html = include_str!("../static/index.html");
    HttpResponse::Ok().body(html)
}

#[get("/about")]
async fn about() -> HttpResponse {
    let html = include_str!("../static/about.html");
    HttpResponse::Ok().body(html)
}

#[get("/resume")]
async fn resume() -> HttpResponse {
    let html = include_str!("../static/resume.html");
    HttpResponse::Ok().body(html)
}

#[get("/mistakes")]
async fn mistakes() -> HttpResponse {
    let html = include_str!("../static/mistakes.html");
    HttpResponse::Ok().body(html)
}

#[get("/blog")]
async fn blog() -> HttpResponse {
    let html = include_str!("../static/blog.html");
    HttpResponse::Ok().body(html)
}

#[get("/fizzbuzz")]
async fn fizzbuzz() -> HttpResponse {
    let html = include_str!("../static/fizzbuzz.html");
    HttpResponse::Ok().body(html)
}

#[get("/favicon")]
async fn favicon() -> actix_web::Result<actix_files::NamedFile> {
Ok(actix_files::NamedFile::open("../static/favicon.ico")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(Files::new("/static", "./static").prefer_utf8(true))
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
