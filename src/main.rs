extern crate iron;

use iron::prelude::*;
use iron::mime::Mime;
use iron::status;
use std::env;

fn main() {
    let url = format!("0.0.0.0:{}", env::var("PORT").unwrap());

    Iron::new(|_: &mut Request| {
	    let instance = env::var("CF_INSTANCE_INDEX").unwrap();
	    let payload = format!("{{\"instance\": {} }}", instance);

        let content_type = "application/json".parse::<Mime>().unwrap();
        Ok(Response::with((content_type, status::Ok, payload)))
    }).http(&url[..]).unwrap();

    println!("Bound on {:?}", url);
}
