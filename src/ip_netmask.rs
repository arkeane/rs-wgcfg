use std::net::Ipv4Addr;
use std::fmt;
use std::str::FromStr;

pub struct IpNetmask{
    pub ip: Ipv4Addr,
    pub netmask: i32,
}

pub struct ParseError{}

impl IpNetmask{
    pub fn new(ip: Ipv4Addr, netmask: i32) -> IpNetmask{
        IpNetmask {
            ip,
            netmask,
        }
    }
}

impl fmt::Display for IpNetmask{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result{
        write!(fmt, "{}/{}", self.ip, self.netmask)
    }
}

impl Clone for IpNetmask{
    fn clone(&self) -> Self{
        IpNetmask { ip: (self.ip), netmask: (self.netmask) }
    }
}

impl FromStr for IpNetmask{

    type Err = ParseError;

    fn from_str(s: &str)-> Result<Self, Self::Err>{
        let ipv4;
        let nm;

        let parts: Vec<&str> = s.split("/").collect();
        ipv4 = Ipv4Addr::from_str(parts[0]).unwrap();
        nm = i32::from_str(parts[1]).unwrap();
        Ok(IpNetmask{
            ip: (ipv4),
            netmask: (nm),
        })
    }
}