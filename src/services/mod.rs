extern crate diesel;
extern crate rocket;
use crate::models;
use crate::schema;
use diesel::delete;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use rand::Rng;
use rocket::response::status::BadRequest;
use rocket::response::{status::Created, Debug};
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{get, post, put};
use rocket_dyn_templates::{context, Template};
use std::{env, result};

// Models
use self::schema::posts::dsl::*;
use models::Post;

pub fn establish_connection_pg() -> PgConnection {
    dotenv().ok();

    let database_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    return PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
}

#[derive(Serialize, Deserialize)]
pub struct NewPost {
    title: String,
    body: String,
}

type Result<T, E = Debug<diesel::result::Error>> = result::Result<T, E>;

#[get("/")]
pub fn list() -> Template {
    let connection = &mut establish_connection_pg();
    let results = posts
        .filter(published.eq(true))
        .load::<Post>(connection)
        .expect("Error loading posts");
    Template::render("posts", context! {posts: &results, count: results.len()})
}

#[post("/post", format = "json", data = "<post>")]
pub fn create_post(post: Json<NewPost>) -> Result<Created<Json<NewPost>>> {
    let connection: &mut PgConnection = &mut establish_connection_pg();
    let post_id: i32 = rand::thread_rng().gen::<i32>();

    let new_post: Post = Post {
        id: post_id,
        title: post.title.to_string(),
        body: post.body.to_string(),
        published: true,
    };

    diesel::insert_into(posts)
        .values(&new_post)
        .execute(connection)
        .expect("Error saving new post");
    Ok(Created::new("/").body(post))
}

#[get("/post/<post_id>")]
pub fn get_post(post_id: i32) -> Result<Json<Post>, BadRequest<String>> {
    let connection: &mut PgConnection = &mut establish_connection_pg();
    let result = posts.find(post_id).first::<Post>(connection);
    match result {
        Ok(post) => Ok(Json(post)),
        Err(err) => Err(BadRequest(Some(format!(
            "Failed to retrieve post: {}",
            err
        )))),
    }
}

#[get("/posts")]
pub fn posts_as_json() -> Json<Vec<Post>> {
    let connection: &mut PgConnection = &mut establish_connection_pg();
    let results: Vec<Post> = posts
        .load::<Post>(connection)
        .expect("Error loading all posts");
    Json(results)
}

#[put("/post/<post_id>")]
pub fn delete_post(post_id: i32) -> Result<Json<bool>, BadRequest<String>> {
    let connection: &mut PgConnection = &mut establish_connection_pg();
    let deleted = delete(posts.filter(id.eq(post_id))).execute(connection);
    match deleted {
        Ok(deleted) => Ok(Json(deleted != 0)),
        Err(err) => Err(BadRequest(Some(format!("Failed to delete post: {}", err)))),
    }
}
