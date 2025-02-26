use clap::Parser;
use error::ZebroError;
use regex::Regex;
use std::io::Write;
use std::net::TcpStream;

const DEFAULT_PORT: &str = "9100";

mod error;

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
    }
}

fn run() -> Result<(), ZebroError> {
    let cli = Cli::parse();

    let address = cli.address;
    let ip = cli.ip;
    let port = cli.port;
    let zpl_code = cli.zpl;

    let full_address = match (address, ip) {
        (Some(address), _) => address.clone(),
        (None, Some(ip)) => format!("{}:{}", ip, port),
        (None, None) => return Err(ZebroError::MissingAddress),
    };

    validate_address(&full_address)?;
    validate_zpl_code(&zpl_code)?;

    print_zpl(&full_address, &zpl_code)?;

    println!("ZPL code sent successfully!");

    Ok(())
}

fn validate_address(address: &str) -> Result<(), ZebroError> {
    let re = Regex::new(r"^\d{1,3}(\.\d{1,3}){3}:\d{1,5}$").unwrap();
    if let false = re.is_match(address) {
        return Err(ZebroError::InvalidAddress);
    }
    Ok(())
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

pub fn print_zpl(
    address: &str,
    zpl_code: &str,
) -> Result<(), ZebroError> {
    let mut stream = TcpStream::connect(address).map_err(|_| ZebroError::ConnectionError(address.to_string()))?;
    stream.write_all(zpl_code.as_bytes())?;
    stream.flush()?;
    Ok(())
}

#[derive(Parser, Debug)]
#[command(name = "zebro", version = "0.1.0", about = "A CLI tool to send ZPL code to a printer")]
struct Cli {
    #[arg(short, long, value_name = "IP:PORT")]
    address: Option<String>,
    #[arg(short, long, value_name = "IP")]
    ip: Option<String>,
    #[arg(short, long, value_name = "PORT", default_value_t = String::from(DEFAULT_PORT))]
    port: String,
    #[arg(short, long, value_name = "ZPL_CODE")]
    zpl: String,
}