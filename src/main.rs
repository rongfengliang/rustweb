#![feature(plugin)]
#![plugin(rocket_codegen)]
mod user;
extern crate rocket;
#[macro_use]
extern crate mysql;
#[get("/")]
fn hello() -> String {
    format!("Hello, {} year old named {}!", "hello", 33)
}

#[get("/login/<name>/<age>")]
fn login(name:String,age:i32)->String{
   let user= user::modle::User{
        name:name,
        age:333
    };
    user.mysqldata();
    user.login(user.name.clone(), age)
}
fn main() {
    rocket::ignite().mount("/", routes![hello,login]).launch();
}