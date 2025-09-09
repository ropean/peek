use anyhow::Result;
use std::net::IpAddr;
use trust_dns_resolver::config::*;
use trust_dns_resolver::TokioAsyncResolver;
use local_ip_address::local_ip;

pub async fn get_local_ips() -> Vec<String> {
    let mut ips = Vec::new();
    
    // Get local network IP (the one used for internet)
    if let Ok(local_ip) = local_ip() {
        ips.push(format!("Local: {}", local_ip));
    }
    
    // Try to get public IP
    if let Ok(public_ip) = get_public_ip().await {
        ips.push(format!("Public: {}", public_ip));
    }
    
    
    // Add hostname info
    if let Ok(hostname) = hostname::get() {
        if let Ok(hostname_str) = hostname.into_string() {
            ips.push(format!("Hostname: {}", hostname_str));
        }
    }
    
    ips
}

async fn get_public_ip() -> Result<String> {
    // Try multiple IP detection services for reliability
    let services = [
        "https://api.ipify.org",
        "https://icanhazip.com",
        "https://ipecho.net/plain",
    ];
    
    for service in &services {
        if let Ok(response) = reqwest::get(*service).await {
            if let Ok(ip_text) = response.text().await {
                let ip = ip_text.trim();
                if !ip.is_empty() && ip.parse::<IpAddr>().is_ok() {
                    return Ok(ip.to_string());
                }
            }
        }
    }
    
    Err(anyhow::anyhow!("Failed to get public IP"))
}

pub async fn resolve_domain(domain: &str) -> Result<Vec<String>> {
    if domain.is_empty() {
        return Ok(vec![]);
    }

    // Check if it's already an IP address
    if let Ok(ip) = domain.parse::<IpAddr>() {
        return Ok(vec![ip.to_string()]);
    }

    // Resolve domain to IP addresses
    let resolver = TokioAsyncResolver::tokio(ResolverConfig::default(), ResolverOpts::default());
    
    let mut ips = Vec::new();
    
    // Try IPv4 lookup
    if let Ok(response) = resolver.ipv4_lookup(domain).await {
        for ip in response.iter() {
            ips.push(ip.to_string());
        }
    }
    
    // Try IPv6 lookup
    if let Ok(response) = resolver.ipv6_lookup(domain).await {
        for ip in response.iter() {
            ips.push(ip.to_string());
        }
    }
    
    if ips.is_empty() {
        ips.push("Resolution failed".to_string());
    }
    
    Ok(ips)
}
