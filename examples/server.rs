extern crate iron;
extern crate router;
extern crate rustc_serialize;
extern crate bodyparser;
extern crate persistent;

use std::str::FromStr;
use iron::prelude::*;
use iron::status;
use iron::headers::ContentType;
use router::Router;
use persistent::Read;

fn json_handler(req: &mut Request) -> IronResult<Response> {
    let body = req.get::<bodyparser::Raw>().ok().unwrap().unwrap();

    let mut resp = Response::new();
    resp.headers.set(ContentType(FromStr::from_str("text/plain").unwrap()));
    Ok(resp.set(body).set(status::Ok))
}

fn main() {
    let mut router = Router::new();
    router.post("/json", json_handler);

    let mut chain = Chain::new(router);
    chain.link_before(Read::<bodyparser::MaxBodyLength>::one(10 * 1024 * 1024));
    println!("Server running at http://localhost:3000/");
    Iron::new(chain).http("localhost:3000").unwrap();
}
