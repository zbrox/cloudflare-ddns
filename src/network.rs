use serde_derive::{Deserialize, Serialize};
use anyhow::{anyhow};

#[derive(Deserialize, Debug)]
struct CloudflareListResponse {
    success: bool,
    errors: Vec<String>,
    result: Option<Vec<ObjectWithId>>,
}

#[derive(Deserialize, Debug)]
struct CloudflareUpdateResponse {
    success: bool,
    errors: Vec<String>,
    result: ObjectWithId,
}

#[derive(Deserialize, Debug, PartialEq)]
struct ObjectWithId {
    id: String,
}

#[derive(Serialize, Debug)]
struct UpdateIpData {
    id: String,
    r#type: String,
    name: String,
    content: String,
}

pub fn get_zone_identifier(zone: &str, email: &str, key: &str) -> anyhow::Result<String> {
    let client = reqwest::blocking::Client::new();
    let url = format!("https://api.cloudflare.com/client/v4/zones?name={}", zone);

    let response = client
        .get(&url)
        .header("X-Auth-Email", email)
        .header("X-Auth-Key", key)
        .header("Content-Type", "application/json")
        .send()?;

    if response.status() != 200 {
        return Err(anyhow!("API Error: HTTP {}", response.status()));
    }

    let body: CloudflareListResponse = response
        .json()?;

    if !body.success {
        let err: String = body
            .errors
            .iter()
            .map(|s| format!("{}\n", s.to_owned()))
            .collect();
        return Err(anyhow!("API Error: {}", err));
    }

    let zones = match body.result {
        Some(v) => v,
        None => { 
            return Err(anyhow!("No zones returned"))
        },
    };

    Ok(zones[0].id.clone())
}

pub fn get_dns_record_id(
    zone_id: &str,
    domain: &str,
    email: &str,
    key: &str,
) -> anyhow::Result<String> {
    let client = reqwest::blocking::Client::new();
    let url = format!(
        "https://api.cloudflare.com/client/v4/zones/{}/dns_records?name={}",
        zone_id, domain
    );

    let response = client
        .get(&url)
        .header("X-Auth-Email", email)
        .header("X-Auth-Key", key)
        .header("Content-Type", "application/json")
        .send()?;

    if response.status() != 200 {
        return Err(anyhow!("API Error: HTTP {}", response.status()));
    }
        
    let body: CloudflareListResponse = response.json()?;

    if !body.success {
        let err: String = body
            .errors
            .iter()
            .map(|s| format!("{}\n", s.to_owned()))
            .collect();
        return Err(anyhow!("API Error: {}", err));
    }

    let records = match body.result {
        Some(v) => v,
        None => { 
            return Err(anyhow!("No DNS records returned"))
        },
    };

    let id = match records.first() {
        Some(v) => v.id.clone(),
        None => {
            return Err(anyhow!(
                "Unexpected API result for DNS record. Check if you passed the right options."
            ))
        }
    };

    Ok(id)
}

pub fn get_current_ip() -> anyhow::Result<String> {
    Ok(reqwest::blocking::Client::new()
        .get("http://ipv4.icanhazip.com")
        .send()?
        .text()?
        .trim()
        .into())
}

pub fn update_ddns(
    ip: &str,
    domain: &str,
    zone_id: &str,
    record_id: &str,
    email: &str,
    key: &str,
) -> anyhow::Result<()> {
    let client = reqwest::blocking::Client::new();
    let url = format!(
        "https://api.cloudflare.com/client/v4/zones/{}/dns_records/{}",
        zone_id, record_id
    );

    let update_data = UpdateIpData {
        id: zone_id.to_owned(),
        r#type: "A".to_owned(),
        name: domain.to_owned(),
        content: ip.to_owned(),
    };

    let response = client
        .put(&url)
        .header("X-Auth-Email", email)
        .header("X-Auth-Key", key)
        .header("Content-Type", "application/json")
        .json(&update_data)
        .send()?;

    if response.status() != 200 {
        return Err(anyhow!("API Error: HTTP {}", response.status()));
    }

    let body: CloudflareUpdateResponse = response
        .json()?;

    if !body.success {
        let err: String = body
            .errors
            .iter()
            .map(|s| format!("{}\n", s.to_owned()))
            .collect();
        return Err(anyhow!("Unsuccessful update of DNS record: {}", err));
    }

    Ok(())
}
