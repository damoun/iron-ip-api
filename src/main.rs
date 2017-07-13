extern crate iron;
extern crate mount;
extern crate router;
extern crate logger;
extern crate env_logger;


use iron::prelude::*;
use mount::Mount;
use logger::Logger;


mod api;


fn main() {
    env_logger::init().unwrap();

    let mut middleware = Mount::new();
    middleware.mount("/v1", api::middleware());

    let mut chain = Chain::new(middleware);
    let (logger_before, logger_after) = Logger::new(None);
    chain.link_before(logger_before);
    chain.link_after(logger_after);
    Iron::new(chain).http("0.0.0.0:8080").unwrap();
}
