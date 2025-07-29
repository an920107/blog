#[derive(Clone)]
pub struct ServerConfiguration {
    pub host: String,
    pub port: u16,
}

impl ServerConfiguration {
    pub fn new() -> Self {
        let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = std::env::var("PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse::<u16>()
            .unwrap();

        Self { host, port }
    }
}
