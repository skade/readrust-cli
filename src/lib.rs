extern crate serde;
#[macro_use]
extern crate serde_derive;

pub static URL: &'static str = "http://readrust.net/rust2018/feed.json";

#[derive(Debug, Deserialize, Serialize)]
pub struct Feed {
    pub version: String,
    pub title: String,
    pub home_page_url: String,
    pub feed_url: String,
    pub description: String,
    pub author: Author,
    pub items: Vec<Item>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Author {
    name: String,
    url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Item {
    id: String,
    title: String,
    content_text: String,
    url: String,
    date_published: String,
    author: Author,
}