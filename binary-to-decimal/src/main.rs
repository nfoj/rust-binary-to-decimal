#[macro_use]
extern crate rocket;

use rocket::{fs::FileServer, Build};

#[get("/")]
fn index() -> &'static str {
    include_str!("templates/index.html")
}

#[post("/convert")]
fn convert(binary: <std::string::String as ToString>::From) -> String {
    fn binary_to_decimal(binary: &str) -> Result<i32, String> {
        let mut decimal = 0;
        let mut power = 0;

        for c in binary.chars().rev() {
            if c != '0' && c != '1' {
                return Err("Entrada inválida. Somente 0s e 1s são permitidos.".to_string());
            }
            decimal += (c as u8 - b'0') as i32 * 2i32.pow(power);
            power += 1;
        }

        Ok(decimal)
    }

    match binary_to_decimal(&binary) {
        Ok(result) => result.to_string(),
        Err(error) => error,
    }
}

#[launch]
fn rocket() -> rocket::Rocket<Build> {
    rocket::build()
        .mount("/", routes![index, convert])
        .mount("/static", FileServer::from("static"))
}
