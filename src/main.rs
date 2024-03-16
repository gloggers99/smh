mod login;
mod schema;
mod home;
mod posts;
mod api;

#[macro_use] extern crate rocket;

use std::env;
use std::sync::{Arc, Mutex};
use diesel::{Connection, PgConnection};
use dotenvy::dotenv;
use rocket::fs::{FileServer, relative};
use rocket::response::content::RawHtml;
use rocket_dyn_templates::{context, Template};

#[get("/")]
fn index() -> RawHtml<Template> {
    RawHtml(Template::render("index", context! {}))
}

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).unwrap()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        // enable templating
        .attach(
            Template::fairing()
        )
        // establish postgres connection & share it between functions
        .manage(Arc::new(Mutex::new(establish_connection())))
        .mount("/",
               routes![
                   index,             // - GET  /

                   login::login,      // - GET  /login
                   login::login_post, // - POST /login

                   posts::post_post,  // - POST /post

                   api::get_post,     // - GET  /api/post/<i32>

                   home::home         // - GET  /home
               ]
        )
        .mount("/static", FileServer::from(relative!("static")))
}