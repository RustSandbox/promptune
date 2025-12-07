use crate::{Error, Result};
use rig::completion::Prompt;
use rig::prelude::CompletionClient;
use rig::providers::gemini;
use std::fs;
use std::path::Path;

const DEFAULT_CACHE_DIR: &str = "./data/prompts";
const DEFAULT_MODEL: &str = "gemini-2.0-flash";

/// Configuration for prompt refinement.
#[derive(Clone)]
pub struct Config {
    pub cache_dir: String,
    pub model: String,
    pub api_key: String,
}

impl Config {
    /// Create config from `GEMINI_API_KEY` environment variable.
    pub fn from_env() -> Result<Self> {
        let api_key = std::env::var("GEMINI_API_KEY")
            .map_err(|_| Error::MissingEnvVar("GEMINI_API_KEY".into()))?;

        Ok(Self {
            cache_dir: DEFAULT_CACHE_DIR.into(),
            model: DEFAULT_MODEL.into(),
            api_key,
        })
    }

    /// Set cache directory.
    pub fn with_cache_dir(mut self, dir: impl Into<String>) -> Self {
        self.cache_dir = dir.into();
        self
    }

    /// Set model name.
    pub fn with_model(mut self, model: impl Into<String>) -> Self {
        self.model = model.into();
        self
    }
}

/// Refines and caches prompts using an LLM.
#[derive(Debug, Clone)]
pub struct Prompter {
    pub original: String,
    pub refined: String,
}

impl Prompter {
    /// Create a prompter. Loads from cache if available, otherwise refines via LLM.
    pub async fn new(prompt: impl Into<String>, key: &str, config: &Config) -> Result<Self> {
        let original = prompt.into();
        let path = format!("{}/{}", config.cache_dir, key);

        let refined = match fs::read_to_string(&path) {
            Ok(cached) => cached,
            Err(_) => {
                let refined = refine(&original, config).await?;
                save(&path, &refined)?;
                refined
            }
        };

        Ok(Self { original, refined })
    }

    /// Get the refined prompt.
    pub fn get(&self) -> &str {
        &self.refined
    }
}

async fn refine(prompt: &str, config: &Config) -> Result<String> {
    let client =
        gemini::Client::new(&config.api_key).map_err(|e| Error::MissingEnvVar(e.to_string()))?;

    let agent = client
        .agent(&config.model)
        .preamble("Refine this prompt. Return only the improved version, nothing else.")
        .temperature(0.5)
        .build();

    Ok(agent.prompt(prompt).await?)
}

fn save(path: &str, content: &str) -> Result<()> {
    if let Some(parent) = Path::new(path).parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, content)?;
    Ok(())
}
