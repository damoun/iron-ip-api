use router::Router;
use iron::prelude::*;
use iron::status;
use super::super::headers;


pub fn router() -> Router {
    let mut router = Router::new();
    router.get("", retrieve_ip, "ip");

    fn retrieve_ip(request: &mut Request) -> IronResult<Response> {
        let addr = match request.headers.has::<headers::XForwardedFor>() {
            true => request.headers.get::<headers::XForwardedFor>().unwrap().addr.ip(),
            false => request.remote_addr.ip()
        };
        Ok(Response::with((status::Ok, addr.to_string())))
    };
 
    router
}
