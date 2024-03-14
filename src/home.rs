use std::sync::{Arc, Mutex};
use diesel::PgConnection;
use rocket::http::CookieJar;
use rocket::response::Redirect;
use rocket::State;
use rocket_dyn_templates::{context, Template};
use crate::login;

#[get("/home")]
pub fn home(cookies: &CookieJar<'_>, database: &State<Arc<Mutex<PgConnection>>>) -> Result<Template, Redirect> {
    // make sure user is logged in before allowing entry
    match cookies.get_private("userid") {
        Some(cookie) => {
            // get username:
            let mut conn = match database.lock() {
                Ok(conn) => conn,
                Err(_) => return Err(Redirect::to("/"))
            };

            let username = match login::request_user_information(&mut *conn, cookie.value().parse().unwrap()) {
                Some(user_information) => user_information.username,
                None => return Err(Redirect::to("/"))
            };

            Ok(Template::render("home", context! {
                username: username
            }))
        },
        None => {
            Err(Redirect::to("/"))
        }
    }
}