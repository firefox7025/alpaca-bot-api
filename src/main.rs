extern crate iron;
extern crate router;


use iron::prelude::*;
use iron::status;
use iron::Iron;
use iron::Request;
use iron::Response;
use iron::IronResult;
use router::{Router};

fn main() {
    let mut router = Router::new();
    router.get("/", handler, "handler");
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
