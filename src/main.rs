#[macro_use] extern crate rocket;
use rocket_dyn_templates::{Template, context};
mod sas;

#[get("/api")]
async fn api() -> String {
    sas::modif()
}

#[get("/")]
async fn index() -> Template {
    Template::render("index", context!{
        modif: sas::modif()
    })
}

#[shuttle_runtime::main]
async fn rocket() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build()
        .mount("/", routes![index, api])
        .attach(Template::fairing());
    Ok(rocket.into())
}
