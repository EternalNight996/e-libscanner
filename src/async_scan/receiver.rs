use crate::data::DATA;
use crate::frame::{
    result::{HostInfo, PortInfo, PortStatus, ScanResults},
    ScanSetting, ScanType,
};
use pnet_packet::Packet;
use std::net::{IpAddr, SocketAddr};
use std::sync::{Arc, Mutex};
use std::time::Instant;

pub(crate) async fn receive_packets(
    rx: &mut Box<dyn pnet_datalink::DataLinkReceiver>,
    scan_setting: ScanSetting,
    scan_result: &Arc<Mutex<ScanResults>>,
    stop: &Arc<Mutex<bool>>,
) {
    let start_time = Instant::now();
    loop {
        match rx.next() {
            Ok(frame) => {
                let frame = pnet_packet::ethernet::EthernetPacket::new(frame).unwrap();
                match frame.get_ethertype() {
                    pnet_packet::ethernet::EtherTypes::Ipv4 => {
                        ipv4_handler(&frame, &scan_setting, &scan_result);
                    }
                    pnet_packet::ethernet::EtherTypes::Ipv6 => {
                        ipv6_handler(&frame, &scan_setting, &scan_result);
                    }
                    _ => {}
                }
            }
            Err(_) => {}
        }
        if *stop.lock().unwrap() {
            break;
        }
        if Instant::now().duration_since(start_time) > scan_setting.timeout {
            break;
        }
    }
}

fn ipv4_handler(
    ethernet: &pnet_packet::ethernet::EthernetPacket<'_>,
    scan_setting: &ScanSetting,
    scan_result: &Arc<Mutex<ScanResults>>,
) {
    if let Some(packet) = pnet_packet::ipv4::Ipv4Packet::new(ethernet.payload()) {
        match packet.get_next_level_protocol() {
            pnet_packet::ip::IpNextHeaderProtocols::Tcp => {
                tcp_handler_v4(&packet, scan_setting, scan_result);
            }
            pnet_packet::ip::IpNextHeaderProtocols::Udp => {
                udp_handler_v4(&packet, scan_setting, scan_result);
            }
            pnet_packet::ip::IpNextHeaderProtocols::Icmp => {
                icmp_handler_v4(&packet, scan_setting, scan_result);
            }
            _ => {}
        }
    }
}

fn ipv6_handler(
    ethernet: &pnet_packet::ethernet::EthernetPacket<'_>,
    scan_setting: &ScanSetting,
    scan_result: &Arc<Mutex<ScanResults>>,
) {
    if let Some(packet) = pnet_packet::ipv6::Ipv6Packet::new(ethernet.payload()) {
        match packet.get_next_header() {
            pnet_packet::ip::IpNextHeaderProtocols::Tcp => {
                tcp_handler_v6(&packet, scan_setting, scan_result);
            }
            pnet_packet::ip::IpNextHeaderProtocols::Udp => {
                udp_handler_v6(&packet, scan_setting, scan_result);
            }
            pnet_packet::ip::IpNextHeaderProtocols::Icmpv6 => {
                icmp_handler_v6(&packet, scan_setting, scan_result);
            }
            _ => {}
        }
    }
}

fn tcp_handler_v4(
    packet: &pnet_packet::ipv4::Ipv4Packet<'_>,
    scan_setting: &ScanSetting,
    scan_result: &Arc<Mutex<ScanResults>>,
) {
    let tcp_packet = pnet_packet::tcp::TcpPacket::new(packet.payload());
    if let Some(tcp_packet) = tcp_packet {
        let host_info: HostInfo = HostInfo {
            ip_addr: IpAddr::V4(packet.get_source()),
            ttl: packet.get_ttl(),
        };
        handle_tcp_packet(tcp_packet, host_info, &scan_setting, scan_result);
    }
}

fn tcp_handler_v6(
    packet: &pnet_packet::ipv6::Ipv6Packet<'_>,
    scan_setting: &ScanSetting,
    scan_result: &Arc<Mutex<ScanResults>>,
) {
    let tcp_packet = pnet_packet::tcp::TcpPacket::new(packet.payload());
    if let Some(tcp_packet) = tcp_packet {
        let host_info: HostInfo = HostInfo {
            ip_addr: IpAddr::V6(packet.get_source()),
            ttl: packet.get_hop_limit(),
        };
        handle_tcp_packet(tcp_packet, host_info, &scan_setting, scan_result);
    }
}

fn udp_handler_v4(
    packet: &pnet_packet::ipv4::Ipv4Packet<'_>,
    scan_setting: &ScanSetting,
    scan_result: &Arc<Mutex<ScanResults>>,
) {
    let udp = pnet_packet::udp::UdpPacket::new(packet.payload());
    if let Some(udp) = udp {
        handle_udp_packet(udp, &scan_setting, scan_result);
    }
}

fn udp_handler_v6(
    packet: &pnet_packet::ipv6::Ipv6Packet<'_>,
    scan_setting: &ScanSetting,
    scan_result: &Arc<Mutex<ScanResults>>,
) {
    let udp = pnet_packet::udp::UdpPacket::new(packet.payload());
    if let Some(udp) = udp {
        handle_udp_packet(udp, &scan_setting, scan_result);
    }
}

