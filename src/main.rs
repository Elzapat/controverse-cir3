#![feature(decl_macro)]

#[macro_use]
extern crate rocket;

pub mod texts;

use rocket_contrib::{serve::StaticFiles, templates::Template};
use texts::*;

#[get("/")]
fn accueil() -> Template {
    Template::render("accueil", "")
}

#[get("/acteurs")]
fn acteurs() -> Template {
    let texts = load_texts().unwrap();
    Template::render("acteurs", texts.acteurs)
}

#[get("/introduction")]
fn introduction() -> Template {
    let texts = load_texts().unwrap();
    Template::render("introduction", texts.introduction)
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
    let texts = load_texts().unwrap();
    Template::render("deuxieme_periode", texts.deuxieme_periode)
}

#[get("/interview")]
fn interview() -> Template {
    Template::render("interview", "")
}

#[get("/conclusion")]
fn conclusion() -> Template {
    let texts = load_texts().unwrap();
    Template::render("conclusion", texts.conclusion)
}

#[get("/bibliographie")]
fn bibliographie() -> Template {
    Template::render("bibliographie", "")
}

#[get("/a_propos")]
fn a_propos() -> Template {
    let texts = load_texts().unwrap();
    Template::render("a_propos", texts.a_propos)
}

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![
                accueil,
                acteurs,
                introduction,
                vue_ensemble,
                premiere_periode,
                deuxieme_periode,
                interview,
                conclusion,
                bibliographie,
                a_propos,
                textes,
            ],
        )
        .mount("/accueil", routes![accueil])
        .mount(
            "/textes",
            routes![
                acteurs_textes,
                premiere_periode_textes,
                deuxieme_periode_textes,
                conclusion_textes,
                a_propos_textes,
                introduction_textes,
            ],
        )
        .mount("/static", StaticFiles::from("static"))
        .attach(Template::fairing())
        .launch();
}
