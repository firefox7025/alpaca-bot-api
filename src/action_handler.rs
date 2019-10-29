extern crate rustc_serialize;
use crate::stock_actions::Action;

use iron::status;
use iron::IronResult;
use iron::Request;
use iron::Response;

use rustc_serialize::json;

#[derive(RustcDecodable, RustcEncodable)]
struct FinanicalAction {
    company: String,
    price: f64,
    action: Action,
}

pub fn action_handler(_req: &mut Request) -> IronResult<Response> {
    let mut vec = Vec::new();
    let action = FinanicalAction {
        company: "AMD".to_string(),
        price: 34.32,
        action: Action::BUY,
    };
    vec.push(action);
    let response = json::encode(&vec).unwrap();
    return Ok(Response::with((status::Ok, response)));
}
