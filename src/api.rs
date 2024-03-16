use std::sync::{Arc, Mutex};
use diesel::prelude::*;
use rocket::response::content::RawJson;
use rocket::State;
use crate::posts::PostInformation;

#[get("/api/post/<post_id>")]
pub fn get_post(database: &State<Arc<Mutex<PgConnection>>>, post_id: i32) -> Option<RawJson<String>> {
    use crate::schema::posts::dsl::*;

    let mut conn = match database.lock() {
        Ok(conn) => conn,
        Err(_) => return None
    };

    match posts
        .filter(id.eq(post_id))
        .limit(1)
        .select(PostInformation::as_select())
        .load(&mut *conn) {
        Ok(results) => {
            Some(
                RawJson(
                    match serde_json::to_string(&(results[0])) {
                        Ok(res) => res,
                        Err(_) => return None
                    }
                )
            )
        },
        Err(_) => {
            None
        }
    }
}
