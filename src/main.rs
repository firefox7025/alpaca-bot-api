extern crate clap;
extern crate iron;
extern crate router;

#[macro_use]
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

use log::{info, trace, warn};

fn main() {

    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    info!("Starting service...");
    let mut router = Router::new();
    router.get("/", handler, "handler");
    router.get("/actions", action_handler::action_handler, "action_handler");
    router.get("/:query", query_handler, "query_handler");

    Iron::new(router).http("localhost:3000");

    fn handler(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "OK")))
    }

    fn query_handler(req: &mut Request) -> IronResult<Response> {
        let ref query = req.extensions.get::<Router>()
            .unwrap().find("query").unwrap_or("/");
        Ok(Response::with((status::Ok, *query)))
    }
}
