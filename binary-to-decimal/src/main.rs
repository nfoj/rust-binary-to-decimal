#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    include_str!("templates/index.html")
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
