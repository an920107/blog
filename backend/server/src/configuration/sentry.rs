#[derive(Clone)]
pub struct SentryConfiguration {
    pub dsn: String,
    pub options: sentry::ClientOptions,
}

impl SentryConfiguration {
    pub fn new() -> Self {
        let dsn = std::env::var("SENTRY_DSN").unwrap_or("".to_string());

        Self {
            dsn: dsn,
            options: sentry::ClientOptions {
                release: sentry::release_name!(),
                traces_sample_rate: 1.0,
                send_default_pii: true,
                max_request_body_size: sentry::MaxRequestBodySize::Always,
                attach_stacktrace: true,
                ..Default::default()
            },
        }
    }
}
