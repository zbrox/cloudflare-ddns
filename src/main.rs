mod network;
mod file;

use std::path::PathBuf;
use quicli::prelude::*;
use structopt::StructOpt;
use human_panic::{setup_panic};
use network::{get_zone_identifier, get_dns_record_id, get_current_ip, update_ddns};
use file::{read_cache_file, write_cache_file};

#[derive(Debug, StructOpt)]
/// Inform Cloudflare's DDNS service of the current IP address for your domain
struct Cli {
    /// Your Cloudflare login email
    #[structopt(long = "email", short = "e")]
    email: String,

    /// The auth key you need to generate in your Cloudflare profile
    #[structopt(long = "key", short = "k")]
    auth_key: String,

    /// The zone in which your domain is (usually that is your base domain name)
    #[structopt(long = "zone", short = "z")]
    zone: String,

    /// The domain for which you want to report the current IP address
    #[structopt(long = "domain", short = "d")]
    domain: String,

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
        },
        None => None,
    };


    let current_ip = get_current_ip()?;
    if cached_ip.is_some() && current_ip == cached_ip.unwrap() {
        println!("IP is unchanged. Exiting...");
        return Ok(());
    }

    if should_use_cache {
        println!("Saving current IP {} to cache file {:?}...", &current_ip, &args.cache.clone().unwrap());
        write_cache_file(&args.cache.unwrap(), &current_ip)?;
    }

    let zone_id = get_zone_identifier(&args.zone, &args.email, &args.auth_key)?;
    let record_id = get_dns_record_id(&zone_id, &args.domain, &args.email, &args.auth_key)?;

    update_ddns(&current_ip, &args.domain, &zone_id, &record_id, &args.email, &args.auth_key)?;

    println!("Successfully updated the A record for {} to {}", &args.domain, &current_ip);

    Ok(())
}
