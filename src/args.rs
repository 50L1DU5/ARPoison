// File: args.rs

use clap::{App, Arg, ArgMatches};
use std::process::exit;
use std::str::FromStr;
use std::net::Ipv4Addr;
use pnet::datalink::MacAddr;

#[derive(Clone, Debug)]
pub struct AppArgs {
    pub interface: String,
    pub target:    Ipv4Addr,
    pub t_mac:     MacAddr,
    pub host:      Ipv4Addr,
    pub bidirect:  bool,
}

pub fn get_args() -> AppArgs {
    let matches: ArgMatches = App::new("ARPoison")  
        .author("solidus")                                  
        .version("0.1.0")                                   
        .about("A stupid attempt to learn Rust")            
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
            eprintln!("Could not parse target");
            exit(1);
        }

    };
    let t_mac = match MacAddr::from_str(&t_mac) {
        Ok(m) => m,
        Err(_) => {
            eprintln!("Could not parse target");
            exit(1);
        }
    };
    let host = match host.parse() {
        Ok(h) => h,
        Err(_) => {
            eprintln!("Could not parse target");
            exit(1);
        }
    };
    let bi = match matches.occurrences_of("bi") {
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
