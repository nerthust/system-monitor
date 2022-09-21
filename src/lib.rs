pub mod core;

use procfs::net::{TcpState, UdpState};

pub trait Casting {
    fn to_string(&self) -> String;
}

impl Casting for TcpState {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

impl Casting for UdpState {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}
