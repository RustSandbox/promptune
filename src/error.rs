use thiserror::Error;

/// Library error type.
#[derive(Debug, Error)]
pub enum Error {
    #[error("IO: {0}")]
    Io(#[from] std::io::Error),

    #[error("LLM: {0}")]
    Llm(#[from] rig::completion::PromptError),

    #[error("Missing env var: {0}")]
    MissingEnvVar(String),
}

/// Library result type.
pub type Result<T> = std::result::Result<T, Error>;
