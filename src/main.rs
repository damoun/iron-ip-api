extern crate rustless;
extern crate iron;

use iron::Iron;
use rustless::{ Application, Api, Nesting, Versioning };

fn main() {
   let  api = Api::build(|api| {
   	api.version("v1", Versioning::Path);

	api.mount(Api::build(|ip_api| {
	    ip_api.namespace("ip", |ip_ns| {
	        ip_ns.get("", |endpoint| {
		    endpoint.handle(|client, _params| {
		    	let ip = client.request.remote_addr().ip().to_string();
			client.text(ip)
		    })
		})
	    })
	}))
   });

   let app = Application::new(api);

   Iron::new(app).http("127.0.0.1:8080").unwrap();
}
