use actix_web::{get, web, App, HttpServer, Responder, http::{Method, StatusCode}, Either, HttpResponse, Result, Error};
use actix_files::NamedFile;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}")
}

async fn default_handler(req_method: Method) -> Result<impl Responder, Error> {
    match req_method {
        Method::GET => {
            let file = NamedFile::open("static/404.html")?
                .customize()
                .with_status(StatusCode::NOT_FOUND);
            Ok(Either::Left(file))
        },
        _ => Ok(Either::Right(HttpResponse::MethodNotAllowed())),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(greet)
            .default_service(web::to(default_handler))
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}
