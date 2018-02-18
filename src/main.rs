extern crate iron;
extern crate router;
extern crate rustc_serialize;

use iron::prelude::*;
use iron::status;
use iron::mime::Mime;
use router::Router;
use std::io::Read;
use rustc_serialize::json;

mod lib;

fn main() {
    Iron::new(|r: &mut Request| {
        println!("{:?}", r);
        let content_type = "application/json".parse::<Mime>().unwrap();

        Ok(Response::with((content_type, status::Ok, "")))
    }).http("localhost:7777").unwrap();
}
