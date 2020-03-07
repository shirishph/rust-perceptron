#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_cors;

const X1:usize = 0;
const X2:usize = 1;
const RESULT:usize = 2;

use rocket_cors::AllowedHeaders;
use rocket_cors::AllowedOrigins;
use rocket_cors::Error;

#[get("/base")]
fn base() -> &'static str {
    "Hello, world 456!" 
}

fn main() -> Result<(), Error> {

    let allowed_origins = AllowedOrigins::all();
    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()?;

    rocket::ignite()
        .mount("/", routes![base])
        .attach(cors)
        .launch();

    let truth_table = [
        [1, 1, 1],
        [1, -1, -1],
        [-1, 1, -1],
        [-1, -1, -1]
    ];
    let mut pattern;
    for i in 0..truth_table.len() {
        pattern = truth_table[i];
        for j in 0..pattern.len() {
            print!(" {:?}", pattern[j]);
        }
        println!(" ")
    }

    let mut w1 = 0;
    let mut w2 = 0;
    let mut bias = 0;
    let learning_rate = 1;
    let mut y_in;
    let mut y;
    let mut logme = Vec::new();
    let mut add_this:Vec<i64>;

    for epoch in 0..2 {
        println!("\n=================\n{:?}", epoch);

        for i in 0..truth_table.len() {
            pattern = truth_table[i];
            println!("-------------------------------");
            println!("input pattern: {:?}", pattern);
            println!("w1: {}, w2: {}, bias: {}", w1, w2, bias);

            y_in = bias + (w1 * pattern[X1]) + (w2 * pattern[X2]);
            println!("y_in: {}", y_in);

            if y_in > 0 {
                y = 1;
            } else if y_in == 0 {
                y = 0;
            }
            else {
                y = -1;
            }

            println!("y: {}", y);

            if y != pattern[RESULT] {
                println!("   {} (y) does not match {:?} (result), so weights will be updated", y, pattern[RESULT]);

                w1 = w1 + (learning_rate * pattern[RESULT] * pattern[X1]);
                println!("   w1: {}", w1);

                w2 = w2 + (learning_rate * pattern[RESULT] * pattern[X2]);
                println!("   w2: {}", w2);

                bias = bias + (learning_rate * pattern[RESULT]);
                println!("   bias: {}", bias);
            }
            else {
                println!("   {} (y) matches {:?} (result), so weights will not be updated", y, pattern[RESULT]);
                println!("   w1: {}", w1);
                println!("   w2: {}", w2);
                println!("   bias: {}", bias);
            }

            add_this = vec![pattern[X1], pattern[X2], pattern[RESULT], y_in, y, w1, w2, bias];
            logme.push(add_this);
        }
    }

    for x in &logme {
        for y in x {
            print!("{number:>width$}", number=y, width=5);
        }
        println!("\n")
    }

    Ok(())
}
