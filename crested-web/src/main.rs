use actix_web::{get, middleware, App, HttpRequest, HttpResponse, HttpServer, Result};
use actix_web::http::StatusCode;
use actix_files::Files;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .service(Files::new("/webcam", "./crested-web/static/webcam/")
                .index_file("webcam.html"))
            .service(Files::new("/", "./crested-web/static/major/")
                .index_file("index.html"))

    })
    .bind(("127.0.0.1", 60000))?
    .run()
    .await
}
