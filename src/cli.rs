use clap::{Parser, Subcommand};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::PathBuf;

use crate::http_client::{HttpClient, HttpResponse, RequestConfig};

#[derive(Parser, Debug)]
#[command(name = "peek", about = "Peek - GUI + CLI HTTP inspector")]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Single or multiple URL requests
    Cli {
        /// One or more URLs to request
        #[arg(value_name = "URL")]
        urls: Vec<String>,

        /// Force SSL (https)
        #[arg(long = "ssl")]
        ssl: bool,

        /// Disable SSL (use http)
        #[arg(long = "no-ssl")]
        no_ssl: bool,

        /// HTTP method (GET or POST)
        #[arg(short = 'X', long = "method", default_value = "GET")]
        method: String,

        /// Request body (implies POST)
        #[arg(short = 'd', long = "data")]
        data: Option<String>,

        /// Follow redirects
        #[arg(short = 'r', long = "follow-redirects")]
        follow_redirects: bool,

        /// Also query www.<domain> if top-level
        #[arg(short = 'a', long = "all")]
        all: bool,

        /// Timeout in seconds
        #[arg(short = 't', long = "timeout")]
        timeout: Option<u64>,

        /// Output file (writes text or JSON)
        #[arg(short = 'o', long = "output")]
        output: Option<PathBuf>,

    /// Format: text or json
    #[arg(short = 'f', long = "format", default_value = "text")]
    format: String,

        /// Quiet
        #[arg(short = 'q', long = "quiet")]
        quiet: bool,

        /// Verbose
        #[arg(long = "verbose")]
        verbose: bool,
    },

    /// Batch mode: read URLs from a file or stdin
    Batch {
        /// File to read URLs from (one per line). If omitted, read from stdin.
        #[arg(value_name = "FILE")]
        file: Option<PathBuf>,

        /// Other flags follow the same meaning as `cli` subcommand
        #[arg(short = 'r', long = "follow-redirects")]
        follow_redirects: bool,

        #[arg(short = 'a', long = "all")]
        all: bool,

    #[arg(short = 't', long = "timeout")]
        timeout: Option<u64>,

        #[arg(short = 'c', long = "concurrency", default_value_t = 1)]
        concurrency: usize,

        #[arg(short = 'o', long = "output")]
        output: Option<PathBuf>,

        /// Format: text or json
        #[arg(short = 'f', long = "format", default_value = "text")]
        format: String,
    },
}

fn normalize_url_and_ssl(raw: &str, ssl_flag: Option<bool>) -> (String, bool) {
    let s = raw.trim();
    if s.starts_with("https://") {
        (s.replacen("https://", "", 1), true)
    } else if s.starts_with("http://") {
        (s.replacen("http://", "", 1), false)
    } else if let Some(force_ssl) = ssl_flag {
        (s.to_string(), force_ssl)
    } else {
        // default to true (keep GUI defaults)
        (s.to_string(), true)
    }
}

fn format_response_text(response: &HttpResponse) -> String {
    let mut output = String::new();

    output.push_str(&format!("Your requested address is: {}\n", response.requested_url));
    if response.requested_url != response.final_url {
        output.push_str(&format!("Final URL (after redirects): {}\n", response.final_url));
    }
    output.push('\n');

    if !response.client_ips.is_empty() {
        output.push_str("Your IP(s):");
        if response.client_ips.len() > 1 {
            output.push('\n');
            for ip in &response.client_ips {
                output.push_str(&format!("{}\n", ip));
            }
        } else {
            output.push_str(&format!(" {}\n", response.client_ips[0]));
        }
        output.push('\n');
    }

    if !response.server_ips.is_empty() {
        output.push_str("Responded IP(s):");
        if response.server_ips.len() > 1 {
            output.push('\n');
            for ip in &response.server_ips {
                output.push_str(&format!("{}\n", ip));
            }
        } else {
            output.push_str(&format!(" {}\n", response.server_ips[0]));
        }
        output.push('\n');
    }

    output.push_str(&format!("Responded status code: {}\n\n", response.status_code));

    if !response.headers.is_empty() {
        output.push_str("Responded headers:\n");
        for (key, value) in &response.headers {
            output.push_str(&format!("{}: {}\n", key, value));
        }
        output.push('\n');
    }

    if !response.body.trim().is_empty() {
        output.push_str("Responded source code:\n");
        output.push_str(&response.body);
    }

    output
}

