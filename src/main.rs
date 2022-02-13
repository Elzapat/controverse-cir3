#![feature(decl_macro)]

#[macro_use]
extern crate rocket;

mod back_office;
pub mod texts;

use rocket_contrib::{serve::StaticFiles, templates::Template};
use texts::load_texts;

#[get("/")]
fn homepage() -> Template {
    Template::render("homepage", "")
}

#[get("/about")]
fn about() -> Template {
    Template::render("about", "")
}

#[get("/actors")]
fn actors() -> Template {
    let texts = load_texts();
    Template::render("actors", texts.actors)
}

#[get("/first_period")]
fn first_period() -> Template {
    let texts = load_texts();
    Template::render("first_period", texts.first_period)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![homepage, actors, first_period, about])
        .mount("/home", routes![homepage])
        .mount("/back-office", routes![back_office::back_office])
        .mount("/static", StaticFiles::from("static"))
        .attach(Template::fairing())
        .launch();
}
