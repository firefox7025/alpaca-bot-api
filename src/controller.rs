mod alpaca_web_client;

use iron::status;
use iron::Iron;
use iron::IronResult;
use iron::Request;
use iron::Response;

pub fn buy() {}

pub fn sell() {}

pub fn get_current(req: &mut Request) -> IronResult<Response> {
    let webresponse = alpaca_web_client::get_stocks();
    println!("{:?}, what are you", webresponse);
    match webresponse {
        Some(Any) => println!("proper returned from the server"),
        None => println!("An error occured will try again later"),
    }
    Ok(Response::with((status::Ok, "OK")))
}

pub fn pick_stocks() {}
