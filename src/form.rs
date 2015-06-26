use std::convert::{From};
use std::collections::{BTreeMap};

use url::form_urlencoded::serialize;
use hyper::client::{IntoUrl, RequestBuilder};
use hyper::header::ContentType;

pub trait ToForm {
    fn to_form(&self) -> String;
}

impl<K, V> ToForm for BTreeMap<K, V> where K: AsRef<str>, V: AsRef<str> {
    fn to_form(&self) -> String {
        serialize(self.iter())
    }
}

pub struct FormParam<'a> {
    value: &'a ToForm,
    encoded: String
}

impl<'a> From<&'a ToForm> for FormParam<'a> {
    fn from(data_ref: &'a ToForm) -> FormParam<'a> {
        FormParam {
            value: data_ref,
            encoded: String::new()
        }
    }
}

pub trait FormBody<'a, U: IntoUrl> {
    fn form(mut self, data: &'a mut FormParam<'a>) -> RequestBuilder<'a, U>;
}

impl<'a, U: IntoUrl> FormBody<'a, U> for RequestBuilder<'a, U> {
    fn form(self, data: &'a mut FormParam<'a>) -> RequestBuilder<'a, U> {
        let serialized = data.value.to_form();
        data.encoded.push_str(serialized.as_ref());

        self.header(ContentType::form_url_encoded()).body(data.encoded.as_bytes())
    }
}
