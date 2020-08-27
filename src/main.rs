extern crate diesel;
extern crate dotenv;
extern crate select;
extern crate newsinboundr_db;

use newsinboundr_db::db::models::SourceHttp;
use newsinboundr_db::db::schema::NewsSourceType;
use newsinboundr_db::create_news;
use dotenv::dotenv;
use std::env;
use std::vec::Vec;
use std::fmt::Error;
use futures::future::join_all;
use tokio::task::*;
use rss::Channel;
// use md5::{Md5, Digest};
use atom_syndication::Feed;
use chrono::{DateTime, Utc};
use select::document::Document;

//
//
// fn gen_color(source_url: &str) -> String {
//     let mut hasher = Md5::new();
//     hasher.update(source_url);
//     match String::from_utf8(hasher.finalize().to_vec()) {
//         Ok(col) => col,
//         _ => "aaaaaa".to_string()
//     }
// }
//

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
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let connection_pool = newsinboundr_db::establish_connection(database_url);
    if let Some(list) = newsinboundr_db::get_atom_sources(connection_pool.get().unwrap()) {
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

                            let connection = pool.get().unwrap();

                            let _ = create_news(&connection,
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

    if let Some(list) = newsinboundr_db::get_rss_sources(connection_pool.get().unwrap()) {
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

                        for i in ch.items() {
                            let connection = pool.get().unwrap();

                            let guid = match i.guid() {
                                Some(guid) => guid.value().to_string(),
                                None => {
                                    let uuid = uuid::Uuid::new_v4();
                                    format!("{}", uuid)
                                }
                            };

                            let published = match i.pub_date() {
                                Some(pub_date) => {
                                    match DateTime::parse_from_rfc2822(pub_date) {
                                        Ok(pub_date) => pub_date.naive_utc(),
                                        Err(_) => Utc::now().naive_utc()
                                    }
                                },
                                None => Utc::now().naive_utc()
                            };

                            let mut description_used = false;
                            let title = match i.title() {
                                Some(title) => title,
                                None => {
                                    match i.description() {
                                        Some(description) => {
                                            description_used = true;
                                            description
                                        },
                                        None => ""
                                    }
                                }
                            };

                            let content = match (i.content(), description_used) {
                                (Some(cntnt), false) => {
                                    match i.description() {
                                        Some(description) => {
                                            let mut c = format!("{}<br><br>", cntnt.clone());
                                            c.push_str(description.clone());
                                            let r = c.to_owned();
                                            r
                                        },
                                        None => cntnt.to_string()
                                    }
                                },
                                (Some(cntnt), true) => cntnt.to_string(),
                                (None, false) => {
                                    match i.description() {
                                        Some(description) => {
                                            description.to_string()
                                        },
                                        None => "".to_string()
                                    }
                                },
                                (None, true) => "".to_string()
                            };

                            let link = match i.link() {
                                Some(link) => link,
                                None => ""
                            };


                            let _ = create_news(&connection,
                                        &guid,
                                        &NewsSourceType::Rss,
                                        &s_id,
                                        &published,
                                        title,
                                        &content,
                                        link,
                                        s_color.as_ref()
                            );
                        }

                    },
                    Err(e) => {
                        println!("Could not fetch rss feed '{}'. Error: '{}'", s_url, e.to_string())

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
