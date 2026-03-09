//! Logging configuration module
//!
//! Provides tracing subscriber configuration for console logging.

use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

/// Initialize logging with default configuration
pub fn init_default() {
	init("info", true);
}

/// Initialize logging from environment variables
pub fn init_from_env() {
	let level = std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());
	let pretty = std::env::var("KLOUD_LOG_PRETTY").map(|v| v == "true").unwrap_or(true);

	init(&level, pretty);
}

/// Initialize logging with the given level and format
fn init(level: &str, pretty: bool) {
	let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(level));

	let subscriber = tracing_subscriber::registry().with(env_filter);

	if pretty {
		subscriber
			.with(
				fmt::layer()
					.with_target(true)
					.with_thread_ids(true)
					.with_file(true)
					.with_line_number(true)
					.pretty(),
			)
			.init();
	} else {
		subscriber
			.with(
				fmt::layer()
					.with_target(true)
					.with_thread_ids(true)
					.with_file(true)
					.with_line_number(true),
			)
			.init();
	}
}
