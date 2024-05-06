use anyhow::Result;
use pnet::datalink::{self, NetworkInterface};
use pnet::packet::ethernet::{EtherTypes, EthernetPacket};
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::udp::UdpPacket;
use pnet::packet::Packet;
use tracing::info;

pub fn run() -> Result<()> {
    let interfaces = datalink::interfaces();
    let interface = interfaces
        .into_iter()
        .find(|iface: &NetworkInterface| iface.name == "eth0")
        .unwrap();

    let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(datalink::Channel::Ethernet(tx, rx)) => (tx, rx),
        _ => panic!("Failed to create datalink channel."),
    };

    loop {
        match rx.next() {
            Ok(frame) => {
                let packet = EthernetPacket::new(frame).unwrap();
                match packet.get_ethertype() {
                    EtherTypes::Ipv4 => ipv4_handler(&packet),
                    EtherTypes::Ipv6 => unimplemented!("IPv6 is not supported."),
                    _ => (),
                }
            }
            Err(e) => {
                panic!("An error occurred while reading: {}", e);
            }
        }
    }
}

fn ipv4_handler(ethernet: &EthernetPacket) {
    let Some(packet) = Ipv4Packet::new(ethernet.payload()) else {
        return;
    };

    match packet.get_next_level_protocol() {
        IpNextHeaderProtocols::Tcp => tcp_handler(&TcpPacket::new(packet.payload()).unwrap()),
        IpNextHeaderProtocols::Udp => udp_handler(&UdpPacket::new(packet.payload()).unwrap()),
        _ => (),
    }
}

fn tcp_handler(packet: &TcpPacket) {
    println!(
        "TCP Packet - Src Port: {}, Dst Port: {}",
        packet.get_source(),
        packet.get_destination()
    );
}

fn udp_handler(packet: &UdpPacket) {
    println!(
        "UDP Packet - Src Port: {}, Dst Port: {}",
        packet.get_source(),
        packet.get_destination()
    );
}
