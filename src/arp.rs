use std::{
    net::Ipv4Addr,
    time::{Duration, Instant},
};

use anyhow::anyhow;
use clap::Parser;
use pnet::{
    datalink::{self, Channel},
    ipnetwork::IpNetwork,
    packet::{
        MutablePacket, Packet,
        arp::{ArpHardwareTypes, ArpOperations, ArpPacket, MutableArpPacket},
        ethernet::{EtherTypes, EthernetPacket, MutableEthernetPacket},
    },
};

/// Send an ARP packet
#[derive(Parser, Debug)]
pub struct Args {
    interface: String,
    target_ip: Ipv4Addr,
    #[clap(long, short, default_value_t = 5)]
    timeout: u64,
}

pub fn send_arp(args: Args) -> anyhow::Result<()> {
    let interface = datalink::interfaces()
        .into_iter()
        .find(|iface| iface.name == args.interface)
        .ok_or(anyhow::anyhow!(
            "Could not find interface {}",
            args.interface
        ))?;

    let source_mac = interface
        .mac
        .ok_or(anyhow!("Interface does not have MAC address"))?;
    let source_ip = interface
        .ips
        .iter()
        .filter(|ip| ip.is_ipv4())
        .next()
        .ok_or(anyhow!("Interface does not have IPv4 addr"))?;

    // 14 ethernet  + 28 arp
    let mut buffer = [0u8; 42];
    let mut eth_packet = MutableEthernetPacket::new(&mut buffer)
        .ok_or(anyhow!("Unable to create ethernet packet"))?;

    eth_packet.set_destination([0xff; 6].into());
    eth_packet.set_source(source_mac);
    eth_packet.set_ethertype(EtherTypes::Arp);

    {
        let mut arp_payload = MutableArpPacket::new(eth_packet.payload_mut())
            .ok_or(anyhow!("Unable to create mutable ARP packet"))?;
        arp_payload.set_hardware_type(ArpHardwareTypes::Ethernet);
        arp_payload.set_protocol_type(EtherTypes::Ipv4);
        arp_payload.set_hw_addr_len(6);
        arp_payload.set_proto_addr_len(4);
        arp_payload.set_operation(ArpOperations::Request);
        arp_payload.set_sender_hw_addr(source_mac);
        if let IpNetwork::V4(ip) = source_ip {
            arp_payload.set_sender_proto_addr(ip.ip());
        } else {
            unreachable!()
        }
        arp_payload.set_target_hw_addr([0; 6].into());
        arp_payload.set_target_proto_addr(args.target_ip);
    }

    let channel = datalink::channel(&interface, Default::default())?;
    if let Channel::Ethernet(mut tx, mut rx) = channel {
        tx.send_to(eth_packet.packet(), None)
            .ok_or(anyhow!("Send failed"))??;

        let start = Instant::now();
        while Instant::now() - start < Duration::from_secs(args.timeout) {
            let packet_data = rx.next()?;
            if let Some(ethernet) = EthernetPacket::new(packet_data) {
                if ethernet.get_ethertype() == EtherTypes::Arp {
                    if let Some(arp) = ArpPacket::new(ethernet.payload()) {
                        if arp.get_operation() == ArpOperations::Reply
                            && arp.get_sender_proto_addr() == args.target_ip
                        {
                            println!(
                                "REPLY: {} is at {}",
                                arp.get_sender_proto_addr(),
                                arp.get_sender_hw_addr()
                            );
                            return Ok(());
                        }
                    }
                }
            }
        }
    } else {
        return Err(anyhow!("Unhandled channel type"));
    }

    Ok(())
}
