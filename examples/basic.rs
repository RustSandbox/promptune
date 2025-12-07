use promptune::{Config, Prompter};

#[tokio::main]
async fn main() -> promptune::Result<()> {
    let config = Config::from_env()?;

    let instruction = "you are a rust instructor";
    let prompter = Prompter::new(instruction, "Rust_signature", &config).await?;

    println!("Initial prompt: {}", instruction);
    println!("Augmented prompt: {}", prompter.get());

    Ok(())
}
