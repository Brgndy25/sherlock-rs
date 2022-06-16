use reqwest;
use reqwest::StatusCode;
use tokio;
mod siteinfo;
use siteinfo::list::list;
 
fn main() { 
    let mut emptlist = Vec::new();
    emptlist = list(&mut emptlist);
    let client = reqwest::Client::new();
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(12)
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            for x in 0..emptlist.len() {
                let response = client
                    .get(&emptlist[x])
                    .send()
                    .await
                    .unwrap();
                match response.status() {
                    StatusCode::OK => println!("{}", emptlist[x]),
                    _other => println!("NOT FOUND!"),
                };
            }
        })
}


