#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct AlpacaConfig {
    url: String,
    api: String,
    secret: String
}

pub fn parse_matches(matches: clap::ArgMatches) -> AlpacaConfig {

    let alpaca_url = matches.value_of("ALPACA_URL").unwrap().to_string();
    let alpaca_api = matches.value_of("API_KEY").unwrap().to_string();
    let alpaca_secret = matches.value_of("API_SECRET_KEY").unwrap().to_string();

    return AlpacaConfig {
        url: alpaca_url,
        api: alpaca_api,
        secret: alpaca_secret,
    }
}
