# promptune

A tiny tool to auto-augment prompts programmatically.

## Install

```toml
[dependencies]
promptune = "0.1.0"
tokio = { version = "1", features = ["full"] }
```

## Usage

```rust
use promptune::{Config, Prompter};

#[tokio::main]
async fn main() -> promptune::Result<()> {
    let config = Config::from_env()?;
    let prompter = Prompter::new("your prompt", "cache_key", &config).await?;

    println!("{}", prompter.get());
    Ok(())
}
```

## Configuration

Set `GEMINI_API_KEY` environment variable.

```rust
let config = Config::from_env()?
    .with_cache_dir("./cache")
    .with_model("gemini-2.0-flash");
```

## License

MIT
