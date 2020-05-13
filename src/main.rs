mod file;
mod network;

use file::{read_file, write_file};
use human_panic::setup_panic;
use network::{get_current_ip, get_dns_record_id, get_zone_identifier, update_ddns};
use quicli::prelude::*;
use std::path::PathBuf;
use structopt::StructOpt;
use anyhow::{Context, Result};

#[derive(Deserialize)]
struct Config {
    api_token: String,
    zone: String,
    domain: String,
}

#[derive(Debug, StructOpt)]
/// Inform Cloudflare's DDNS service of the current IP address for your domain
struct Cli {
    /// Your TOML config file containing all the required options (api_token, zone, domain) which you can use instead of passing the arguments to the command line
    #[structopt(long = "config", short = "f")]
    config: Option<PathBuf>,

    /// The api token you need to generate in your Cloudflare profile
    #[structopt(long = "token", short = "t", required_unless = "config")]
    api_token: Option<String>,

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

fn main() -> Result<()> {
    setup_panic!();
    let args = Cli::from_args();

    env_logger::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let should_use_cache = args.cache.is_some();
    let cached_ip: Option<String> = match args.cache.clone() {
        Some(v) => {
            if v.exists() {
                Some(read_file(&v).context("Could not read cache file")?)
            } else {
                Some("0.0.0.0".to_owned())
            }
        }
        None => None,
    };

    let current_ip = get_current_ip()?;
    if cached_ip.is_some() && current_ip == cached_ip.unwrap() {
        log::info!("IP is unchanged. Exiting...");
        return Ok(());
    }

    let (api_token, zone, domain) = match args.config {
        Some(c) => {
            let config_str = read_file(&c)?;
            let config: Config = toml::from_str(&config_str)?;
            (config.api_token, config.zone, config.domain)
        }
        None => (
            args.api_token.expect("API token is not set"),
            args.zone.expect("Zone is not set"),
            args.domain.expect("Domain is not set"),
        ),
    };

    update(&current_ip, &api_token, &zone, &domain)?;

    log::info!(
        "Successfully updated the A record for {} to {}",
        &domain, &current_ip
    );

    if should_use_cache {
        log::info!(
            "Saving current IP {} to cache file {:?}...",
            &current_ip,
            &args.cache.clone().unwrap()
        );
        write_file(&args.cache.unwrap(), &current_ip)?;
    }

    Ok(())
}

fn update(
    current_ip: &str,
    api_token: &str,
    zone: &str,
    domain: &str,
) -> Result<()> {
    let zone_id = get_zone_identifier(&zone, &api_token).context("Error getting the zone identifier")?;
    let record_id = get_dns_record_id(&zone_id, &domain, &api_token).context("Error getting the DNS record ID")?;

    update_ddns(
        &current_ip,
        &domain,
        &zone_id,
        &record_id,
        &api_token,
    ).context("Error updating the DNS record")?;

    Ok(())
}
