use eframe::egui;
use std::sync::Arc;
use std::sync::mpsc;
use std::thread;

use crate::http_client::{HttpClient, HttpResponse, RequestConfig};

pub struct PeekApp {
    http_client: Arc<std::sync::Mutex<HttpClient>>,
    url_input: String,
    use_ssl: bool,
    use_post: bool,
    allow_redirects: bool,
    query_all: bool,
    response_text: String,
    is_loading: bool,
    response_receiver: Option<mpsc::Receiver<Result<Vec<HttpResponse>, String>>>,
    last_url_input: String, // Track previous URL input to detect changes
    first_frame: bool, // Track if this is the first frame for centering
}

impl PeekApp {
    pub fn new(http_client: Arc<std::sync::Mutex<HttpClient>>) -> Self {
        Self {
            http_client,
            url_input: "aceapp.dev".to_string(),
            use_ssl: true,
            use_post: false,
            allow_redirects: false,
            query_all: true,
            response_text: String::new(),
            is_loading: false,
            response_receiver: None,
            last_url_input: "aceapp.dev".to_string(),
            first_frame: true,
        }
    }

    fn is_top_level_domain(url: &str) -> bool {
        // Count the number of dots in the domain
        // A top-level domain has exactly one dot (e.g., "aceapp.dev")
        // A subdomain has more than one dot (e.g., "www.aceapp.dev")
        let parts: Vec<&str> = url.split('.').collect();
        parts.len() == 2
    }

    fn make_request(&mut self) {
        if self.is_loading {
            return;
        }

        self.is_loading = true;
        self.response_text = "Requesting...".to_string();

        // Determine which URLs to query
        let mut urls = vec![self.url_input.clone()];

        // If query_all is enabled and this is a top-level domain, also query www.domain
        if self.query_all && Self::is_top_level_domain(&self.url_input) {
            urls.push(format!("www.{}", self.url_input));
        }

        let configs: Vec<RequestConfig> = urls.into_iter().map(|url| {
            RequestConfig {
                url,
                use_ssl: self.use_ssl,
                use_post: self.use_post,
                allow_redirects: self.allow_redirects,
            }
        }).collect();

        let http_client = self.http_client.clone();
        let (tx, rx) = mpsc::channel();
        self.response_receiver = Some(rx);

        // Spawn a new thread with its own tokio runtime
        thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
            let responses = rt.block_on(async {
                let client = http_client.lock().unwrap();
                let mut responses = Vec::new();

                for config in configs {
                    match client.make_request(config).await {
                        Ok(response) => responses.push(response),
                        Err(e) => {
                            // If one request fails, we still want to return partial results
                            // But we'll return an error if all fail
                            eprintln!("Request failed: {}", e);
                        }
                    }
                }

                responses
            });

            let result = if responses.is_empty() {
                Err("Error: All requests failed".to_string())
            } else {
                Ok(responses)
            };

            let _ = tx.send(result);
        });
    }

    fn process_url_input(&mut self) {
        // Only process if URL has changed
        if self.url_input == self.last_url_input {
            return;
        }
        
        let trimmed_url = self.url_input.trim();
        
        // Check if URL includes protocol and update SSL checkbox accordingly
        if trimmed_url.starts_with("https://") {
            self.use_ssl = true;
            self.url_input = trimmed_url.replace("https://", "");
        } else if trimmed_url.starts_with("http://") {
            self.use_ssl = false;
            self.url_input = trimmed_url.replace("http://", "");
        } else {
            // No protocol specified, just trim but keep SSL checkbox unchanged
            self.url_input = trimmed_url.to_string();
        }
        
        // Update the last URL input to track changes
        self.last_url_input = self.url_input.clone();
    }

    fn format_response(&self, response: &HttpResponse) -> String {
        let mut output = String::new();

        output.push_str(&format!("Your requested address is: {}\n", response.requested_url));
        if response.requested_url != response.final_url {
            output.push_str(&format!("Final URL (after redirects): {}\n", response.final_url));
        }
        output.push('\n');

        // Client IPs
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

        // Server IPs
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

        // Status code
        output.push_str(&format!("Responded status code: {}\n\n", response.status_code));

        // Headers
        if !response.headers.is_empty() {
            output.push_str("Responded headers:\n");
            for (key, value) in &response.headers {
                output.push_str(&format!("{}: {}\n", key, value));
            }
            output.push('\n');
        }

        // Response body (only show if not empty)
        if !response.body.trim().is_empty() {
            output.push_str("Responded source code:\n");
            output.push_str(&response.body);
        }

        output
    }

    fn format_responses(&self, responses: &[HttpResponse]) -> String {
        if responses.len() == 1 {
            return self.format_response(&responses[0]);
        }

        let mut output = String::new();
        for (i, response) in responses.iter().enumerate() {
            output.push_str(&format!("========== Request {} ==========\n\n", i + 1));
            output.push_str(&self.format_response(response));
            if i < responses.len() - 1 {
                output.push_str("\n\n");
            }
        }
        output
    }
}

