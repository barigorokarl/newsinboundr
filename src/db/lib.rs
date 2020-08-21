use diesel::{PgConnection, RunQueryDsl};
use diesel::result::Error;
use crate::db::models::NewNews;
use crate::db::schema::news;
use crate::db::schema::NewsSourceType;
use diesel::r2d2::{Pool, ConnectionManager, PooledConnection};

pub fn create_news<'a>(conn: &PooledConnection<ConnectionManager<PgConnection>>,
                       guid: &'a str,
                       source_type: &'a NewsSourceType,
                       source_id: &'a i32,
                       insert_date: &'a chrono::NaiveDateTime,
                       title: &'a str,
                       content: &'a str,
                       link: &'a str,
                       color: &'a str) -> Result<usize, Error> {

    let new_news = NewNews{
        guid,
        source_type,
        source_id,
        insert_date,
        title,
        content,
        link,
        color
    };

    diesel::insert_into(news::table)
        .values(&new_news)
        .execute(conn)
}