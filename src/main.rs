extern crate readrust;
extern crate reqwest;
extern crate serde_json;

fn main() {
    let client = reqwest::Client::builder()
        .proxy(reqwest::Proxy::http("http://localhost:3128").unwrap())
        .build().unwrap();

    let mut resp = client.get(readrust::URL).send().unwrap();

    assert!(resp.status().is_success());

    let data = resp.text().unwrap();

    let feed = serde_json::from_str::<readrust::Feed>(&data).unwrap();

    println!("feed = {:?}", feed);
}
