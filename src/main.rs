extern crate iron;
extern crate mount;
extern crate router;


use iron::Iron;
use mount::Mount;


mod api;


fn main() {
    let mut middleware = Mount::new();
    middleware.mount("/v1", api::middleware());
    Iron::new(middleware).http("0.0.0.0:8080").unwrap();
}
