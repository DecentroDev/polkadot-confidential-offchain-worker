use scale_info::prelude::string::{String, ToString};
use scale_info::prelude::vec::Vec;
use sp_runtime::offchain::{http, Duration};

/// A struct representing an HTTP request configuration.
pub struct HttpRequest {
    pub url: String,
    pub headers: Vec<(String, String)>,
}

impl HttpRequest {
    /// Create a new `HttpRequest` with default values.
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
            headers: Vec::new(),
        }
    }

    /// Add a header to the request.
    pub fn add_header(mut self, key: &str, value: &str) -> Self {
        self.headers.push((key.to_string(), value.to_string()));
        self
    }
}

/// A trait for fetching data with flexible HTTP requests.
pub trait OffchainFetcher {
    /// Send an HTTP request and return the response body.
    fn fetch(request: HttpRequest) -> Result<Vec<u8>, &'static str> {
        let mut request_builder = http::Request::get(&request.url);

        // Set headers
        for (key, value) in request.headers.iter() {
            request_builder = request_builder.add_header(key, value);
        }
        let deadline = sp_io::offchain::timestamp().add(Duration::from_millis(2_000));
        // Send request
        let pending = request_builder
            .deadline(deadline) // 5-second timeout
            .send()
            .map_err(|_| "Failed to send HTTP request")?;

        let response = pending.wait().map_err(|_| "No response received")?;

        if response.code != 200 {
            return Err("Non-200 response code received");
        }

        Ok(response.body().collect::<Vec<u8>>())
    }

    /// Send an HTTP request and return the response as a UTF-8 string.
    fn fetch_string(request: HttpRequest) -> Result<String, &'static str> {
        let bytes = Self::fetch(request)?;
        String::from_utf8(bytes).map_err(|_| "Failed to parse response as UTF-8")
    }
}

/// A default implementation of `OffchainFetcher`.
pub struct DefaultOffchainFetcher;

impl OffchainFetcher for DefaultOffchainFetcher {}
