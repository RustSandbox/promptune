use thiserror::Error;

/// Library error type.
#[derive(Debug, Error)]
pub enum Error {
    #[error("IO: {0}")]
    Io(#[from] std::io::Error),

    #[error("LLM: {0}")]
    Llm(#[from] Box<rig::completion::PromptError>),

    #[error("Missing env var: {0}")]
    MissingEnvVar(String),
}

impl From<rig::completion::PromptError> for Error {
    fn from(e: rig::completion::PromptError) -> Self {
        Error::Llm(Box::new(e))
    }
}

/// Library result type.
pub type Result<T> = std::result::Result<T, Error>;
