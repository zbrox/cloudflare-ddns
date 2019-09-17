mod network;
mod file;

use std::path::PathBuf;
use quicli::prelude::*;
use structopt::StructOpt;
use human_panic::{setup_panic};
use network::{get_zone_identifier, get_dns_record_id, get_current_ip};

#[derive(Debug, StructOpt)]
/// Inform Cloudflare's DDNS service of the current IP address for your domain
struct Cli {
    /// Your Cloudflare login email
    #[structopt(long = "email", short = "e")]
    email: String,

    /// The auth key you need to generate in your Cloudflare profile
    #[structopt(long = "key", short = "k")]
    auth_key: String,

    /// The zone in which your domain is (usually that is your domain without the subdomain)
    #[structopt(long = "zone", short = "z")]
    zone: String,

    /// The domain for which you want to report the current IP address
    #[structopt(long = "domain", short = "d")]
    domain: String,

    /// Cache file for previously reported IP address (if skipped the IP will be reported on every execution)
    #[structopt(long = "cache", short = "c")]
    cache: Option<PathBuf>,

    // TODO: implement logging later
    /// Log file (if skipped stdOut is used)
    #[structopt(long = "log", short = "l")]
    log_file: Option<PathBuf>,
}

fn main() -> CliResult {
    setup_panic!();

    let args = Cli::from_args();
    Ok(())
}
