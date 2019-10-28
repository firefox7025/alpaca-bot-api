use std::io::{self, Write};
use hyper::Client;
use hyper::rt::{self, Future, Stream};


pub fn get_stocks() {
    rt::run(rt::lazy(|| {
        let client = Client::new();
        let uri = "http://httpbin.org/ip".parse().unwrap();

        client
            .get(uri)
            .map(|res| {
                println!("Response: {}", res.status());
            })
            .map_err(|err| {
                println!("Error: {}", err);
            })
    }));
}
