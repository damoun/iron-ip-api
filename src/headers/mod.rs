use std::fmt;
use std::net::IpAddr;
use std::str::FromStr;
use hyper;
use iron;


#[derive(Debug, Clone)]
pub struct XForwardedFor {
    pub client_ip: IpAddr,
    pub proxies_ip: Vec<IpAddr>
}

impl iron::headers::Header for XForwardedFor {
    fn header_name() -> &'static str {
        "X-Forwarded-For"
    }

    fn parse_header(raw: &[Vec<u8>]) -> hyper::Result<XForwardedFor> {
        iron::headers::parsing::from_one_raw_str(raw)
    }
}

impl iron::headers::HeaderFormat for XForwardedFor {
    fn fmt_header(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.client_ip.to_string())
    }
}


impl FromStr for XForwardedFor {
    type Err = hyper::Error;

    fn from_str(str: &str) -> hyper::Result<XForwardedFor> {
        let mut ip_addrs: Vec<IpAddr> = Vec::new();

        for ip_str in str.split(',').map(str::trim) {
            match ip_str.parse() {
                Ok(ip) => ip_addrs.push(ip),
                Err(_) => return Err(hyper::error::Error::Header),
            }
        }

        match ip_addrs.split_first() {
            Some((ip_addr, proxies_ip)) => {
                Ok(XForwardedFor {
                    client_ip: *ip_addr,
                    proxies_ip: proxies_ip.to_vec(),
                })
            }
            None => Err(hyper::error::Error::Header),
        }
    }
}
