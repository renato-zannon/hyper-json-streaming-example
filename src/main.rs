#![feature(io)]

extern crate hyper;
extern crate "rustc-serialize" as rustc_serialize;

mod json_streamer;

use hyper::Client;

use std::old_io::BufferedReader;
use json_streamer::JsonObjectStreamer;

fn main() {
    let mut client = Client::new();
    let res = client.get("http://localhost:4567/").send().unwrap();

    for obj in BufferedReader::new(res).json_objects() {
        println!("object arrived: {}", obj);
    }
}
