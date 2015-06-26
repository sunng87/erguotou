extern crate iron;
extern crate router;
extern crate rustc_serialize;
extern crate bodyparser;
extern crate persistent;
extern crate urlencoded;

use std::str::FromStr;
use iron::prelude::*;
use iron::status;
use iron::headers::ContentType;
use router::Router;
use persistent::Read;
use urlencoded::UrlEncodedBody;

fn json_handler(req: &mut Request) -> IronResult<Response> {
    let body = req.get::<bodyparser::Raw>().ok().unwrap().unwrap();

    let mut resp = Response::new();
    resp.headers.set(ContentType(FromStr::from_str("text/plain").unwrap()));
    Ok(resp.set(body).set(status::Ok))
}

fn form_handler(req: &mut Request) -> IronResult<Response> {
    let mut resp = Response::new();
    match req.get_ref::<UrlEncodedBody>() {
        Ok(ref hashmap) => {
            let body = format!("{:?}", hashmap);
            resp.headers.set(ContentType(FromStr::from_str("text/plain").unwrap()));
            Ok(resp.set(body).set(status::Ok))
        },
        Err(ref e) => {
            println!("{:?}", e);
            Ok(resp.set(status::BadRequest))
        }
    }
}

fn main() {
    let mut router = Router::new();
    router.post("/json", json_handler);
    router.post("/form", form_handler);

    let mut chain = Chain::new(router);
    chain.link_before(Read::<bodyparser::MaxBodyLength>::one(10 * 1024 * 1024));
    println!("Server running at http://localhost:3000/");
    Iron::new(chain).http("localhost:3000").unwrap();
}
