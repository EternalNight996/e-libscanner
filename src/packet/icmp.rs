use e_utils::random;
use pnet_packet::icmp::echo_request::MutableEchoRequestPacket;
use pnet_packet::icmp::IcmpTypes;
use pnet_packet::Packet;
/// Build icmp packet
pub fn build_icmp_packet(icmp_packet: &mut MutableEchoRequestPacket<'_>) {
    icmp_packet.set_icmp_type(IcmpTypes::EchoRequest);
    icmp_packet.set_sequence_number(random!(#u16));
    icmp_packet.set_identifier(random!(#u16));
    let icmp_check_sum = pnet_packet::util::checksum(&icmp_packet.packet(), 1);
    icmp_packet.set_checksum(icmp_check_sum);
}
