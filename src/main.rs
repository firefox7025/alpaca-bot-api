extern crate clap;
extern crate iron;
extern crate router;

extern crate log;
extern crate log4rs;
extern crate reqwest;
extern crate serde_derive;

mod alpaca;
mod stock_actions;
mod action_handler;

use iron::status;
use iron::Iron;
use iron::Request;
use iron::Response;
use iron::IronResult;
use router::{Router};

use log::info;
use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Logger, Root};
use clap::{Arg, App};

fn main() {

    let stdout = ConsoleAppender::builder().build();

    let requests = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} - {m}{n}")))
        .build("log/requests.log")
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("requests", Box::new(requests)))
        .logger(Logger::builder()
                .appender("requests")
                .additive(false)
                .build("app::requests", LevelFilter::Info))
        .build(Root::builder().appender("stdout").build(LevelFilter::Info))
        .unwrap();

    let _handle = log4rs::init_config(config).unwrap();
    info!("Starting service...");


    let matches = App::new("Alpaca Bot Trader API")
                          .version("1.0")
                          .author("Alexander Montgomery <evaderxander@gmail.com>")
                          .about("Service that executes actions for making trades")
                          .arg(Arg::with_name("Alpaca Key")
                              .help("The api key to use for alpaca")
                               .required(false)
                               .index(1))
                          .arg(Arg::with_name("Alpaca url")
                              .help("The endpoint url to use for alpaca")
                              .required(false)
                              .index(2))
                          .get_matches();



    let mut router = Router::new();
    router.get("/", handler, "handler");
    router.get("/actions", action_handler::action_handler, "action_handler");
    router.get("/:query", query_handler, "query_handler");

    let server = Iron::new(router).http("localhost:3000");
    info!("server returned {:?}", server);

    fn handler(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "OK")))
    }

    fn query_handler(req: &mut Request) -> IronResult<Response> {
        let ref query = req.extensions.get::<Router>()
            .unwrap().find("query").unwrap_or("/");
        Ok(Response::with((status::Ok, *query)))
    }
}
