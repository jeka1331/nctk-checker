mod settings;
use std::{net::{IpAddr, SocketAddr, TcpStream}, str::FromStr};
use hostname;

use settings::Settings;

fn main() {
    loop {
        let settings = Settings::new();
        let _hostname: String = match hostname::get() {
            Ok(hostname) => match hostname.into_string(){
                Ok(hostname) => hostname,
                Err(_) => panic!("Error with parse hostname"),
            },
            Err(_) => panic!("Error with parse hostname"),
        };
        println!("{:?}", settings);

        // Print out our settings
        let addresses = match settings {
            Ok(settings) => settings.addresses,
            Err(_) => {
                panic!("Failed to read addresses config file")
            }
        };

        for address in &addresses {
            let port = match address.port {
                Some(port) => port,
                None => 0,
            };
            let ports = match address.ports.clone() {
                Some(ports) => ports,
                None => vec![port],
            };

            for _port in ports.clone() {
                let full_address = SocketAddr::new(
                    match IpAddr::from_str(&address.host) {
                        Ok(address) => address,
                        Err(_) => panic!("Failed to parse ip address"),
                    },
                    _port.try_into().unwrap()
                );
                match TcpStream::connect_timeout(
                    &full_address, 
                    std::time::Duration::from_millis(500)
                ) {
                    Ok(_) => println!(
                        "{} -> {} | Connection established successfully", 
                        _hostname,
                        full_address
                    ),
                    Err(e) => println!(
                        "{} -> {} | Error establishing connection: {}", 
                        _hostname,
                        full_address, 
                        e
                    ),
                }
            }
        }
    }
}
