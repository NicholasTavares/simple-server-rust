use actix_web::{get, web, Responder, Result};
use serde::Serialize;
use std::time::{Duration, Instant};

#[derive(Serialize)]
struct Response {
    input: usize,
    looped_times: i32,
    duration: Duration,
    primes: Vec<i32>,
}

fn is_prime(number: i32) -> bool {
    if number <= 1 {
        return false;
    }

    for i in 2..number {
        if number % i == 0 {
            return false;
        }
    }

    return true;
}

#[get("/prime/{count}")]
async fn index(count: web::Path<String>) -> Result<impl Responder> {
    let start = Instant::now();

    let mut primes = Vec::new();

    let mut n: i32 = 0;

    let converted_count = count.parse::<i32>().unwrap().try_into().unwrap();

    while primes.len() < converted_count {
        if is_prime(n) {
            primes.push(n);
        }

        n += 1;
    }

    let duration = start.elapsed();

    let obj = Response {
        input: converted_count,
        looped_times: n,
        duration,
        primes,
    };
    Ok(web::Json(obj))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    HttpServer::new(|| App::new().service(index))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
