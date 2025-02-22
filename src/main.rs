use clap::{Arg, Command, ArgMatches};
use error::ZebroError;
use regex::Regex;
use std::io::Write;
use std::net::TcpStream;

const DEFAULT_PORT: &str = "9100";

mod error;

fn main() {
    if let Err(e) = run() {
        eprintln!("Fehler: {}", e);
    }
}

fn run() -> Result<(), ZebroError> {
    let matches = get_matches();

    let address = matches.get_one::<String>("address");
    let ip = matches.get_one::<String>("ip");
    let port = *matches.get_one::<&str>("port").unwrap_or(&DEFAULT_PORT);
    let zpl_code = matches.get_one::<String>("zpl").ok_or(ZebroError::MissingZplCode)?;

    let full_address = match (address, ip) {
        (Some(address), _) => address.clone(),
        (None, Some(ip)) => format!("{}:{}", ip, port),
        (None, None) => return Err(ZebroError::MissingAddress),
    };

    validate_address(&full_address)?;
    validate_zpl_code(&zpl_code)?;

    print_zpl(&full_address, zpl_code)?;

    println!("ZPL code sent successfully!");

    Ok(())
}

fn validate_address(address: &str) -> Result<(), ZebroError> {
    let re = Regex::new(r"^\d{1,3}(\.\d{1,3}){3}:\d{1,5}$").unwrap();
    if re.is_match(address) {
        Ok(())
    } else {
        Err(ZebroError::InvalidAddress)
    }
}

fn validate_zpl_code(code: &str) -> Result<(), ZebroError> {
    if code.is_empty() {
        return Err(ZebroError::MissingZplCode)
    };
    let re = Regex::new(r"[\x00-\x08\x0B-\x0C\x0E-\x1F]").unwrap();
    if re.is_match(code) {
        return Err(ZebroError::InvalidZplCode)
    };
    Ok(())
}

fn get_matches() -> ArgMatches {
    Command::new("zebro")
        .version("0.1.0")
        .about("A CLI tool to send ZPL code to a printer")
        .arg(
            Arg::new("address")
                .short('a')
                .long("address")
                .value_name("IP:PORT")
                .help("Define the address of the printer (IP:PORT)")
                .num_args(1),
        )
        .arg(
            Arg::new("ip")
                .short('i')
                .long("ip")
                .value_name("IP")
                .help("Define the IP of the printer")
                .num_args(1),
        )
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .value_name("PORT")
                .help("Define the port of the printer (default: 9100)")
                .num_args(1),
        )
        .arg(
            Arg::new("zpl")
                .short('z')
                .long("zpl")
                .value_name("ZPL_CODE")
                .help("The ZPL code to send to the printer")
                .num_args(1),
        )
        .get_matches()
}

pub fn print_zpl(
    address: &str,
    zpl_code: &str,
) -> Result<(), ZebroError> {
    let mut stream = TcpStream::connect(address).map_err(|_| ZebroError::ConnectionError(address.to_string()))?;
    stream.write_all(zpl_code.as_bytes())?;
    stream.flush()?;
    Ok(())
}