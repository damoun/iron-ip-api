extern crate iron;
extern crate rustless;


use iron::Iron;
use rustless::Application;


mod api;


fn main() {
    let app = Application::new(self::api::root());

    Iron::new(app).http("0.0.0.0:8080").unwrap();
}
