#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate select;

mod db;

use db::models::{SourceAtom, SourceRss, SourceHttp};
use db::schema::source_atom::dsl::*;
use db::schema::source_rss::dsl::*;
use diesel::pg::PgConnection;
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};
use tokio_diesel::*;
use dotenv::dotenv;
use std::env;
use std::vec::Vec;
use std::fmt::Error;
use futures::future::join_all;
use tokio::task::*;
use rss::Channel;
use md5::{Md5, Digest};
use atom_syndication::{Feed, Content};
use crate::db::models::News;
use crate::db::lib::create_news;
use crate::db::schema::NewsSourceType;
use chrono::{DateTime, Utc};
use select::document::Document;
use diesel::r2d2::PooledConnection;


pub fn establish_connection() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager =
        ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager).expect("Error building pool")
}

fn gen_color(source_url: &str) -> String {
    let mut hasher = Md5::new();
    hasher.update(source_url);
    match String::from_utf8(hasher.finalize().to_vec()) {
        Ok(col) => col,
        _ => "aaaaaa".to_string()
    }
}

fn get_atom_sources(connection: PooledConnection<ConnectionManager<PgConnection>>) -> Option<Vec<SourceAtom>> {
    return None;
    let results = source_atom
        .load(&connection)
        .expect("Error loading posts");

    if results.is_empty() {
        None
    } else {
        Some(results)
    }
}

fn get_rss_sources(connection: PooledConnection<ConnectionManager<PgConnection>>) -> Option<Vec<SourceRss>> {

    let results = source_rss
        .load(&connection)
        .expect("Error loading posts");

    if results.is_empty() {
        None
    } else {
        Some(results)
    }
}

async fn fetch_http(item: &SourceHttp) -> Option<String> {
    if let Some(feed_url) = item.url.clone() {
        match reqwest::get(&feed_url).await {
            Ok(resp) => {
                match resp.text().await {
                    Ok(text) => Some(text),
                    Err(e) => {
                        println!("Could not fetch text for url '{}': {}", feed_url, e.to_string());
                        None
                    }
                }
            },
            Err(e) => {
                println!("Could not fetch feed from url {}, reason '{}'", feed_url, e.to_string());
                None
            }
        }
    } else {
        None
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut tasks: Vec<JoinHandle<Result<(),()>>> = vec![];
    let connection_pool = establish_connection();
    if let Some(list) = get_atom_sources(connection_pool.get().unwrap()) {
        for s in list {
            let pool = connection_pool.clone();
            if s.url.is_none() {
                continue;
            }

            tasks.push(tokio::spawn(async move {
                let s_url = if s.url.is_some() {
                    s.url.clone().unwrap()
                } else {
                    "".to_string()
                };
                let s_id = s.id;
                let s_color = if s.color.is_some() {
                    s.color.clone().unwrap()
                } else {
                    "aaaaaa".to_string()
                };
                let raw = fetch_http(&s.into()).await;
                if raw.is_none() {
                    ()
                }
                match raw.unwrap().parse::<Feed>() {
                    Ok(ch) => {
                        for i in &ch.entries {
                            let mut content = match &i.content {
                                Some(c) => {
                                    match &c.content_type {
                                        Some(ct) if ct.eq("html") || ct.eq("xhtml") => {
                                            let d = Document::from(c.value.as_ref().unwrap().as_ref());
                                            let body = d.find(select::predicate::Name("body")).next();
                                            if body.is_some() {
                                                body.unwrap().text()
                                            } else {
                                                "".to_string()
                                            }
                                        },
                                        Some(_mime) if c.value.is_some() => {
                                            c.value.clone().unwrap()
                                        },
                                        _ => {
                                            "".to_string()
                                        }
                                    }
                                },
                                _ => {
                                    "".to_string()
                                }
                            };

                            if content == "" && i.summary.as_ref().is_some() {
                                content = i.summary.as_ref().unwrap().to_string();
                            }

                            let link = if i.links().len() > 0 {
                                i.links().get(0).unwrap().href()
                            } else {
                                ""
                            };

                            let published = if i.published.is_some() {
                                i.published.unwrap().naive_utc()
                            } else {
                                Utc::now().naive_utc()
                            };

                            let mut connection = pool.get().unwrap();

                            create_news(&connection,
                                        i.id.as_str(),
                                        &NewsSourceType::Atom,
                                        &s_id,
                                        &published,
                                        i.title(),
                                        content.as_ref(),
                                        link.as_ref(),
                                        s_color.as_ref()
                            );
                        }
                    }
                    Err(e) => {
                        println!("Could not fetch atom feed '{}'. Error: '{}'", s_url, e.to_string())

                    }
                }
                print!(".");
                std::result::Result::Ok(())
            }));
        }
    }

    if let Some(list) = get_rss_sources(connection_pool.get().unwrap()) {
        for s in list {
            let pool = connection_pool.clone();
            if s.url.is_none() {
                continue;
            }

            tasks.push(tokio::spawn(async move {
                let s_url = if s.url.is_some() {
                    s.url.clone().unwrap()
                } else {
                    "".to_string()
                };
                let s_id = s.id;
                let s_color = if s.color.is_some() {
                    s.color.clone().unwrap()
                } else {
                    "aaaaaa".to_string()
                };
                match Channel::from_url(&s_url) {
                    Ok(ch) => {

                        println!("{:?}", ch);

                        //     let mut connection = pool.get().unwrap();
                        //
                        //     create_news(&connection,
                        //                 i.id.as_str(),
                        //                 &NewsSourceType::Atom,
                        //                 &s_id,
                        //                 &published,
                        //                 i.title(),
                        //                 content.as_ref(),
                        //                 link.as_ref(),
                        //                 s_color.as_ref()
                        //     );
                        // }
                    }
                    Err(e) => {
                        println!("Could not fetch atom feed '{}'. Error: '{}'", s_url, e.to_string())

                    }
                }
                print!(".");
                std::result::Result::Ok(())
            }));
        }
    }

    join_all(tasks).await;
    Ok(())
}
