extern crate readrust;
extern crate reqwest;
extern crate serde_json;
#[macro_use]
extern crate prettytable;

fn main() {
    let client = reqwest::Client::builder()
        .proxy(reqwest::Proxy::http("http://localhost:3128").unwrap())
        .build().unwrap();

    let mut resp = client.get(readrust::URL).send().unwrap();

    assert!(resp.status().is_success());

    let data = resp.text().unwrap();

    let feed = serde_json::from_str::<readrust::Feed>(&data).unwrap();
    let mut table = prettytable::Table::new();

    // Add a row per time
    table.add_row(row!["Title", "Author", "Link"]);

    // Print the table to stdout
    for item in feed.items {
        let title = if item.title.len() >= 50 {
                        &item.title[0..50]
                    } else {
                        &item.title
                    };

        table.add_row(row![title, item.author.name, item.url]);
    }

    table.printstd();
}
