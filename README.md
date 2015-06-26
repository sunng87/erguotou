# Erguotou

[![Build Status](https://travis-ci.org/sunng87/erguotou.svg?branch=master)](https://travis-ci.org/sunng87/erguotou)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![crates.io](http://meritbadge.herokuapp.com/erguotou)](https://crates.io/crates/erguotou)

Erguotou (the library) is a set of utilities for writing http client
with [hyper](https://github.com/hyperium/hyper).

[Erguotou](http://en.wikipedia.org/wiki/Erguotou) (the name) is a kind of Chinese wine (Bai-jiu).

This library is working in progress. Any suggestion are welcome on API
design.


## Cargo

```toml
[dependencies]
erguotou = "*"
```

## Usage

### JsonRPC

Demo code

```rust
use std::collections::BTreeMap;
use hyper::client::Client;
use rustc_serialize::json::ToJson;
use erguotou::json::{JsonParam, JsonRPC};

// the data you are going to send
// it can be anything of rustc_serialize::json::ToJson
let mut data = BTreeMap::new();
data.insert("hello".to_owned(), "world".to_owned());

// the hyper client
let client = Client::new();

// construct JsonParam from data
let mut json_param: JsonParam = JsonParam::from(&data as &ToJson);
// the hyper request builder calls, no need for body() and header()
// a single json() call to fill them all
let mut resp = client.post("http://localhost:8080")
    .json(&mut json_param).send().unwrap();

```

### Form Post

Demo code

```rust
use std::collections::BTreeMap;
use hyper::client::Client;
use erguotou::form::{ToForm, FormParam, FormBody};

// the data you are going to send
// it can be anything implements ToForm.
// By default, we have BTreeMap built-in for ToForm
let mut data = BTreeMap::new();
data.insert("name".to_owned(), "Ning Sun".to_owned());

// the hyper client
let client = Client::new();

// construct JsonParam from data
let mut form_data: FormParam = FormParam::from(&data as &ToForm);
// the hyper request builder calls, no need for body() and header()
// a single form() call to fill them all
let mut resp = client.post("http://localhost:8080")
    .form(&mut form_data).send().unwrap();

```

## License

MIT
