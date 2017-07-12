use rustless::{ Api, Nesting, Versioning };


mod remoteaddr;


pub fn root() -> Api {
    Api::build(|api| {
        api.version("v1", Versioning::Path);
        api.mount(remoteaddr::ip("ip"));
    })
}
