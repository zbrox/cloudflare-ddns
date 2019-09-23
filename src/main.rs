mod file;
mod network;

use file::{read_cache_file, write_cache_file};
use human_panic::setup_panic;
use network::{get_current_ip, get_dns_record_id, get_zone_identifier, update_ddns};
use quicli::fs::read_file;
use quicli::prelude::*;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Deserialize)]
struct Config {
    email: String,
    auth_key: String,
    zone: String,
    domain: String,
}

#[derive(Debug, StructOpt)]
/// Inform Cloudflare's DDNS service of the current IP address for your domain
struct Cli {
    /// Your TOML config file containing all the required options (email, auth_key, zone, domain) which you can use instead of passing the arguments to the command line
    #[structopt(long = "config", short = "f")]
    config: Option<PathBuf>,

    /// Your Cloudflare login email
    #[structopt(long = "email", short = "e", required_unless = "config")]
    email: Option<String>,

    /// The auth key you need to generate in your Cloudflare profile
    #[structopt(long = "key", short = "k", required_unless = "config")]
    auth_key: Option<String>,

    /// The zone in which your domain is (usually that is your base domain name)
    #[structopt(long = "zone", short = "z", required_unless = "config")]
    zone: Option<String>,

    /// The domain for which you want to report the current IP address
    #[structopt(long = "domain", short = "d", required_unless = "config")]
    domain: Option<String>,

    /// Cache file for previously reported IP address (if skipped the IP will be reported on every execution)
    #[structopt(long = "cache", short = "c")]
    cache: Option<PathBuf>,
}

fn main() -> CliResult {
    setup_panic!();
    let args = Cli::from_args();

    let should_use_cache = args.cache.is_some();
    let cached_ip: Option<String> = match args.cache.clone() {
        Some(v) => {
            if v.exists() {
                Some(read_cache_file(&v.clone())?)
            } else {
                Some("0.0.0.0".to_owned())
            }
        }
        None => None,
    };

    let current_ip = get_current_ip()?;
    if cached_ip.is_some() && current_ip == cached_ip.unwrap() {
        println!("IP is unchanged. Exiting...");
        return Ok(());
    }

    if should_use_cache {
        println!(
            "Saving current IP {} to cache file {:?}...",
            &current_ip,
            &args.cache.clone().unwrap()
        );
        write_cache_file(&args.cache.unwrap(), &current_ip)?;
    }

    let (email, auth_key, zone, domain) = match args.config {
        Some(c) => {
            let config_str = read_file(c)?;
            let config: Config = toml::from_str(&config_str)?;
            (config.email, config.auth_key, config.zone, config.domain)
        }
        None => (
            args.email.expect("Email is not set"),
            args.auth_key.expect("Auth key is not set"),
            args.zone.expect("Zone is not set"),
            args.domain.expect("Domain is not set"),
        ),
    };

    update(&current_ip, &email, &auth_key, &zone, &domain)?;

    println!(
        "Successfully updated the A record for {} to {}",
        &domain, &current_ip
    );

    Ok(())
}

fn update(
    current_ip: &str,
    email: &str,
    auth_key: &str,
    zone: &str,
    domain: &str,
) -> Result<(), Error> {
    let zone_id = get_zone_identifier(&zone, &email, &auth_key)?;
    let record_id = get_dns_record_id(&zone_id, &domain, &email, &auth_key)?;

    update_ddns(
        &current_ip,
        &domain,
        &zone_id,
        &record_id,
        &email,
        &auth_key,
    )?;

    Ok(())
}
