#[macro_use] extern crate rocket;

use rocket::serde::json::Json;

#[get("/say_hello")]
fn say_hello() -> &'static str {
    "Hello, World!"
}

#[get("/say_hello_to/<name>")]
fn say_hello_to(name: String) -> String {
    format!("Hello, {}!", name)
}

#[post("/fibonacci/<n>")]
fn fibonacci_num(n: u64) -> Json<u64> {
    fn fib(n: u64) -> u64 {
        if n == 0 {
            return 0;
        }
        let mut nm2 = 0;
        let mut nm1 = 1;
        for _ in 2..=n {
            let temp = nm1;
            nm1 = nm1 + nm2;
            nm2 = temp;
        }
        nm1
    }

    let result = fib(n);
    Json(result)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/basic_endpoints", routes![say_hello, say_hello_to, fibonacci_num])
}
