use rustless::{ Nesting, Namespace };


pub fn ip(path: &str) -> Namespace {
    Namespace::build(path, |namespace| {
        namespace.get("", |endpoint| {
            endpoint.handle(|client, _params| {
                let ip_addr = client.request.remote_addr().ip();
                client.text(ip_addr.to_string())
            })
        });
    })
}
