use crate::cache::Cache;
use anyhow::Result;
use reqwest::Client;
use tracing::debug;

/// HTTP client with built-in caching for API exploration.
pub struct Explorer {
    client: Client,
    cache: Cache,
}

impl Explorer {
    pub fn new() -> Self {
        let client = Client::builder()
            .user_agent("undiscovered/0.1 (scientific data exploration)")
            .gzip(true)
            .brotli(true)
            .timeout(std::time::Duration::from_secs(120))
            .build()
            .expect("failed to build HTTP client");

        Self {
            client,
            cache: Cache::new(24), // 24-hour TTL
        }
    }

    /// Fetch URL with caching. Returns raw response body as string.
    pub async fn fetch(&self, url: &str) -> Result<String> {
        if let Some(cached) = self.cache.get(url) {
            debug!("cache hit: {url}");
            return Ok(cached);
        }

        debug!("fetching: {url}");
        let response = self.client.get(url).send().await?;
        let status = response.status();

        if !status.is_success() {
            anyhow::bail!("HTTP {status} from {url}");
        }

        let body = response.text().await?;
        self.cache.set(url, &body)?;
        Ok(body)
    }

    /// Fetch and parse JSON.
    pub async fn fetch_json<T: serde::de::DeserializeOwned>(&self, url: &str) -> Result<T> {
        let body = self.fetch(url).await?;
        let parsed = serde_json::from_str(&body)?;
        Ok(parsed)
    }

    /// Fetch without caching (for real-time feeds).
    pub async fn fetch_live(&self, url: &str) -> Result<String> {
        let response = self.client.get(url).send().await?;
        let status = response.status();

        if !status.is_success() {
            anyhow::bail!("HTTP {status} from {url}");
        }

        Ok(response.text().await?)
    }

    /// POST form-encoded data with caching (keyed by url + body).
    pub async fn post_form(&self, url: &str, params: &[(&str, &str)]) -> Result<String> {
        // Build a cache key from url + sorted params
        let mut cache_key = url.to_string();
        for (k, v) in params {
            cache_key.push_str(&format!("&{k}={v}"));
        }

        if let Some(cached) = self.cache.get(&cache_key) {
            debug!("cache hit: {cache_key}");
            return Ok(cached);
        }

        debug!("POST {url}");
        let response = self.client.post(url).form(params).send().await?;
        let status = response.status();

        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            anyhow::bail!("HTTP {status} from POST {url}: {body}");
        }

        let body = response.text().await?;
        self.cache.set(&cache_key, &body)?;
        Ok(body)
    }

    pub fn cache(&self) -> &Cache {
        &self.cache
    }
}
