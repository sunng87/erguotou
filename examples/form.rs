extern crate hyper;
extern crate rustc_serialize;
extern crate erguotou;
#[macro_use]
extern crate maplit;

use std::io::Read;

use hyper::client::{Client};
use erguotou::form::{ToForm, FormBody, FormParam};

fn main() {
    let data = btreemap! {
        "hello".to_owned() => "world?".to_owned(),
        "name".to_owned() => "Ning Sun".to_owned()
    };

    let client = Client::new();

    let mut form_data: FormParam = FormParam::from(&data as &ToForm);
    let mut resp = client.post("http://localhost:3000/form")
        .form(&mut form_data).send().unwrap();

    let mut resp_body = String::new();
    let _ = resp.read_to_string(&mut resp_body);

    println!("{}", resp_body);
}
