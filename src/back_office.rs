use crate::Texts;
use rocket_contrib::templates::Template;
use std::collections::HashMap;

#[get("/")]
pub fn back_office() -> Template {
    let context: HashMap<&str, Texts> = [(
        "texts",
        Texts {
            homepage_1: String::from("hello"),
            homepage_2: String::from("hello2"),
        },
    )]
    .into_iter()
    .collect();

    Template::render("back_office", context)
}
