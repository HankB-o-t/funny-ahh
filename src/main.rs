#[macro_use] extern crate rocket;
use rocket_dyn_templates::{Template, context};
mod sas;

#[get("/")]
fn index() -> Template {
    Template::render("index", context!{
        modif: sas::modif()
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .attach(Template::fairing())
}
