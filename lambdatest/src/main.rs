use actix_web::{get, App, HttpResponse, HttpServer};

#[actix_rt::main]
async fn main() -> Result<(), actix_web::Error> {
    HttpServer::new(move || App::new().service(index))
    .bind("0.0.0.0:8080")?
    .run()
    .await?;
    Ok(())
}

async fn index(
    counter_mutex: Data<Mutex<usize>>,
    counter_cell: Data<Cell<u32>>,
    counter_atomic: Data<AtomicUsize>,
    req: HttpRequest,
) -> HttpResponse {
    println!("{:?}", req);

    // Increment the counters
    *counter_mutex.lock().unwrap() += 1;
    counter_cell.set(counter_cell.get() + 1);
    counter_atomic.fetch_add(1, Ordering::SeqCst);

    let body = format!(
        "global mutex counter: {}, local counter: {}, global atomic counter: {}",
        *counter_mutex.lock().unwrap(),
        counter_cell.get(),
        counter_atomic.load(Ordering::SeqCst),
    );
    HttpResponse::Ok().body(body)
}