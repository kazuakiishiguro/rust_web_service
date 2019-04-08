extern crate iron;
extern crate rustc_serialize;

use std::collections::HashMap;

use iron::status;
use iron::{Handler};
use iron::headers::ContentType;
use iron::prelude::*;

use rustc_serialize::json;

#[derive(RustcEncodable)]
pub struct Letter {
    title: String,
    message: String
}

#[derive(Debug)]
struct Router {
    routes: HashMap<String, Box<Handler>>
}

impl Router {
    fn new() -> Self {
        Router { routes: HashMap::new() }
    }
    fn add_route<H>(&mut self, path: String, handler: H) where H: Handler {
        self.routes.insert(path, Box::new(handler));
    }
}

fn json(_: &mut Request) -> IronResult<Response> {
    let letter = Letter {
        title: "aww".to_string(),
        message: "yeah".to_string()
    };
    let payload = json::encode(&letter).unwrap();
    Ok(Response::with((ContentType::json().0, status::Ok, payload)))
}

fn bad(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with(status::BadRequest))
}

fn main() {
    let mut router = Router::new();

    router.add_route("json".to_string(), json);

    router.add_route("error".to_string(), bad);

    let host = "localhost:3000";

    println!("binding on {}", host);
    Iron::new(router).http(host).unwrap();
}