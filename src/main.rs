extern crate clap;
extern crate iron;
extern crate router;
extern crate hyper;
extern crate hyper_tls;
extern crate log;
extern crate log4rs;
extern crate serde_derive;
extern crate mount;

mod action_handler;
mod stock_actions;
mod match_parser;
mod controller;

use iron::status;
use iron::Iron;
use iron::IronResult;
use iron::Request;
use iron::Response;
use iron::Chain;
use router::Router;

use mount::Mount;

use clap::{App, Arg};
use log::info;
use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Logger, Root};
use log4rs::encode::pattern::PatternEncoder;


fn main() {
    let _handle = log4rs::init_config(logging_config()).unwrap();
    info!("Starting service...");

    let _matches = App::new("Alpaca Bot Trader API")
        .version("1.0")
        .author("Alexander Montgomery <evaderxander@gmail.com>")
        .about("Service that executes actions for making trades")
        .arg(
            Arg::with_name("API_KEY")
                .long("api_key")
                .short("a")
                .help("The api key for alpaca")
                .required(true)
                .takes_value(true)
                .index(1),
        )
        .arg(
            Arg::with_name("API_SECRET_KEY")
                .long("secret_key")
                .short("s")
                .help("The api secret key to use for alpaca")
                .required(true)
                .takes_value(true)
                .index(2),
        )
        .arg(
            Arg::with_name("ALPACA_URL")
                .help("The endpoint url to use for alpaca defaults to dev")
                .short("u")
                .long("url")
                .required(false)
                .takes_value(true)
                .required(true)
                .index(3),
        )
        .get_matches();
    let values = match_parser::parse_matches(_matches);

    let router = router!(
        get_stocks: get "/getStocks" => controller::router(),
        health: get "/health" => controller::health_route()
    );

    let mut mount = Mount::new();
    mount.mount("/", router);
    let mut chain = Chain::new(mount);
    chain.link("creds", values);

    let server = Iron::new(chain).http("localhost:3000");
    info!("server returned {:?}", server);
}


fn logging_config() -> log4rs::config::Config {
    let stdout = ConsoleAppender::builder().build();
    return Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .logger(
            Logger::builder()
                .appender("requests")
                .additive(false)
                .build("app::requests", LevelFilter::Info),
        )
        .build(Root::builder().appender("stdout").build(LevelFilter::Info))
        .unwrap();
}
