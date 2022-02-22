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
        let mut textes = load_texts().unwrap();

        textes.$name = $name.clone();

        let message = match fs::write("static/textes.ron", ron::to_string(&textes).unwrap()) {
            Ok(_) => Some("Succès".to_owned()),
            Err(e) => Some(format!("Erreur : {e}")),
        };

        Template::render("textes", TextesPage { message, textes })
    }};
}

textes_struct!(AccueilTextes, introduction);
textes_struct!(ActeursTextes, fda, ttandme, google, chercheurs);
textes_struct!(PremierePeriodeTextes, introduction, commercialisation);
textes_struct!(
    DeuxiemePeriodeTextes,
    introduction_1,
    introduction_2,
    introduction_3,
    conditions_1,
    conditions_2,
    enjeux_1,
    enjeux_2,
    enjeux_3,
    inquietudes_1,
    inquietudes_2,
    inquietudes_3
);

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
pub fn accueil_textes(accueil: Form<AccueilTextes>) -> Template {
    textes_function!(accueil)
}

#[get("/?textes=acteurs&<acteurs..>", rank = 2)]
pub fn acteurs_textes(acteurs: Form<ActeursTextes>) -> Template {
    textes_function!(acteurs)
}

#[get("/?textes=premiere_periode&<premiere_periode..>", rank = 3)]
pub fn premiere_periode_textes(premiere_periode: Form<PremierePeriodeTextes>) -> Template {
    textes_function!(premiere_periode)
}

#[get("/?textes=deuxieme_periode&<deuxieme_periode..>", rank = 4)]
pub fn deuxieme_periode_textes(deuxieme_periode: Form<DeuxiemePeriodeTextes>) -> Template {
    textes_function!(deuxieme_periode)
}
