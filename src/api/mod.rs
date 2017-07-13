use mount::Mount;


mod remoteaddr;


pub fn middleware() -> Mount {
    let mut middleware = Mount::new();
    middleware.mount("/ip", remoteaddr::router());
    middleware
}
