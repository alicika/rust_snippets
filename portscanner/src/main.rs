use clap::{App, Arg};
use futures::{stream, StreamExt};
use std::{
    net::{IpAddr, SocketAddr, ToSocketAddrs},
    time::Duration,
};
use tokio::net::TcpStream;

mod ports;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let cli_matches = App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .about(clap::crate_description!())
        .arg(
            Arg::with_name("target")
                .help("The target to scan")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("concurrency")
                .help("Concurrency")
                .long("concurrency")
                .short("c")
                .default_value("1002"),
        )
        .arg(
            Arg::with_name("verbose")
                .help("Display detailed information")
                .long("verbose")
                .short("v"),
        )
        .arg(
            Arg::with_name("full")
                .help("Scan all 65535 ports")
                .long("full"),
        )
        .arg(
            Arg::with_name("timeout")
                .help("Connection timeout")
                .long("timeout")
                .short("t")
                .default_value("3"),
        )
        .setting(clap::AppSettings::ArgRequiredElseHelp)
        .setting(clap::AppSettings::VersionlessSubcommands)
        .get_matches();

    let full = cli_matches.is_present("full");
    let verbose = cli_matches.is_present("verbose");
    let concurrency = cli_matches
        .value_of("concurrency")
        .unwrap()
        .parse::<usize>()
        .unwrap_or(1002);
    let timeout = cli_matches
        .value_of("timeout")
        .unwrap()
        .parse::<u64>()
        .unwrap_or(3);
    let target = cli_matches.value_of("target").unwrap();

    if verbose {
        let ports = if full {
            String::from("all the 65535 ports")
        } else {
            String::from("the most common 1002 ports")
        };
        println!(
            "Scanning {} of {}. Concurrency: {:?}. Timeout: {:?}",
            &ports, target, concurrency, timeout
        );
    }

    let socket_addresses: Vec<SocketAddr> = format!("{}:0", target).to_socket_addrs()?.collect();

    if socket_addresses.is_empty() {
        return Err(anyhow::anyhow!("Socket_addresses list is empty"));
    }

    scan(socket_addresses[0].ip(), full, concurrency, timeout).await;

    Ok(())
}

async fn scan(target: IpAddr, full: bool, concurrency: usize, timeout: u64) {
    let ports = stream::iter(get_ports(full));

    ports
        .for_each_concurrent(concurrency, |port| scan_port(target, port, timeout))
        .await;
}

async fn scan_port(target: IpAddr, port: u16, timeout: u64) {
    let timeout = Duration::from_secs(timeout);
    let socket_address = SocketAddr::new(target.clone(), port);

    if tokio::time::timeout(timeout, TcpStream::connect(&socket_address))
        .await
        .is_ok()
    {
        println!("{}", port);
    }
}

fn get_ports(full: bool) -> Box<dyn Iterator<Item = u16>> {
    if full {
        Box::new((1..=u16::MAX).into_iter())
    } else {
        Box::new(ports::MOST_COMMON_PORTS_1002.to_owned().into_iter())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn get_result() {
        let numbers: [u16; 119] = [
            5601, 9300, 80, 23, 443, 21, 22, 25, 3389, 110, 445, 139, 143, 53, 135, 3306, 8080,
            1723, 111, 995, 993, 5900, 1025, 587, 8888, 199, 1720, 465, 548, 113, 81, 6001, 10000,
            514, 5060, 179, 1026, 2000, 8443, 8000, 32768, 554, 26, 1433, 49152, 2001, 515, 8008,
            49154, 1027, 5666, 646, 5000, 5631, 631, 49153, 8081, 2049, 88, 79, 5800, 106, 2121,
            1110, 49155, 6000, 513, 990, 5357, 427, 49156, 543, 544, 5101, 144, 7, 389, 8009, 3128,
            444, 9999, 5009, 7070, 5190, 3000, 5432, 1900, 3986, 13, 1029, 9, 5051, 6646, 49157,
            1028, 873, 1755, 2717, 4899, 9100, 119, 37, 1000, 3001, 5001, 82, 10010, 1030, 9090,
            2107, 1024, 2103, 6004, 1801, 5050, 19, 8031, 1041, 255,
        ];
        assert_eq!(Box::new(numbers), get_ports(false))
    }
}
