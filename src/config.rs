pub struct Server {
    address: (String, u16),
    log_level: String,
    clickhouse_url: String,
    clickhouse_user: String,
    clickhouse_password: String,
}

impl Server {
    pub fn new(
        address: (String, u16),
        log_level: String,
        clickhouse_url: String,
        clickhouse_user: String,
        clickhouse_password: String,
    ) -> Self {
        Server {
            address,
            log_level,
            clickhouse_url,
            clickhouse_user,
            clickhouse_password,
        }
    }

    pub fn address(&self) -> (String, u16) {
        self.address.clone()
    }

    pub fn log_level(&self) -> &str {
        &self.log_level
    }

    pub fn clickhouse_url(&self) -> &str {
        &self.clickhouse_url
    }

    pub fn clickhouse_user(&self) -> &str {
        &self.clickhouse_user
    }

    pub fn clickhouse_password(&self) -> &str {
        &self.clickhouse_password
    }
}

impl std::fmt::Display for Server {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", ascii(self))
    }
}

pub fn from_env() -> Server {
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .unwrap_or(8080);
    let log_level = std::env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string());
    let clickhouse_url =
        std::env::var("CLICKHOUSE_URL").unwrap_or_else(|_| "http://localhost:8123".to_string());
    let clickhouse_user =
        std::env::var("CLICKHOUSE_USER").unwrap_or_else(|_| "default".to_string());
    let clickhouse_password =
        std::env::var("CLICKHOUSE_PASSWORD").unwrap_or_else(|_| String::new());

    Server::new(
        ("0.0.0.0".to_string(), port),
        log_level,
        clickhouse_url,
        clickhouse_user,
        clickhouse_password,
    )
}

fn ascii(server: &Server) -> String {
    let (_, port) = server.address();

    let url = format!("http://localhost:{port}");
    let version = env!("CARGO_PKG_VERSION");
    format!(
        "
        ┌─────────────────────────────────┐
        │                                 │
        │   ╔═╗┬ ┬┌┬┐┌┬┐┬ ┬              │
        │   ║  ├─┤ │ ││││ │              │
        │   ╚═╝┴ ┴ ┴ ┴ ┴└─┘              │
        │                                 │
        └─────────────────────────────────┘
        
        Server running at: {url}
        Version: {version}
        "
    )
}