fn icmp_handler_v4(
    packet: &pnet_packet::ipv4::Ipv4Packet<'_>,
    scan_setting: &ScanSetting,
    scan_result: &Arc<Mutex<ScanResults>>,
) {
    let icmp_packet = pnet_packet::icmp::IcmpPacket::new(packet.payload());
    if let Some(_icmp) = icmp_packet {
        if scan_setting
            .ip_set
            .contains(&IpAddr::V4(packet.get_source()))
            && !scan_result
                .lock()
                .unwrap()
                .ip_set
                .contains(&IpAddr::V4(packet.get_source()))
        {
            scan_result.lock().unwrap().result.ips.push(HostInfo {
                ip_addr: IpAddr::V4(packet.get_source()),
                ttl: packet.get_ttl(),
            });
            scan_result
                .lock()
                .unwrap()
                .ip_set
                .insert(IpAddr::V4(packet.get_source()));
        }
    }
}

fn icmp_handler_v6(
    packet: &pnet_packet::ipv6::Ipv6Packet<'_>,
    scan_setting: &ScanSetting,
    scan_result: &Arc<Mutex<ScanResults>>,
) {
    let icmp_packet = pnet_packet::icmp::IcmpPacket::new(packet.payload());
    if let Some(_icmp) = icmp_packet {
        if scan_setting
            .ip_set
            .contains(&IpAddr::V6(packet.get_source()))
            && !scan_result
                .lock()
                .unwrap()
                .ip_set
                .contains(&IpAddr::V6(packet.get_source()))
        {
            scan_result.lock().unwrap().result.ips.push(HostInfo {
                ip_addr: IpAddr::V6(packet.get_source()),
                ttl: packet.get_hop_limit(),
            });
            scan_result
                .lock()
                .unwrap()
                .ip_set
                .insert(IpAddr::V6(packet.get_source()));
        }
    }
}

fn handle_tcp_packet(
    tcp_packet: pnet_packet::tcp::TcpPacket<'_>,
    host_info: HostInfo,
    scan_setting: &ScanSetting,
    scan_result: &Arc<Mutex<ScanResults>>,
) {
    let socket_addr: SocketAddr = SocketAddr::new(host_info.ip_addr, tcp_packet.get_source());
    if tcp_packet.get_flags() == pnet_packet::tcp::TcpFlags::SYN | pnet_packet::tcp::TcpFlags::ACK {
        match scan_setting.scan_type {
            ScanType::TcpSynScan => {
                if !scan_result
                    .lock()
                    .unwrap()
                    .socket_set
                    .contains(&socket_addr)
                {
                    let port_info = PortInfo {
                        port: socket_addr.port(),
                        status: PortStatus::Open,
                        describe: DATA
                            .portmap
                            .get(&socket_addr.port())
                            .unwrap_or(&"unknown type")
                            .to_string(),
                    };
                    // Avoid deadlock.
                    let exists: bool = if let Some(r) = scan_result
                        .lock()
                        .unwrap()
                        .result
                        .ip_with_port
                        .get_mut(&socket_addr.ip())
                    {
                        r.push(port_info.clone());
                        true
                    } else {
                        false
                    };
                    if !exists {
                        scan_result
                            .lock()
                            .unwrap()
                            .result
                            .ip_with_port
                            .insert(socket_addr.ip(), vec![port_info]);
                    }
                    scan_result.lock().unwrap().socket_set.insert(socket_addr);
                }
            }
            _ => {
                scan_result.lock().unwrap().result.ips.push(host_info);
                match host_info.ip_addr {
                    IpAddr::V4(ip) => {
                        scan_result.lock().unwrap().ip_set.insert(IpAddr::V4(ip));
                    }
                    IpAddr::V6(ip) => {
                        scan_result.lock().unwrap().ip_set.insert(IpAddr::V6(ip));
                    }
                }
            }
        }
    } else if tcp_packet.get_flags()
        == pnet_packet::tcp::TcpFlags::RST | pnet_packet::tcp::TcpFlags::ACK
    {
        match scan_setting.scan_type {
            ScanType::TcpSynScan => {
                if !scan_result
                    .lock()
                    .unwrap()
                    .socket_set
                    .contains(&socket_addr)
                {
                    let port_info = PortInfo {
                        port: socket_addr.port(),
                        status: PortStatus::Closed,
                        describe: DATA
                            .portmap
                            .get(&socket_addr.port())
                            .unwrap_or(&"unknown type")
                            .to_string(),
                    };
                    // Avoid deadlock.
                    let exists: bool = if let Some(r) = scan_result
                        .lock()
                        .unwrap()
                        .result
                        .ip_with_port
                        .get_mut(&socket_addr.ip())
                    {
                        r.push(port_info.clone());
                        true
                    } else {
                        false
                    };
                    if !exists {
                        scan_result
                            .lock()
                            .unwrap()
                            .result
                            .ip_with_port
                            .insert(socket_addr.ip(), vec![port_info]);
                    }
                    scan_result.lock().unwrap().socket_set.insert(socket_addr);
                }
            }
            _ => {
                scan_result.lock().unwrap().result.ips.push(host_info);
                match host_info.ip_addr {
                    IpAddr::V4(ip) => {
                        scan_result.lock().unwrap().ip_set.insert(IpAddr::V4(ip));
                    }
                    IpAddr::V6(ip) => {
                        scan_result.lock().unwrap().ip_set.insert(IpAddr::V6(ip));
                    }
                }
            }
        }
    }
}

fn handle_udp_packet(
    _udp_packet: pnet_packet::udp::UdpPacket<'_>,
    _scan_setting: &ScanSetting,
    _scan_result: &Arc<Mutex<ScanResults>>,
) {
}