pub fn run_from_args() -> i32 {
    // Manual pre-parse handling for -v/--version so it's available globally
    // and won't collide with subcommand flags in clap's help output.
    for a in std::env::args().skip(1) {
        if a == "-v" || a == "--version" {
            const TEMPLATE: &str = include_str!("./version_template.txt");
            let version = env!("CARGO_PKG_VERSION");
            let out = TEMPLATE.replace("${version}", version);
            println!("{}", out);
            return 0;
        }
    }

    let args = CliArgs::parse();

    match args.command {
        Some(Commands::Cli {
            urls,
            ssl,
            no_ssl,
            method,
            data,
            follow_redirects,
            all,
            timeout,
            output,
            format,
            quiet: _,
            verbose: _,
        }) => {
            let force_ssl = if ssl { Some(true) } else if no_ssl { Some(false) } else { None };

            if urls.is_empty() {
                eprintln!("No URL provided");
                return 2;
            }

            let client = HttpClient::new();
            let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");

            let mut results: Vec<HttpResponse> = Vec::new();

            for raw in urls {
                // possibly expand www if requested and top-level domain
                let (cleaned, use_ssl) = normalize_url_and_ssl(&raw, force_ssl);

                // If `all` and looks like top-level domain, query www.<domain> as well
                let mut targets = vec![(cleaned.clone(), use_ssl)];
                if all {
                    let parts: Vec<&str> = cleaned.split('.').collect();
                    if parts.len() == 2 {
                        targets.push((format!("www.{}", cleaned), use_ssl));
                    }
                }

                for (url, use_ssl) in targets {
                    let cfg = RequestConfig {
                        url: url.clone(),
                        use_ssl,
                        use_post: method.to_uppercase() == "POST" || data.is_some(),
                        allow_redirects: follow_redirects,
                        body: data.clone(),
                        timeout_secs: timeout,
                    };

                    match rt.block_on(async { client.make_request(cfg).await }) {
                        Ok(resp) => results.push(resp),
                        Err(e) => {
                            eprintln!("Request error for {}: {}", url, e);
                        }
                    }
                }
            }

            // Output results
            if format.to_lowercase() == "json" {
                match serde_json::to_string_pretty(&results) {
                    Ok(s) => {
                        if let Some(path) = output {
                            if let Ok(mut f) = File::create(path) {
                                let _ = f.write_all(s.as_bytes());
                            }
                        } else {
                            println!("{}", s);
                        }
                    }
                    Err(e) => eprintln!("Failed to serialize JSON: {}", e),
                }
            } else {
                let mut out = String::new();
                for (i, resp) in results.iter().enumerate() {
                    if results.len() > 1 {
                        out.push_str(&format!("========== Request {} =========={}\n\n", i + 1, ""));
                    }
                    out.push_str(&format_response_text(resp));
                    if i < results.len() - 1 {
                        out.push_str("\n\n");
                    }
                }

                if let Some(path) = output {
                    if let Ok(mut f) = File::create(path) {
                        let _ = f.write_all(out.as_bytes());
                    }
                } else {
                    println!("{}", out);
                }
            }

            0
        }
        Some(Commands::Batch {
            file,
            follow_redirects,
            all,
            timeout,
            concurrency: _,
            output,
            format,
        }) => {
            // Read URLs from file or stdin
            let reader: Box<dyn BufRead> = match file {
                Some(path) => match File::open(path) {
                    Ok(f) => Box::new(BufReader::new(f)),
                    Err(e) => {
                        eprintln!("Failed to open file: {}", e);
                        return 2;
                    }
                },
                None => Box::new(BufReader::new(io::stdin())),
            };

            let mut urls: Vec<String> = Vec::new();
            for line in reader.lines() {
                if let Ok(l) = line {
                    let s = l.trim();
                    if !s.is_empty() {
                        urls.push(s.to_string());
                    }
                }
            }

            if urls.is_empty() {
                eprintln!("No URLs provided for batch");
                return 2;
            }

            // Reuse CLI code path but without ssl forcing; default SSL true to match GUI
            let client = HttpClient::new();
            let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");

            let mut results: Vec<HttpResponse> = Vec::new();

            for raw in urls {
                let (cleaned, use_ssl) = normalize_url_and_ssl(&raw, None);
                let mut targets = vec![(cleaned.clone(), use_ssl)];
                if all {
                    let parts: Vec<&str> = cleaned.split('.').collect();
                    if parts.len() == 2 {
                        targets.push((format!("www.{}", cleaned), use_ssl));
                    }
                }

                for (url, use_ssl) in targets {
                    let cfg = RequestConfig {
                        url: url.clone(),
                        use_ssl,
                        use_post: false,
                        allow_redirects: follow_redirects,
                        body: None,
                        timeout_secs: timeout,
                    };

                    match rt.block_on(async { client.make_request(cfg).await }) {
                        Ok(resp) => results.push(resp),
                        Err(e) => {
                            eprintln!("Request error for {}: {}", url, e);
                        }
                    }
                }
            }

            if format.to_lowercase() == "json" {
                match serde_json::to_string_pretty(&results) {
                    Ok(s) => {
                        if let Some(path) = output {
                            if let Ok(mut f) = File::create(path) {
                                let _ = f.write_all(s.as_bytes());
                            }
                        } else {
                            println!("{}", s);
                        }
                    }
                    Err(e) => eprintln!("Failed to serialize JSON: {}", e),
                }
            } else {
                let mut out = String::new();
                for (i, resp) in results.iter().enumerate() {
                    if results.len() > 1 {
                        out.push_str(&format!("========== Request {} =========={}\n\n", i + 1, ""));
                    }
                    out.push_str(&format_response_text(resp));
                    if i < results.len() - 1 {
                        out.push_str("\n\n");
                    }
                }

                if let Some(path) = output {
                    if let Ok(mut f) = File::create(path) {
                        let _ = f.write_all(out.as_bytes());
                    }
                } else {
                    println!("{}", out);
                }
            }

            0
        }
        None => {
            // No subcommand -> GUI mode. Indicate caller to start GUI by returning code 127.
            127
        }
    }
}
