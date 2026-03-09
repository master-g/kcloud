//! Configuration management module

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Kloud configuration
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Config {
	/// LLM configuration
	#[serde(default)]
	pub llm: LlmConfig,

	/// Agent configuration
	#[serde(default)]
	pub agent: AgentConfig,

	/// Tools configuration
	#[serde(default)]
	pub tools: ToolsConfig,
}

impl Config {
	/// Load configuration from file and environment variables
	pub fn load() -> Result<Self, crate::Error> {
		let mut config = Self::load_from_file()?;

		// Override with environment variables
		config.llm.load_from_env();
		config.agent.load_from_env();
		config.tools.load_from_env();

		Ok(config)
	}

	/// Load configuration from default locations
	fn load_from_file() -> Result<Self, crate::Error> {
		let config_paths = Self::config_paths();

		for path in config_paths {
			if path.exists() {
				let content = std::fs::read_to_string(&path).map_err(|e| {
					crate::Error::Config(crate::error::ConfigError::ReadError(e.to_string()))
				})?;

				let config: Config = toml::from_str(&content).map_err(|e| {
					crate::Error::Config(crate::error::ConfigError::ParseError(e.to_string()))
				})?;

				return Ok(config);
			}
		}

		// Return default config if no config file found
		Ok(Config::default())
	}

	/// Get configuration file paths in order of priority
	fn config_paths() -> Vec<PathBuf> {
		let mut paths = Vec::new();

		// 1. Environment variable
		if let Ok(path) = std::env::var("KLOUD_CONFIG_FILE") {
			paths.push(path.into());
		}

		// 2. User config directory
		if let Some(dir) = directories::ProjectDirs::from("com", "kloud", "kloud") {
			paths.push(dir.config_dir().join("config.toml"));
		}

		// 3. Home directory
		if let Ok(home) = std::env::var("HOME") {
			paths.push(PathBuf::from(home).join(".config/kloud/config.toml"));
		}

		paths
	}
}

/// LLM configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmConfig {
	/// API key for LLM service
	#[serde(default)]
	pub api_key: Option<String>,

	/// Base URL for API endpoint
	#[serde(default = "default_api_base_url")]
	pub api_base_url: String,

	/// Model name to use
	#[serde(default = "default_model")]
	pub model: String,

	/// Maximum tokens in context
	#[serde(default = "default_max_tokens")]
	pub max_tokens: u32,

	/// Temperature for generation
	#[serde(default = "default_temperature")]
	pub temperature: f32,
}

impl Default for LlmConfig {
	fn default() -> Self {
		Self {
			api_key: None,
			api_base_url: default_api_base_url(),
			model: default_model(),
			max_tokens: default_max_tokens(),
			temperature: default_temperature(),
		}
	}
}

impl LlmConfig {
	fn load_from_env(&mut self) {
		if let Ok(v) = std::env::var("KLOUD_API_KEY") {
			self.api_key = Some(v);
		}
		if let Ok(v) = std::env::var("KLOUD_API_BASE_URL") {
			self.api_base_url = v;
		}
		if let Ok(v) = std::env::var("KLOUD_MODEL") {
			self.model = v;
		}
	}
}

/// Agent configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
	/// Working directory for agent operations
	#[serde(default = "default_workdir")]
	pub workdir: PathBuf,

	/// Maximum concurrent tasks
	#[serde(default = "default_max_concurrent")]
	pub max_concurrent: usize,

	/// Default task timeout in seconds
	#[serde(default = "default_task_timeout")]
	pub task_timeout: u64,
}

impl Default for AgentConfig {
	fn default() -> Self {
		Self {
			workdir: default_workdir(),
			max_concurrent: default_max_concurrent(),
			task_timeout: default_task_timeout(),
		}
	}
}

impl AgentConfig {
	fn load_from_env(&mut self) {
		if let Ok(v) = std::env::var("KLOUD_WORKDIR") {
			self.workdir = v.into();
		}
		if let Ok(v) = std::env::var("KLOUD_MAX_CONCURRENT") {
			if let Ok(n) = v.parse() {
				self.max_concurrent = n;
			}
		}
		if let Ok(v) = std::env::var("KLOUD_TASK_TIMEOUT") {
			if let Ok(n) = v.parse() {
				self.task_timeout = n;
			}
		}
	}
}

/// Tools configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolsConfig {
	/// Allowed commands for bash tool (empty = all allowed)
	#[serde(default)]
	pub allowed_commands: Vec<String>,

	/// Enable path security check
	#[serde(default = "default_true")]
	pub path_security: bool,
}

impl Default for ToolsConfig {
	fn default() -> Self {
		Self {
			allowed_commands: Vec::new(),
			path_security: true,
		}
	}
}

impl ToolsConfig {
	fn load_from_env(&mut self) {
		// ToolsConfig currently has no env overrides
		// This method is here for future extensibility
	}
}

// Default value functions
fn default_api_base_url() -> String {
	"https://api.openai.com/v1".to_string()
}

fn default_model() -> String {
	"gpt-4".to_string()
}

fn default_max_tokens() -> u32 {
	8000
}

fn default_temperature() -> f32 {
	0.7
}

fn default_workdir() -> PathBuf {
	std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

fn default_max_concurrent() -> usize {
	5
}

fn default_task_timeout() -> u64 {
	300
}

fn default_true() -> bool {
	true
}