impl eframe::App for PeekApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Center window on first frame
        if self.first_frame {
            ctx.send_viewport_cmd(egui::ViewportCommand::CenterOnScreen);
            self.first_frame = false;
        }

        // Check for response from background thread
        if let Some(receiver) = &self.response_receiver {
            if let Ok(result) = receiver.try_recv() {
                match result {
                    Ok(responses) => {
                        self.response_text = self.format_responses(&responses);
                    }
                    Err(e) => {
                        self.response_text = e;
                    }
                }
                self.is_loading = false;
                self.response_receiver = None;
            }
        }

        // Request a repaint to keep checking for responses
        if self.is_loading {
            ctx.request_repaint();
        }
        
        // Top panel for controls (fixed height)
        egui::TopBottomPanel::top("top_panel").exact_height(60.0).show(ctx, |ui| {
            ui.add_space(10.0);
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 10.0;
                
                // URL input
                let url_response = ui.add(
                    egui::TextEdit::singleline(&mut self.url_input)
                        .desired_width(500.0)
                        .font(egui::TextStyle::Heading)
                );
                
                // Process URL input changes only when text changes
                if url_response.changed() {
                    self.process_url_input();
                }
                
                // Handle Enter key
                if url_response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    self.make_request();
                }
                
                // SSL checkbox
                ui.checkbox(&mut self.use_ssl, "SSL");

                // POST checkbox
                ui.checkbox(&mut self.use_post, "Post");

                // Redirect checkbox
                ui.checkbox(&mut self.allow_redirects, "Redirect");

                // All checkbox
                ui.checkbox(&mut self.query_all, "All");

                // Request button
                let button_text = if self.is_loading { "Loading..." } else { "Request" };
                if ui.add_enabled(!self.is_loading, egui::Button::new(button_text)).clicked() {
                    self.make_request();
                }
            });
            ui.add_space(10.0);
        });

        // Bottom panel for copyright (fixed height)
        egui::TopBottomPanel::bottom("bottom_panel").exact_height(30.0).show(ctx, |ui| {
            ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::LeftToRight), |ui| {
                ui.label(egui::RichText::new("All rights reserved © Ropean 2025")
                    .size(11.0)
                    .color(egui::Color32::GRAY));
            });
        });

        // Central panel for response (fills all remaining space)
        egui::CentralPanel::default().show(ctx, |ui| {
            let available_height = ui.available_height();
            let available_width = ui.available_width();

            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    // Calculate rows based on available height (approximate)
                    let line_height = 16.0; // Approximate height per line in monospace
                    let rows = (available_height / line_height).max(10.0) as usize;

                    ui.add(
                        egui::TextEdit::multiline(&mut self.response_text)
                            .font(egui::TextStyle::Monospace)
                            .desired_width(available_width)
                            .desired_rows(rows)
                    );
                });
        });
    }
}
