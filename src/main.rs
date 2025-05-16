use chrono::Local;
use dotenv::dotenv;
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Deserialize)]
struct IpifyResponse {
    ip: String,
}

#[derive(Deserialize)]
struct DnsRecord {
    content: String,
    id: String,
    name: String,
    ttl: u32,
    proxied: bool,
}

#[derive(Deserialize)]
struct DnsRecordsListResponse {
    result: Vec<DnsRecord>,
}

#[derive(Serialize)]
struct UpdateDnsRecord {
    #[serde(rename = "type")]
    record_type: String,
    name: String,
    content: String,
    ttl: u32,
    proxied: bool,
}

fn get_env_var(key: &str) -> Result<String, String> {
    env::var(key).map_err(|_| format!("Environment variable {} not found", key))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load .env file if it exists
    dotenv().ok();

    // Retrieve configuration from environment variables
    let cf_api_token = get_env_var("CF_API_TOKEN")?;
    let zone_id = get_env_var("ZONE_ID")?;
    let domain = get_env_var("DOMAIN")?;
    let records_names = get_env_var("RECORDS_NAMES")?;

    log("Configuration loaded from environment variables");

    let client = Client::new();

    // 1. Get current public IP from ipify
    let ip_resp: IpifyResponse = client
        .get("https://api.ipify.org?format=json")
        .send()
        .await?
        .json()
        .await?;

    let current_ip = ip_resp.ip;
    log(&format!("Current IP: {}", current_ip));

    // 2. Get all DNS records for our zone
    let records_url = format!(
        "https://api.cloudflare.com/client/v4/zones/{}/dns_records/?type=A",
        zone_id
    );

    let dns_list_resp: DnsRecordsListResponse = client
        .get(&records_url)
        .bearer_auth(&cf_api_token)
        .send()
        .await?
        .json()
        .await?;

    // Parse subdomain names from records_names
    let subdomain_list: Vec<&str> = records_names.split(',').map(|s| s.trim()).collect();

    log(&format!("Checking {} subdomains...", subdomain_list.len()));

    // 3. Check and update each record
    for subdomain in subdomain_list {
        let full_domain = if subdomain == "*" {
            format!("*.{}", domain)
        } else {
            format!("{}.{}", subdomain, domain)
        };

        log(&format!("Checking domain: {}", full_domain));

        // Find matching record
        let matching_record = dns_list_resp
            .result
            .iter()
            .find(|record| record.name == full_domain);

        match matching_record {
            Some(record) => {
                if record.content != current_ip {
                    log(&format!(
                        "IP changed for {}: updating record...",
                        full_domain
                    ));

                    let record_url = format!(
                        "https://api.cloudflare.com/client/v4/zones/{}/dns_records/{}",
                        zone_id, record.id
                    );

                    let payload = UpdateDnsRecord {
                        record_type: "A".to_string(),
                        name: full_domain.clone(),
                        content: current_ip.clone(),
                        ttl: record.ttl,
                        proxied: record.proxied,
                    };

                    let patch_resp = client
                        .patch(&record_url)
                        .bearer_auth(&cf_api_token)
                        .json(&payload)
                        .send()
                        .await?;

                    if patch_resp.status() == StatusCode::OK {
                        log(&format!(
                            "Successfully updated DNS record for {} to {}",
                            full_domain, current_ip
                        ));
                    } else {
                        let status = patch_resp.status();
                        let err_text = patch_resp.text().await?;
                        eprintln!(
                            "[{} ERROR] Update failed for {} ({}): {}",
                            now(),
                            full_domain,
                            status,
                            err_text
                        );
                    }
                } else {
                    log(&format!("No IP change needed for {}", full_domain));
                }
            }
            None => {
                log(&format!("Warning: No A record found for {}", full_domain));
            }
        }
    }

    Ok(())
}

fn now() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

fn log(msg: &str) {
    println!("[{}] {}", now(), msg);
}
