#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[get("/")]
fn hello() -> String {
    format!("Hello, {} year old named {}!", "hello", 33)
}

fn main() {
    rocket::ignite().mount("/", routes![hello]).launch();
}