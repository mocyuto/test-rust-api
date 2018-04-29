#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
pub mod usecase;
pub mod domain;
pub mod external;


#[get("/debug")]
fn debug() -> &'static str {
    "Hello, HTTP Server"
}

#[get("/user/<user_id>")]
fn get_user(user_id: i32) -> String {
    let user_name = usecase::user::find_by(user_id);
    format!("Hello, {}!", user_name)
}

fn main() {
    rocket::ignite()
    .mount("/", routes![debug,get_user])
    .launch();
}
