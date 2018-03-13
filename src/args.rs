// File: args.rs

use clap::{App, Arg, ArgMatches};
use colored::*;
use std::process::exit;
use std::str::FromStr;
use std::net::Ipv4Addr;
use pnet::datalink::MacAddr;

// AppArgs
// Structure to store
// command line arguments
// which can be passed to
// the main function.
#[derive(Clone, Debug)]
pub struct AppArgs {
    // The physical interface to use to send ARP
    // packets. Usually "eth0" or "enp2s0".
    pub interface: String,
    // Target IP address is the "victim" or IP to which we
    // are going to send the spoofed ARP packets.
    pub target:    Ipv4Addr,
    // The target MAC address. This is the MAC associated
    // with the target's IP address. We don't ask the target
    // for their MAC address, because after all, who can
    // trust the network? ;)
    pub t_mac:     MacAddr,
    // The  "host" is the IP we're going to
    // impersonate by associating our MAC
    // with this IP in the victim's ARP
    // cache.
    pub host:      Ipv4Addr,
    // Accepted, but currently not supported.
    // Mostly because I don't want to support
    // two options for MAC addresses.
    pub bidirect:  bool,
}

pub fn get_args() -> AppArgs {
    let matches: ArgMatches = App::new("ARPoison")
        .author("solidus")
        .version("0.1.0")
        .about("A simple tool to send spoofed ARP packets to poison a victim's cache")
        .arg(Arg::with_name("target")
            .long("target")
            .short("t")
            .help("Target IP to poison")
            .required(true)
            .takes_value(true)
        )                            
        .arg(Arg::with_name("interface")
            .long("interface")                  
            .short("i")
            .help("Interface to use to send spoofed data")
            .required(true)
            .takes_value(true)
        )
        .arg(Arg::with_name("macaddr")
            .long("target-mac")
            .short("m")
            .help("The MAC address of the target")
            .required(true)
            .takes_value(true)
        )
        .arg(Arg::with_name("bidirectional")
            .short("b")
            .long("bidirectional")
            .required(false)
            .help("If present, poison target and host in both directions")
            .takes_value(false)
        )
        .arg(Arg::with_name("HOST")
            .index(1)
            .required(true)
            .help("The host to inpersonate (usually the gateway)")
        )
        .get_matches();                                   
    let iface  = matches.value_of("interface").unwrap();
    let target = matches.value_of("target").unwrap();
    let t_mac  = matches.value_of("macaddr").unwrap();
    let host   = matches.value_of("HOST").unwrap();
    let target = match target.parse() {
        Ok(t) => t,
        Err(_) => {
            eprintln!(
                "{}\n",
                "Error: Could not parse target IP addr"
                .white()
                .on_bright_red()
            );
            exit(1);
        }

    };
    let t_mac = match MacAddr::from_str(&t_mac) {
        Ok(m) => m,
        Err(_) => {
            eprintln!(
                "{}\n",
                "Error: Could not parse target MAC addr"
                .white()
                .on_bright_red()
            );

            exit(1);
        }
    };
    let host = match host.parse() {
        Ok(h) => h,
        Err(_) => {
            eprintln!(
                "{}\n",
                "Error: Could not parse host IP addr"
                .white()
                .on_bright_red()
            );

            exit(1);
        }
    };
    let bi = match matches.occurrences_of("bidirectional") {
        0 => false,
        _ => true,
    };

    AppArgs{
        interface: iface.into(),
        target:    target,
        t_mac:     t_mac,
        host:      host,
        bidirect:  bi,
    }
}
