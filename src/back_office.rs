use crate::load_texts;
use rocket_contrib::templates::Template;

#[get("/")]
pub fn back_office() -> Template {
    Template::render("back_office", load_texts())
}
