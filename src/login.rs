use std::sync::{Arc, Mutex};
use std::time::SystemTime;
use diesel::insert_into;
use diesel::prelude::*;
use rocket::form::{Form, FromForm};
use rocket::http::CookieJar;
use rocket::response::content::RawHtml;
use rocket::response::Redirect;
use rocket::State;
use rocket_dyn_templates::{context, Template};
use crate::schema::users::{email, id, logged};
use crate::schema::users::dsl::users;

#[get("/login")]
pub fn login(cookies: &CookieJar<'_>) -> Result<Redirect, RawHtml<Template>> {
    match cookies.get_private("userid") {
        Some(_) => {
            Ok(Redirect::to("/home"))
        },
        None => {
            Err(RawHtml(Template::render("login", context! {})))
        }
    }
}

#[derive(FromForm, Clone)]
pub struct LoginInformation {
    pub username: String,
    pub password: String
}

#[derive(Queryable, Selectable, Clone)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserInformation {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub password: String,
    pub admin: bool,
    pub created: SystemTime,
    pub logged: SystemTime
}

pub fn update_log_time(conn: &mut PgConnection, user: String) {
    use crate::schema::users::dsl::username;

    match diesel::update(users)
        .filter(username.eq(user))
        .set(logged.eq(SystemTime::now()))
        .execute(conn) {
        Ok(_) => {},
        Err(_) => {
            eprintln!("failed to update login time!");
        }
    }
}

pub fn create_user(conn: &mut PgConnection, new_email: String, new_username: String, new_password: String) {
    use crate::schema::users::dsl::username;
    use crate::schema::users::dsl::password;

    match insert_into(users)
        .values(
            (
                email.eq(new_email),
                username.eq(new_username),
                password.eq(new_password)
            )
        ).execute(conn) {
        Ok(_) => {},
        Err(_) => {
            eprintln!("failed to create user!");
        }
    }
}

pub fn request_user_information(conn: &mut PgConnection, userid: i32) -> Option<UserInformation> {

    match users
        .filter(id.eq(userid))
        .limit(1)
        .select(UserInformation::as_select())
        .load(conn) {
        Ok(results) => {
            Some(results[0].clone())
        },
        Err(_) => {
            None
        }
    }
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

    //create_user(&mut conn, String::from("will"), String::from("test"));

    use crate::schema::users::dsl::users;
    let mut results = match users
        .filter(crate::schema::users::username.eq(login_information.clone().username))
        .limit(1)
        .select(UserInformation::as_select())
        .load(&mut *conn) {
        Ok(res) => res,
        Err(_) => return Redirect::to("/")
    };

    // no accounts found with that username
    if results.len() < 1 {
        // search using email as filter:
        results = match users
            .filter(crate::schema::users::email.eq(login_information.clone().username))
            .limit(1)
            .select(UserInformation::as_select())
            .load(&mut *conn) {
            Ok(res) => res,
            Err(_) => return Redirect::to("/")
        };
        
        if results.len() < 1 {
            return Redirect::to("/");
        }
    }

    let user = &results[0];

    if user.password == login_information.clone().password {
        // set user cookie because we logged in successfully
        update_log_time(&mut conn, login_information.clone().username);
        cookies.add_private(("userid", user.id.to_string()));
        Redirect::to("/home")
    } else {
        Redirect::to("/")
    }
}