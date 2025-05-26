use std::{io, time::Instant};

use futures_util::StreamExt;
use humantime::format_duration;
use miette::{Context, IntoDiagnostic, bail};

use crate::ui::display_chunk;

#[derive(Debug, serde::Serialize)]
pub struct LlamaCppRequest<'a> {
    pub prompt: &'a str,
    pub n_predict: u32,

    /// Set to true to start receiving tokens in real time, rather than waiting until completion to
    /// send everything at once
    pub stream: bool,
}

#[derive(Debug, serde::Deserialize)]
pub struct LlamaCppResponse {
    pub content: String,

    #[expect(dead_code)]
    pub model: String,

    #[expect(dead_code)]
    pub tokens_predicted: u32,

    #[expect(dead_code)]
    pub tokens_evaluated: u32,

    pub prompt: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct LlamaCppStreamChunk {
    #[expect(dead_code)]
    pub index: u32,

    pub content: String,
    pub model: Option<String>,

    #[expect(dead_code)]
    pub tokens: Vec<u32>,
    pub stop: bool,

    #[expect(dead_code)]
    pub id_slot: i32,

    pub tokens_predicted: u32,
    pub tokens_evaluated: u32,
    pub prompt: Option<String>,
}

pub async fn llama_cpp(prompt: &str, client: &reqwest::Client) -> miette::Result<LlamaCppResponse> {
    let request_data = LlamaCppRequest {
        prompt,
        n_predict: 512,
        stream: false,
    };

    let start = Instant::now();
    let response = client
        .post("http://localhost:8080/completion")
        .json(&request_data)
        .send()
        .await
        .into_diagnostic()?;

    let duration = start.elapsed();
    log::info!("Model took {} to run", format_duration(duration));
    if !response.status().is_success() {
        bail!("Error: {}", response.text().await.into_diagnostic()?);
    }

    let response = response
        .json::<LlamaCppResponse>()
        .await
        .into_diagnostic()?;

    Ok(response)
}

pub async fn llama_cpp_stream(
    prompt: &str,
    client: &reqwest::Client,
) -> miette::Result<LlamaCppResponse> {
    let request_data = LlamaCppRequest {
        prompt,
        n_predict: 512,
        stream: true,
    };

    let response = client
        .post("http://localhost:8080/completion")
        .json(&request_data)
        .send()
        .await
        .into_diagnostic()?;

    if !response.status().is_success() {
        bail!("Error: {}", response.text().await.into_diagnostic()?);
    }

    let mut stream = response.bytes_stream();

    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout.lock());
    let mut prompt = Option::<String>::None;
    let mut model = Option::<String>::None;
    let mut complete_content = String::new();
    let mut tokens_predicted = 0;
    let mut tokens_evaluated = 0;
    while let Some(item) = stream.next().await {
        let chunk_bytes = item.into_diagnostic().wrap_err("Serialising chunk")?;
        let chunk_utf8 = str::from_utf8(&chunk_bytes)
            .into_diagnostic()
            .wrap_err("Converting chunk to UTF-8 string slice")?
            .trim();
        log::trace!("Chunk JSON: {chunk_utf8}");
        if let Some(value) = chunk_utf8.strip_prefix("data:") {
            let json_str = value.trim_start();
            let LlamaCppStreamChunk {
                content,
                stop,
                tokens_predicted: chunk_tokens_predicted,
                tokens_evaluated: chunk_tokens_evaluated,
                model: chunk_model,
                prompt: chunk_prompt,
                ..
            } = serde_json::from_str(json_str)
                .into_diagnostic()
                .wrap_err("Deserialising JSON")?;
            display_chunk(&mut handle, &content)?;
            complete_content.push_str(&content);
            if stop {
                prompt = chunk_prompt;
                model = chunk_model;
                tokens_predicted = chunk_tokens_predicted;
                tokens_evaluated = chunk_tokens_evaluated;
            }
        }
    }
    display_chunk(&mut handle, "\n")?;
    complete_content.push('\n');

    let response = LlamaCppResponse {
        content: complete_content,
        model: model.unwrap_or("Unknown model".to_string()),
        tokens_predicted,
        tokens_evaluated,
        prompt: prompt.unwrap_or("Unknown prompt".to_string()),
    };

    Ok(response)
}
