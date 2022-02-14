#![feature(decl_macro)]

#[macro_use]
extern crate rocket;

pub mod texts;

use rocket_contrib::{serve::StaticFiles, templates::Template};
use texts::*;

#[get("/")]
fn accueil() -> Template {
    let texts = load_texts().unwrap();
    Template::render("accueil", texts.accueil)
}

#[get("/a_propos")]
fn a_propos() -> Template {
    Template::render("a_propos", "")
}

#[get("/acteurs")]
fn acteurs() -> Template {
    let texts = load_texts().unwrap();
    Template::render("acteurs", texts.acteurs)
}

#[get("/premiere_periode")]
fn premiere_periode() -> Template {
    let texts = load_texts().unwrap();
    Template::render("premiere_periode", texts.premiere_periode)
}

#[get("/vue_ensemble")]
fn vue_ensemble() -> Template {
    Template::render("vue_ensemble", "")
}

#[get("/deuxieme_periode")]
fn deuxieme_periode() -> Template {
    Template::render("deuxieme_periode", "")
}

#[get("/interview")]
fn interview() -> Template {
    Template::render("interview", "")
}

#[get("/conclusion")]
fn conclusion() -> Template {
    Template::render("conclusion", "")
}

#[get("/bibliography")]
fn bibliography() -> Template {
    Template::render("bibliography", "")
}

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![
                textes,
                accueil,
                acteurs,
                vue_ensemble,
                premiere_periode,
                deuxieme_periode,
                interview,
                conclusion,
                bibliography,
                a_propos
            ],
        )
        .mount("/accueil", routes![accueil])
        .mount(
            "/textes",
            routes![accueil_textes, premiere_periode_textes, acteurs_textes],
        )
        .mount("/static", StaticFiles::from("static"))
        .attach(Template::fairing())
        .launch();
}
