#![feature(plugin)]
#![plugin(rocket_codegen)]
mod user;
extern crate rocket;
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
    user.login(user.name.clone(), age)
   // format!("userlogin, name: {} age: {}!", user.name, user.age)
}

fn main() {
    rocket::ignite().mount("/", routes![hello,login]).launch();
}