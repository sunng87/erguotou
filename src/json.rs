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
}
