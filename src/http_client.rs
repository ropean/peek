use anyhow::Result;
use reqwest::{Client, Method};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use url::Url;

#[derive(Debug, Clone)]
pub struct RequestConfig {
    pub url: String,
    pub use_ssl: bool,
    pub use_post: bool,
    pub allow_redirects: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpResponse {
    pub status_code: u16,
    pub status_text: String,
    pub headers: HashMap<String, String>,
    pub body: String,
    pub client_ips: Vec<String>,
    pub server_ips: Vec<String>,
    pub requested_url: String,  // The URL we originally requested
    pub final_url: String,      // The final URL after redirects
}

pub struct HttpClient {
    client: Client,
}

impl HttpClient {
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .danger_accept_invalid_certs(true)
            .build()
            .expect("Failed to create HTTP client");

        Self { client }
    }

    pub async fn make_request(&self, config: RequestConfig) -> Result<HttpResponse> {
        let url = self.parse_url(&config.url, config.use_ssl)?;
        let requested_url = url.to_string(); // Store the original requested URL
        
        // Get client IPs
        let client_ips = crate::network_utils::get_local_ips().await;
        
        // Get server IPs
        let server_ips = crate::network_utils::resolve_domain(&url.host_str().unwrap_or("")).await?;

        // Create a client with appropriate redirect policy
        let client = if config.allow_redirects {
            // Use the default client (follows redirects)
            &self.client
        } else {
            // Create a temporary client that doesn't follow redirects
            &Client::builder()
                .timeout(Duration::from_secs(30))
                .danger_accept_invalid_certs(true)
                .redirect(reqwest::redirect::Policy::none())
                .build()
                .expect("Failed to create no-redirect HTTP client")
        };

        // Create request
        let method = if config.use_post { Method::POST } else { Method::GET };
        
        let mut request = client.request(method, url.as_str());
        
        if config.use_post {
            request = request.body("");
        }

        // Execute request  
        let response = request.send().await?;
        
        let status_code = response.status().as_u16();
        let status_text = response.status().to_string();
        let final_url = response.url().to_string();

        // Extract headers
        let mut headers = HashMap::new();
        for (name, value) in response.headers() {
            headers.insert(
                name.to_string(),
                value.to_str().unwrap_or("").to_string(),
            );
        }

        // Get response body
        let body = response.text().await?;

        Ok(HttpResponse {
            status_code,
            status_text,
            headers,
            body,
            client_ips,
            server_ips,
            requested_url,
            final_url,
        })
    }

    fn parse_url(&self, url: &str, use_ssl: bool) -> Result<Url> {
        let url = url.trim();
        
        // URL should already be cleaned by the UI, just add the appropriate protocol
        let final_url = if use_ssl {
            format!("https://{}", url)
        } else {
            format!("http://{}", url)
        };

        Ok(Url::parse(&final_url)?)
    }
}
