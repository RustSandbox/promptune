//! A tiny tool to auto-augment prompts programmatically.
//!
//! ```rust,no_run
//! use promptune::{Config, Prompter};
//!
//! #[tokio::main]
//! async fn main() -> promptune::Result<()> {
//!     let config = Config::from_env()?;
//!     let prompter = Prompter::new("your prompt", "cache_key", &config).await?;
//!     println!("{}", prompter.get());
//!     Ok(())
//! }
//! ```

mod error;
mod prompts;

pub use error::{Error, Result};
pub use prompts::{Config, Prompter};
