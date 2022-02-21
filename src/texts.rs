use rocket::request::FromForm;
use rocket::{request::Form, response::Redirect};
use rocket_contrib::templates::Template;
use serde::{Deserialize, Serialize};
use std::{error::Error, fs};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Textes {
    #[serde(default)]
    pub accueil: AccueilTextes,
    #[serde(default)]
    pub premiere_periode: PremierePeriodeTextes,
    #[serde(default)]
    pub acteurs: ActeursTextes,
    #[serde(default)]
    pub deuxieme_periode: DeuxiemePeriodeTextes,
}

macro_rules! textes_struct {
    ($name:ident,$($field:ident),*) => {
        #[derive(Debug, Clone, Serialize, Deserialize, Default, FromForm)]
        pub struct $name {
            $(
                #[serde(default)]
                pub $field: String,
            )*
        }
    };
}

macro_rules! textes_function {
    ($name:ident) => {{
        let mut textes = match load_texts() {
            Ok(t) => t,
            Err(e) => return Redirect::to(uri!(textes: format!("Erreur : {e}"))),
        };

        textes.$name = $name.clone();

        match fs::write("static/textes.ron", ron::to_string(&textes).unwrap()) {
            Ok(_) => Redirect::to(uri!(textes: "Succès")),
            Err(e) => Redirect::to(uri!(textes: format!("Erreur : {e}"))),
        }
    }};
}

textes_struct!(AccueilTextes, introduction);
textes_struct!(ActeursTextes, fda, ttandme, google, chercheurs);
textes_struct!(PremierePeriodeTextes, introduction, commercialisation);
textes_struct!(DeuxiemePeriodeTextes, introduction);

#[derive(Debug, Clone, Serialize, Default)]
struct TextesPage {
    message: Option<String>,
    textes: Textes,
}

pub fn load_texts() -> Result<Textes, Box<dyn Error>> {
    let file = fs::read_to_string("static/textes.ron")?;

    Ok(ron::from_str(&file)?)
}

#[get("/textes?<message>", rank = 5)]
pub fn textes(message: Option<String>) -> Template {
    println!("HERE");
    Template::render(
        "textes",
        TextesPage {
            message,
            textes: load_texts().unwrap(),
        },
    )
}

#[get("/?textes=accueil&<accueil..>", rank = 1)]
pub fn accueil_textes(accueil: Form<AccueilTextes>) -> Redirect {
    textes_function!(accueil)
}

#[get("/?textes=acteurs&<acteurs..>", rank = 2)]
pub fn acteurs_textes(acteurs: Form<ActeursTextes>) -> Redirect {
    textes_function!(acteurs)
}

#[get("/?textes=premiere_periode&<premiere_periode..>", rank = 3)]
pub fn premiere_periode_textes(premiere_periode: Form<PremierePeriodeTextes>) -> Redirect {
    textes_function!(premiere_periode)
}

#[get("/?textes=deuxieme_periode&<deuxieme_periode..>", rank = 4)]
pub fn deuxieme_periode_textes(deuxieme_periode: Form<DeuxiemePeriodeTextes>) -> Redirect {
    textes_function!(deuxieme_periode)
}
