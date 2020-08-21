use std::vec::Vec;
use crate::db::schema::NewsSourceType;
use rss::{Channel, Item};
use super::schema::news;


#[derive(Queryable, Debug, Clone)]
pub struct SourceAtom  {
    pub id: i32,
    pub url: Option<String>,
    pub display_name: Option<String>,
    pub color: Option<String>,
}

#[derive(Queryable, Debug, Clone)]
pub struct SourceRss  {
    pub id: i32,
    pub url: Option<String>,
    pub display_name: Option<String>,
    pub color: Option<String>,
}

#[derive(Debug)]
pub struct SourceHttp {
    pub url: Option<String>,
    pub display_name: Option<String>,
    pub color: Option<String>,
    pub news_source_type: NewsSourceType,
    pub news_source_id: i32
}

impl From<SourceAtom> for SourceHttp {
    fn from(item: SourceAtom) -> Self {
        SourceHttp {
            url: item.url,
            display_name: item.display_name,
            color: item.color,
            news_source_type: NewsSourceType::Atom,
            news_source_id: item.id,
        }
    }
}

impl From<SourceRss> for SourceHttp {
    fn from(item: SourceRss) -> Self {
        SourceHttp {
            url: item.url,
            display_name: item.display_name,
            color: item.color,
            news_source_type: NewsSourceType::Rss,
            news_source_id: item.id,
        }
    }
}

#[derive(Queryable, Debug)]
pub struct News {
    pub id: Option<i32>,
    pub guid: Option<String>,
    pub news_source_type: Option<NewsSourceType>,
    pub news_source_id: Option<i32>,
    pub insert_date: Option<chrono::NaiveDateTime>,
    pub title: Option<i32>,
    pub content: Option<String>,
    pub link: Option<String>,
    pub color: Option<String>,
}

#[derive(Insertable)]
#[table_name="news"]
pub struct NewNews <'a>{
    pub guid: &'a str,
    pub source_type: &'a NewsSourceType,
    pub source_id: &'a i32,
    pub insert_date: &'a chrono::NaiveDateTime,
    pub title: &'a str,
    pub content: &'a str,
    pub link: &'a str,
    pub color: &'a str,
}

