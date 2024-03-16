use std::sync::{Arc, Mutex};
use std::time::SystemTime;
use diesel::{insert_into, Queryable, Selectable};
use diesel::prelude::*;
use rocket::form::Form;
use rocket::http::CookieJar;
use rocket::State;
use rocket_dyn_templates::{context, Template};
use crate::login;

#[derive(Queryable, Selectable, Clone)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PostInformation {
    pub id: i32,
    pub created: SystemTime,
    pub author: String,
    pub content: String,
    pub description: String
}

#[derive(FromForm, Clone)]
pub struct PostCreationInformation {
    title: String,
    content: String,
    description: String
}

pub fn create_post(conn: &mut PgConnection,
                   new_title: String,
                   new_author: String,
                   new_description: String,
                   new_content: String) -> Result<usize, String>
{
    use crate::schema::posts::dsl::*;

    match insert_into(posts)
        .values(
            (
                title.eq(new_title),
                author.eq(new_author),
                description.eq(new_description),
                content.eq(new_content)
            )
        )
        .execute(conn) {
        Ok(res) => Ok(res),
        Err(err) => Err(err.to_string())
    }
}

#[post("/post", data = "<post_creation_information>")]
pub fn post_post(post_creation_information: Form<PostCreationInformation>,
                 cookies: &CookieJar<'_>,
                 database: &State<Arc<Mutex<PgConnection>>>) -> Template
{
    // check if we are logged in
    match cookies.get_private("userid") {
        Some(userid) => {
            let mut conn = match database.lock() {
                Ok(conn) => conn,
                Err(_) => return Template::render("post", context! {
                    result: "Failed to post."
                })
            };

            let user_information = match login::request_user_information(&mut *conn, userid.value().parse().unwrap()) {
                Some(user) => user,
                None => return Template::render("post", context! {
                    result: "Unable to retrieve user information"
                })
            };

            match create_post(&mut conn,
                              post_creation_information.clone().title,
                              user_information.username,
                              post_creation_information.clone().description,
                              post_creation_information.clone().content) {
                Ok(res) => {
                    Template::render("post", context! {
                        result: format!("Post created successfully `{}`", res)
                    })
                },
                Err(err) => {
                    Template::render("post", context! {
                        result: format!("Failed to create post! `{}`", err)
                    })
                }
            }
        },
        // You have to be logged in to make a post
        None => Template::render("post", context! {
            result: "You must be logged in to post."
        })
    }
}