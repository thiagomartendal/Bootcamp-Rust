use rocket_dyn_templates::{Template, context};

#[get("/")]
pub fn index() -> Template {
    Template::render("home/index", context! {})
}

#[get("/route")]
pub fn route() -> &'static str {
    "Olá Mundo!"
}
