use std::sync::{Arc, Mutex};
use std::time::SystemTime;
use diesel::prelude::*;
use rocket::form::{Form, FromForm};
use rocket::http::CookieJar;
use rocket::response::content::RawHtml;
use rocket::response::Redirect;
use rocket::State;
use rocket_dyn_templates::{context, Template};

#[get("/login")]
pub fn login() -> RawHtml<Template> {
    RawHtml(Template::render("login", context! {}))
}

#[derive(FromForm, Clone)]
pub struct LoginInformation {
    pub username: String,
    pub password: String
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserInformation {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub admin: bool,
    pub created: SystemTime,
    pub logged: SystemTime
}

#[post("/login", data = "<login_information>")]
pub fn login_post(login_information: Form<LoginInformation>,
                  cookies: &CookieJar<'_>,
                  database: &State<Arc<Mutex<PgConnection>>>) -> Redirect 
{
    let mut conn = match database.lock() {
        Ok(conn) => conn,
        Err(_) => return Redirect::to("/")
    };

    use crate::schema::users::dsl::users;
    let results = users
        .filter(crate::schema::users::username.eq(login_information.clone().username))
        .limit(1)
        .select(UserInformation::as_select())
        .load(&mut *conn)
        .unwrap();

    // no accounts found with that username
    if results.len() < 1 {
        return Redirect::to("/");
    }

    let user = &results[0];

    if user.password == login_information.clone().password {
        // set user cookie because we logged in successfully
        // TODO: use private cookies in the future.
        cookies.add(("userid", user.id.to_string()));
        Redirect::to("/home")
    } else {
        Redirect::to("/")
    }
}