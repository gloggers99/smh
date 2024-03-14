use rocket::http::CookieJar;
use rocket::response::Redirect;
use rocket_dyn_templates::{context, Template};

#[get("/home")]
pub fn home(cookies: &CookieJar<'_>) -> Result<Template, Redirect> {
    match cookies.get("userid") {
        Some(_) => {
            Ok(Template::render("home", context! {}))
        },
        None => {
            Err(Redirect::to("/"))
        }
    }
}