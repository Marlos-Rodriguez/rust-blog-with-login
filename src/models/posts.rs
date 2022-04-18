use super::super::schema::posts;

use chrono::NaiveDateTime;
use uuid::Uuid;

use diesel::pg::PgConnection;
use diesel::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, Queryable)]
pub struct Post {
    pub id: String,
    pub title: String,
    pub slug: String,
    pub user_id: String,
    pub body: String,
    pub published: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub id: &'a str,
    pub title: &'a str,
    pub slug: &'a str,
    pub body: &'a str,
    pub published: &'a bool,
    pub author_id: &'a str,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct NewPostHandler {
    pub title: String,
    pub body: String,
    pub published: bool,
}

impl Post {
    pub fn slug(title: &String) -> String {
        return title.replace(" ", "-").to_lowercase();
    }

    pub fn create_post(
        conn: &PgConnection,
        post: NewPostHandler,
        user_id: &String,
    ) -> Result<Post, diesel::result::Error> {
        let slug = Post::slug(&post.title);
        let id: String = Uuid::new_v4().to_string();

        let new_post = NewPost {
            id: &id,
            title: &post.title,
            slug: &slug,
            body: &post.body,
            published: &post.published,
            author_id: &user_id,
        };

        diesel::insert_into(posts::table)
            .values(new_post)
            .get_result(conn)
    }
}
