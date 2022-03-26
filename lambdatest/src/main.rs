use actix_web::{get, App, HttpResponse, HttpServer};

#[actix_rt::main]
async fn main() -> Result<(), actix_web::Error> {
    HttpServer::new(move || App::new().service(index))
    .bind("0.0.0.0:8080")?
    .run()
    .await?;
    Ok(())
}
