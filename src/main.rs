//! Kloud - Main entry point

use kloud::{logging, Result};

#[tokio::main]
async fn main() -> Result<()> {
	// Initialize logging from environment variables
	// Use KLOUD_LOG_PRETTY, KLOUD_LOG_DIR, KLOUD_LOG_COLORED to customize
	logging::init_from_env();

	tracing::info!("Starting kloud...");

	// TODO: Implement CLI and main loop
	tracing::info!("CLI not yet implemented");

	Ok(())
}
