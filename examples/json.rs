extern crate hyper;
extern crate rustc_serialize;
extern crate erguotou;

use std::collections::BTreeMap;
use std::io::Read;

use hyper::client::{Client};
use rustc_serialize::json::{ToJson};
use erguotou::json::{JsonParam, JsonRPC};

fn main() {
    let mut data = BTreeMap::new();
    data.insert("hello".to_owned(), "world".to_owned());

    let mut client = Client::new();

    let mut json_param: JsonParam = JsonParam::from(&data as &ToJson);
    let mut resp = client.post("http://localhost:3000/json")
        .json(&mut json_param).send().unwrap();

    let mut resp_body = String::new();
    let _ = resp.read_to_string(&mut resp_body);

    println!("{}", resp_body);
}
