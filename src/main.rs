#![feature(decl_macro)]

#[macro_use]
extern crate rocket;

mod back_office;

use rocket_contrib::{serve::StaticFiles, templates::Template};
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct Texts {
    homepage_1: String,
    homepage_2: String,
}

#[get("/")]
fn homepage() -> Template {
    Template::render(
        "homepage",
        Texts {
            homepage_1: String::from("hello"),
            homepage_2: String::from("hello2"),
        },
    )
}

#[get("/about")]
fn about() -> Template {
    Template::render("about", "")
}

#[get("/actors")]
fn actors() -> Template {
    Template::render("actors", "")
}

fn main() {
    rocket::ignite()
        .mount("/", routes![homepage, about, actors])
        .mount("/home", routes![homepage])
        .mount("/back-office", routes![back_office::back_office])
        .mount("/static", StaticFiles::from("static"))
        .attach(Template::fairing())
        .launch();
}
