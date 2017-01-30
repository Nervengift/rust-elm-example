extern crate iron;
extern crate mount;
extern crate router;
extern crate staticfile;

use std::sync::atomic::{AtomicBool, Ordering};
use std::path::Path;

use iron::prelude::*;
use iron::{Handler};
use iron::status;

use mount::Mount;
use router::Router;
use staticfile::Static;

// The Toggle struct simply implements a iron::Handler that toggles an internal boolean
// every time it is called and returns its new value as "JSON"
struct Toggle {
    state: AtomicBool,
}

impl Toggle {
    fn new() -> Toggle {
        let t = AtomicBool::new(false);
        Toggle {state: t}
    }
}

impl Handler for Toggle {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        let state = !self.state.fetch_xor(true, Ordering::SeqCst);
        let resp_txt = if state {"true"} else {"false"};
        let resp_txt = format!("{{\"status\" : {}}}", resp_txt);
        Ok(Response::with((status::Ok, resp_txt)))
    }
}


fn main() {
    let t = Toggle::new();

    // a router for all requests to the backend
    // obviously this is not really necessary for our example with only one endpoint
    let mut api_router = Router::new();
    api_router
        .get("/toggle", t);
    let mut mount = Mount::new();

    // the index.html compiled from the elm source is served as static file
    // another option would be to compile it into the binary using include_str!
    mount
        .mount("/api", api_router)
        .mount("/", Static::new(Path::new("assets/index.html")));

    // start the iron server
    println!("listening on localhost:8080");
    Iron::new(mount).http("localhost:8080").unwrap();
}
