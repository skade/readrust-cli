extern crate readrust;
extern crate reqwest;
extern crate serde_json;

fn main() {
    let mut resp = reqwest::get(readrust::URL).unwrap();

    assert!(resp.status().is_success());

    let data = resp.text().unwrap();

    let feed = serde_json::from_str::<readrust::Feed>(&data).unwrap();

    println!("feed = {:?}", feed);
}
