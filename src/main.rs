use std::{net::IpAddr, str::FromStr, u16};

use clap::Parser;
use ipnet::IpNet;

/// Small tool to scan remote networks aggressively
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// single host or subnet to be scanned
    #[arg(short, long)]
    target: String,
    // TODO: some kind of options
    // #[arg(short, long)]
    // options: String
}

enum Protocol {
    TCP,
    UDP,
}

struct Port {
    number: u16,
    proto: Protocol,
    up: bool,
}

struct Host {
    address: IpAddr,
    availible_ports: Vec<Port>,
    up: bool,
    scanned: bool,
    imcp_avalible: bool,
}

fn scan_single(host: &mut Host) {
    host.scanned = true;
    println!("{}", host.address);
}
fn scan_subnet(subnet: &str) {
    match IpNet::from_str(subnet) {
        Ok(s) => {
            let mut hosts: Vec<Host> = vec![];
            println!("Scanning {} hosts...", s.hosts().count());
            for host in s.hosts() {
                hosts.push(Host {
                    address: (host),
                    availible_ports: (vec![]),
                    up: false,
                    scanned: false,
                    imcp_avalible: false,
                });
            }
            for mut host in hosts {
                scan_single(&mut host)
            }
        }
        Err(_) => println!("could not parse subnet"),
    }
}

fn main() {
    let args = Args::parse();

    match args.target.contains('/') {
        true => {
            println!("Expecting subnet: {}", args.target);
            scan_subnet(&args.target)
        }
        false => {
            println!("Found single host: {}", args.target);
            let ip = IpAddr::from_str(&args.target);
            match ip {
                Ok(s) => {
                    let mut host = Host {
                        address: (s),
                        imcp_avalible: false,
                        availible_ports: (vec![]),
                        scanned: false,
                        up: false,
                    };
                    scan_single(&mut host);
                }
                Err(_) => println!("Could not parse ip address"),
            }
        }
    }
}
