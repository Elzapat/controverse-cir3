#![feature(decl_macro)]

#[macro_use]
extern crate rocket;

mod back_office;

use rocket_contrib::{serve::StaticFiles, templates::Template};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Texts {
    actors: ActorsText,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ActorsText {
    fda: String,
    ttandme: String,
    google: String,
    scientists: String,
    anne_wo1: String,
    anne_wo2: String,
}

pub fn load_texts() -> Texts {
    let file = fs::read_to_string("static/texts.ron").expect("Couldn't load texts file");

    ron::from_str(&file).expect("couldn't deserialize ron file")
}

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

fn main() {
    rocket::ignite()
        .mount("/", routes![homepage, about, actors])
        .mount("/home", routes![homepage])
        .mount("/back-office", routes![back_office::back_office])
        .mount("/static", StaticFiles::from("static"))
        .attach(Template::fairing())
        .launch();
}
