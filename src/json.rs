use std::convert::From;

use hyper::client::{IntoUrl, RequestBuilder};
use hyper::header::ContentType;
use rustc_serialize::Encodable;
use rustc_serialize::json::{ToJson, Encoder};

pub struct JsonParam<'a> {
    value: &'a ToJson,
    serialized: String
}

impl<'a> From<&'a ToJson> for JsonParam<'a> {
    fn from(data_ref: &'a ToJson) -> JsonParam<'a> {
        JsonParam {
            value: data_ref,
            serialized: String::new()
        }
    }
}

pub trait JsonRPC<'a, U: IntoUrl> {
    fn json(mut self, data: &'a mut JsonParam<'a>) -> RequestBuilder<'a, U>;
}

impl<'a, U: IntoUrl> JsonRPC<'a, U> for RequestBuilder<'a, U> {
    fn json(self, data: &'a mut JsonParam<'a>) -> RequestBuilder<'a, U> {
        let json = data.value.to_json();
        {
            let mut encoder = Encoder::new(&mut data.serialized);
            let _ = json.encode(&mut encoder);
        }
        self.header(ContentType::json()).body(data.serialized.as_bytes())
    }
}

#[cfg(test)]
mod test {
    use std::collections::BTreeMap;
    use std::io::Read;

    use hyper::Ok;
    use hyper::client::{Client};
    use rustc_serialize::json::{ToJson};
    use ::json::{JsonParam, JsonRPC};

    #[test]
    fn test_json_wrapper() {
        let mut data = BTreeMap::new();
        data.insert("hello".to_owned(), "world".to_owned());

        let mut client = Client::new();

        let mut json_param: JsonParam = JsonParam::from(&data as &ToJson);
        let mut resp = client.post("http://localhost:8080")
            .json(&mut json_param).send().unwrap();

        let mut resp_body = String::new();
        let _ = resp.read_to_string(&mut resp_body);
        assert_eq!(resp.status, Ok);
        assert_eq!(resp_body, "world".to_string());
    }
}
