use rig::client::completion::CompletionClientDyn;
use rig::completion::Prompt;
use rig::completion::PromptError;
use rig::providers::gemini;
use std::fs::{self, OpenOptions};
use std::io;
use std::io::{ErrorKind, Write};
use std::path::Path;
use tokio::io::AsyncWriteExt;
#[tracing::instrument(ret)]
#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let instract = "you are a rust instructor";
    let augmented_prompt = Prompter::new(instract.to_string(), "Rust_signatur".to_string()).await.unwrap();
    println!("inital prompt: {:}", instract);
    println!("augmented propmt: {:?}", augmented_prompt.get().await);
}

#[derive(Debug)]
pub struct Prompter {
    inial_prompt: String,
    augmented_prompt: String,
    signature: String,
}
impl Prompter {
    pub async fn new(inial_prompt: String, signature: String) -> Result<Self, PromptError> {
        let path = format!("./data/prompts/{}", signature);
        match read_if_exist(path.as_str()) {
            Ok(content) => {
                Ok(Prompter {
                    inial_prompt,
                    augmented_prompt: content,
                    signature,
                })
            }
            Err(e) => {
                let rebutted_prompt = refactor_prompt(inial_prompt.clone()).await;
                let path = Path::new(path.as_str());
                if let Some(parent) = path.parent() {
                    fs::create_dir_all(parent).unwrap();
                }
                let mut file = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(path).unwrap();
                let content = rebutted_prompt.unwrap().clone();
                file.write_all(content.as_bytes()).unwrap();
                Ok(
                    Prompter {
                        inial_prompt,
                        augmented_prompt: content,
                        signature: signature,
                    })
            }
        }
    }

    pub async fn get(&self) -> Result<String, PromptError> {
        Ok(self.augmented_prompt.clone())
    }
}

pub async fn refactor_prompt(inial_prompt: String) -> Result<String, PromptError> {
    let model_name = "gemini-3-pro-preview".to_string();
    let apikey = std::env::var("GEMINI_API_KEY").expect("Gemini API key not set");
    let client = gemini::Client::new(apikey).expect("Failed to create Gemini client");
    let agent = (&client).agent(&model_name)
        .preamble("You are prompt engeener your job is refactor an prompte to repost it. return only pure prompte with any addtional text befor an after and only one option.")
        .temperature(0.5)
        .build();
    let repons = agent
        .prompt(format!(
            "refacor to repost this prompet: {:?}",
            inial_prompt
        ))
        .await;
    repons
}

fn ensure_file_exists(path: &str) -> io::Result<()> {
    let path = Path::new(path);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    todo!();
}

fn read_if_exist(path: &str) -> io::Result<String> {
    let path = Path::new(path);
    if path.exists() {
        fs::read_to_string(path)
    } else {
        Err(io::Error::new(
            ErrorKind::NotFound,
            format!("File not font: {}", path.display()),
        ))
    }
}
