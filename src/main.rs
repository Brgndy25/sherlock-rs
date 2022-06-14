use reqwest;
use reqwest::StatusCode;
use tokio;
mod siteinfo;

fn main() { 
    let emptlist: Vec<String> = Vec::new();
    use siteinfo::list::list;
    let newlist = list(emptlist).clone();
    let client = reqwest::Client::new();
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(12)
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            for x in 0..newlist.len() {
                let response = client
                    .get(&newlist[x])
                    .send()
                    .await
                    .unwrap();
                match response.status() {
                    StatusCode::OK => println!("{}", newlist[x]),
                    _other => println!("NOT FOUND!"),
                };
            }
        })
}
