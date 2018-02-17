extern crate iron;
extern crate router;
mod lib;

use iron::prelude::*;
use iron::status;
use iron::mime::Mime;
use router::Router;
use std::io::Read;
use rustc_serialize::json;


fn main() {
    Iron::new(|r: &mut Request| {
        println!("{:?}", r);
        let content_type = "application/json".parse::<Mime>().unwrap();

        Ok(Response::with((content_type, status::Ok, hero)))
    }).http("localhost:7777").unwrap();
}
