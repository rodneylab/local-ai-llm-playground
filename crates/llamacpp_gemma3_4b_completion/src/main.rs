#![warn(clippy::all, clippy::pedantic)]

mod cli;
mod model;
mod ui;

use std::time::Duration;

use clap::Parser;
use indicatif::ProgressBar;

use crate::{
    cli::Cli,
    model::{LlamaCppResponse, llama_cpp, llama_cpp_stream},
    ui::get_prompt,
};

#[tokio::main]
async fn main() -> miette::Result<()> {
    let cli = &Cli::parse();
    env_logger::Builder::new()
        .filter_level(cli.verbose.log_level_filter())
        .init();

    let client = reqwest::Client::new();
    let prompt = get_prompt()?;
    println!("\nPrompt: `{prompt}`");

    if cli.no_stream {
        let bar = ProgressBar::new_spinner();
        println!("\nSending prompt to llama.cpp server and awaiting response...");
        bar.enable_steady_tick(Duration::from_millis(100));
        let response = llama_cpp(&prompt, &client).await?;
        bar.finish();
        let LlamaCppResponse {
            prompt, content, ..
        } = response;
        println!("Prompt: {prompt}\n\nResponse: {content}");
    } else {
        println!("\nSending prompt to llama.cpp server and awaiting response...");
        llama_cpp_stream(&prompt, &client).await?;
    }

    Ok(())
}
