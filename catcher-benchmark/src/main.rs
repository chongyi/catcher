use catcher_core::request;
use std::time::{Instant, Duration};

fn main() {
    let mut escapes = vec![];
    let client = request::blocking::Client::builder()
        .build()
        .unwrap();
    for i in 0..500 {
        let instant = Instant::now();
        let resp = request::blocking::get("http://127.0.0.1:8888/");
        let escaped = instant.elapsed();
        escapes.push(escaped);

        if (i + 1) % 100 == 0 {
            println!("Completed {} requests", i + 1);
        }
    }

    println!();
    let sum = escapes.iter().sum::<Duration>();
    println!("Total time: {} sec", sum.as_secs_f64());

    let per_request_escaped = sum / escapes.len() as u32;
    println!("Per-request time: {} ms", per_request_escaped.as_secs_f64() * 1000.0);

    println!();
    println!("Requests per second: {} / sec", 1.0 / sum.as_secs_f64() * 500.0);
}
