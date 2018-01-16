extern crate readrust;
extern crate clap;
extern crate reqwest;
extern crate serde_json;
#[macro_use]
extern crate prettytable;

use clap::{Arg, App, SubCommand};
use readrust::{Feed, URL, Item};

fn get_feed() -> Feed {
    let client = reqwest::Client::new();

    let mut resp = client.get(URL).send().unwrap();

    assert!(resp.status().is_success());

    let data = resp.text().unwrap();

    serde_json::from_str::<readrust::Feed>(&data).unwrap()
}

fn print_count(feed: &Feed) {
    println!("Number of posts: {}", feed.items.len());
}

fn print_feed_table<'a, I: Iterator<Item=&'a Item>>(items: I) {
    let mut table = prettytable::Table::new();

    table.add_row(row!["Title", "Author", "Link"]);

    for item in items {
        let title = if item.title.len() >= 50 {
                        &item.title[0..50]
                    } else {
                        &item.title
                    };

        table.add_row(row![title, item.author.name, item.url]);
    }

    table.printstd();
}

fn main() {
    let matches = App::new("readrust")
                          .version("0.1")
                          .author("Florian G. <florian.gilcher@asquera.de>")
                          .about("Reads readrust.net")
                          .args_from_usage(
                              "-n, --number=[NUMBER] 'Number of posts'
                              -c, --count            'Just counts the number'")
                          .get_matches();

    let feed = get_feed();

    if matches.is_present("count") {
        print_count(&feed);
    } else {
        let iter = feed.items.iter();

        if let Some(string) = matches.value_of("number") {
            let number = string.parse().unwrap();
            print_feed_table(iter.take(number))
        } else {
            print_feed_table(iter)
        }
    }
}
