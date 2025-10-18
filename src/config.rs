use std::time::Duration;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Config {
    pub base_url: String,
    pub api_key: String,
    pub timeout: Duration,
    pub retries: u32,
    pub backoff: BackoffConfig,
    pub cache_enabled: bool,
    pub cache_size: usize,
    pub cache_ttl: Duration,
    pub max_connections: usize,
    pub insecure: bool,
    pub client_cert: Option<PathBuf>,
    pub client_key: Option<PathBuf>,
    pub ca_cert: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub struct BackoffConfig {
    pub base_delay: Duration,
    pub max_delay: Duration,
    pub factor: f64,
}

impl Default for BackoffConfig {
    fn default() -> Self {
        Self {
            base_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(2),
            factor: 2.0,
        }
    }
}

impl Config {
    pub fn new(api_key: String) -> Self {
        Self {
            base_url: "http://localhost:8090".to_string(),
            api_key,
            timeout: Duration::from_millis(800),
            retries: 2,
            backoff: BackoffConfig::default(),
            cache_enabled: false,
            cache_size: 100,
            cache_ttl: Duration::from_secs(5),
            max_connections: 100,
            insecure: false,
            client_cert: None,
            client_key: None,
            ca_cert: None,
        }
    }
}

pub struct ConfigBuilder {
    config: Config,
}

impl ConfigBuilder {
    pub fn new(api_key: String) -> Self {
        Self {
            config: Config::new(api_key),
        }
    }

    pub fn base_url(mut self, url: String) -> Self {
        self.config.base_url = url;
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.config.timeout = timeout;
        self
    }

    pub fn retries(mut self, retries: u32) -> Self {
        self.config.retries = retries;
        self
    }

    pub fn backoff(mut self, backoff: BackoffConfig) -> Self {
        self.config.backoff = backoff;
        self
    }

    pub fn cache_enabled(mut self, enabled: bool) -> Self {
        self.config.cache_enabled = enabled;
        self
    }

    pub fn cache_size(mut self, size: usize) -> Self {
        self.config.cache_size = size;
        self
    }

    pub fn cache_ttl(mut self, ttl: Duration) -> Self {
        self.config.cache_ttl = ttl;
        self
    }

    pub fn max_connections(mut self, max_conns: usize) -> Self {
        self.config.max_connections = max_conns;
        self
    }

    pub fn insecure(mut self, insecure: bool) -> Self {
        self.config.insecure = insecure;
        self
    }

    pub fn client_cert(mut self, cert: PathBuf) -> Self {
        self.config.client_cert = Some(cert);
        self
    }

    pub fn client_key(mut self, key: PathBuf) -> Self {
        self.config.client_key = Some(key);
        self
    }

    pub fn ca_cert(mut self, cert: PathBuf) -> Self {
        self.config.ca_cert = Some(cert);
        self
    }

    pub fn build(self) -> Config {
        self.config
    }
}
