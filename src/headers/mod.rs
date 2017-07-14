use std::fmt;
use std::net::SocketAddr;
use hyper;
use iron;


#[derive(Debug, Clone, Copy)]
pub struct XForwardedFor {
    pub addr: SocketAddr
}

impl iron::headers::Header for XForwardedFor {
    fn header_name() -> &'static str {
        "X-Forwarded-For"
    }

    fn parse_header(raw: &[Vec<u8>]) -> hyper::Result<XForwardedFor> {
        let addr: SocketAddr = iron::headers::parsing::from_one_raw_str(raw).unwrap();
        Ok(XForwardedFor { addr: addr })
    }
}

impl iron::headers::HeaderFormat for XForwardedFor {
    fn fmt_header(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.addr.ip().to_string())
    }
}
