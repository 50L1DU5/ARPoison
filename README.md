ARPoison
========

This is a simple program written with the intention of learning the Rust language. It is based on the `arpspoof` utility from dsniff.

Disclaimer: This code is provided for educational purposes only. Don't poison networks you don't have permission to!

## Build
`cargo build`

## Usage
```
ARPoison 0.1.0
solidus
A stupid attempt to learn Rust

USAGE:
    arpoison [FLAGS] <HOST> --interface <interface> --target-mac <macaddr> --target <target>

FLAGS:
    -b, --bidirectional    If present, poison target and host in both directions
    -h, --help             Prints help information
    -V, --version          Prints version information

OPTIONS:
    -i, --interface <interface>    Interface to use to send spoofed data
    -m, --target-mac <macaddr>     The MAC address of the target
    -t, --target <target>          Target IP to poison

ARGS:
    <HOST>    The host to inpersonate (usually the gateway)
```
