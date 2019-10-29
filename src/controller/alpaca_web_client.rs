use hyper::rt::{self, Future, Stream};
use hyper::Client;
use hyper_tls::HttpsConnector;
use std::io::{self, Write};

pub fn get_stocks() -> Option<hyper::Response<hyper::Body>> {
    let mut response = None;
    rt::run(rt::lazy(|| {
        let https = HttpsConnector::new(4).expect("TLS initialization failed");
        let client = Client::builder().build::<_, hyper::Body>(https);
        let uri = "https://paper-api.alpaca.markets".parse().unwrap();

        client
            .get(uri)
            .and_then(|res| {
                println!("Response: {}", res.status());
                res.into_body()
                    // Body is a stream, so as each chunk arrives...
                    .for_each(|chunk| {
                        io::stdout()
                            .write_all(&chunk)
                            .map_err(|e| panic!("example expects stdout is open, error={}", e))
                    })
            })
            .map_err(|err| {
                println!("Error: {}", err);
            })
    }));
    return response;
}
