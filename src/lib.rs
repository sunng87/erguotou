#[deny(warnings)]

//! # Erguotou
//!
//! Erguotou is a set of high-level utilities for working with http requests. Currently, Erguotou
//! is fully based on [Hyper](https://hyperium.github.io/hyper/).
//!

extern crate hyper;
extern crate rustc_serialize;
extern crate url;

pub mod json;
pub mod form;
