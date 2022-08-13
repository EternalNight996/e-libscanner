pub(crate) mod ethernet;
pub(crate) mod icmp;
pub(crate) mod ipv4;
pub(crate) mod ipv6;
pub(crate) mod tcp;
pub(crate) mod udp;

pub(crate) const ICMP_PACKET_SIZE: usize = 66;

#[cfg(not(target_family="windows"))]
pub(crate) const TCP_PACKET_SIZE: usize = 90;

#[cfg(target_family="windows")]
pub(crate) const TCP_PACKET_SIZE: usize = 66;

pub(crate) const UDP_PACKET_SIZE: usize = 66;
