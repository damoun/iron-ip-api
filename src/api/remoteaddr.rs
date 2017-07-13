use router::Router;
use iron::prelude::*;
use iron::status;


pub fn router() -> Router {
    let mut router = Router::new();
    router.get("", retrieve_ip, "ip");

    fn retrieve_ip(request: &mut Request) -> IronResult<Response> {
        let ip_addr = request.remote_addr.ip();
        Ok(Response::with((status::Ok, ip_addr.to_string())))
    };
 
    router
}
