use std::{
    io::{self},
    time::Instant,
};

use futures_util::{Stream, TryStreamExt};
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
#[cfg_attr(test, derive(serde::Serialize))]
pub struct LlamaCppResponse {
    pub content: String,

    #[cfg_attr(not(test), expect(dead_code))]
    pub model: String,

    #[cfg_attr(not(test), expect(dead_code))]
    pub tokens_predicted: u32,

    #[cfg_attr(not(test), expect(dead_code))]
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

pub async fn llama_cpp(
    prompt: &str,
    client: &reqwest::Client,
    base_url: Option<&str>,
) -> miette::Result<LlamaCppResponse> {
    let base_url = base_url.unwrap_or("http://localhost:8080");
    let request_data = LlamaCppRequest {
        prompt,
        n_predict: 512,
        stream: false,
    };

    let start = Instant::now();
    let response = client
        .post(format!("{base_url}/completion"))
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
    base_url: Option<&str>,
) -> miette::Result<LlamaCppResponse> {
    let base_url = base_url.unwrap_or("http://localhost:8080");
    let request_data = LlamaCppRequest {
        prompt,
        n_predict: 512,
        stream: true,
    };

    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout.lock());
    let mut prompt = Option::<String>::None;
    let mut model = Option::<String>::None;
    let mut complete_content = String::new();
    let mut tokens_predicted = 0;
    let mut tokens_evaluated = 0;

    let client = eventsource_client::ClientBuilder::for_url(&format!("{base_url}/completion"))
        .into_diagnostic()
        .wrap_err("Creating eventsource client")?
        .method("POST".to_string())
        .header("content-type", "application/json")
        .into_diagnostic()
        .wrap_err("Creating eventsource client request header")?
        .body(
            serde_json::to_string(&request_data)
                .into_diagnostic()
                .wrap_err("Serialising llama.cpp request JSON body")?,
        )
        .build();
    let mut stream = handle_events(&client);

    while let Ok(Some(value)) = stream.try_next().await {
        if let Some(chunk_value) = value {
            log::trace!("Value: {chunk_value:?}");
            let LlamaCppStreamChunk {
                content,
                stop,
                tokens_predicted: chunk_tokens_predicted,
                tokens_evaluated: chunk_tokens_evaluated,
                model: chunk_model,
                prompt: chunk_prompt,
                ..
            } = chunk_value;
            display_chunk(&mut handle, &content)?;
            complete_content.push_str(&content);

            if stop {
                prompt = chunk_prompt;
                model = chunk_model;
                tokens_predicted = chunk_tokens_predicted;
                tokens_evaluated = chunk_tokens_evaluated;
                break;
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

fn handle_events(
    client: &impl eventsource_client::Client,
) -> impl Stream<Item = Result<Option<LlamaCppStreamChunk>, ()>> {
    client
        .stream()
        .map_ok(|event| match event {
            eventsource_client::SSE::Event(ev) => match serde_json::from_str(&ev.data) {
                Ok(value) => Some(value),
                Err(err) => {
                    log::error!("Unable to deserialise llama.cpp stream message: {err:?}");
                    None
                }
            },
            eventsource_client::SSE::Connected(connection_details) => {
                log::info!("Connected to llama.cpp server stream");
                log::trace!("llama.cpp server connection details: {connection_details:?}");
                None
            }
            eventsource_client::SSE::Comment(comment) => {
                log::trace!("Received llama.cpp stream comment: {comment}");
                None
            }
        })
        .map_err(|err| {
            log::error!("Error streaming llama.cpp response: {err:?}");
        })
}

#[cfg(test)]
mod tests {
    use std::{
        fs::read_to_string,
        net::{IpAddr, Ipv4Addr},
    };

    use http::HeaderValue;
    use mocktail::{Headers, MockSet};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::model::{LlamaCppResponse, llama_cpp_stream};

    use super::llama_cpp;

    #[tokio::test]
    async fn test_llama_cpp_contacts_server_and_processes_response_as_expected() {
        // arrange
        let llamacpp_server = MockServer::start().await;
        let prompt = "Please tell me a funny joke.";
        let client = reqwest::Client::new();

        let json_body = read_to_string("./src/test_fixtures/nonstreaming_response.json").unwrap();
        Mock::given(wiremock::matchers::path("/completion"))
            .and(wiremock::matchers::method("POST"))
            .respond_with(ResponseTemplate::new(200).set_body_raw(json_body, "application/json"))
            .expect(1)
            .mount(&llamacpp_server)
            .await;

        // act
        let outcome = llama_cpp(prompt, &client, Some(&llamacpp_server.uri()))
            .await
            .unwrap();

        // assert
        let LlamaCppResponse {
            ref content,
            ref prompt,
            ..
        } = outcome;
        assert_eq!(
            content,
            "\n\nOkay, here's one:\n\nWhy did the scarecrow win an award?\n\n...Because he was outstanding in his field!\n\n---\n\nDid you enjoy that one?  Would you like to hear another? :)\n"
        );
        assert_eq!(prompt, "<bos>Please tell me a funny joke.");
        insta::assert_json_snapshot!(outcome);
    }

    #[tokio::test]
    async fn test_llama_cpp_stream_contacts_server_and_processes_response_as_expected() {
        // arrange
        let prompt = "Could you tell me what HTML stands for? Please be concise.";

        let mut mocks = MockSet::new();
        let mut response_headers = Headers::new();
        response_headers.insert(
            http::header::CONTENT_TYPE,
            HeaderValue::from_str("text/event-stream").unwrap(),
        );
        let data_json_chunks = [
            r#"{"index":0,"content":"\n\n","tokens":[108],"stop":false,"id_slot":-1,"tokens_predicted":1,"tokens_evaluated":14}"#,
            r#"{"index":0,"content":"Hyper","tokens":[58935],"stop":false,"id_slot":-1,"tokens_predicted":2,"tokens_evaluated":14}"#,
            r#"{"index":0,"content":"Text","tokens":[2067],"stop":false,"id_slot":-1,"tokens_predicted":3,"tokens_evaluated":14}"#,
            r#"{"index":0,"content":" Markup","tokens":[228084],"stop":false,"id_slot":-1,"tokens_predicted":4,"tokens_evaluated":14}"#,
            r#"{"index":0,"content":" Language","tokens":[22160],"stop":false,"id_slot":-1,"tokens_predicted":5,"tokens_evaluated":14}"#,
            r#"{"index":0,"content":"\n","tokens":[107],"stop":false,"id_slot":-1,"tokens_predicted":6,"tokens_evaluated":14}"#,
            r#"{"index":0,"content":"","tokens":[106],"stop":false,"id_slot":-1,"tokens_predicted":7,"tokens_evaluated":14}"#,
            r#"{"index":0,"content":"","tokens":[],"id_slot":0,"stop":true,"model":"gpt-3.5-turbo","tokens_predicted":7,"tokens_evaluated":14,"generation_settings":{"n_predict":512,"seed":4294967295,"temperature":0.800000011920929,"dynatemp_range":0.0,"dynatemp_exponent":1.0,"top_k":40,"top_p":0.949999988079071,"min_p":0.05000000074505806,"top_n_sigma":-1.0,"xtc_probability":0.0,"xtc_threshold":0.10000000149011612,"typical_p":1.0,"repeat_last_n":64,"repeat_penalty":1.0,"presence_penalty":0.0,"frequency_penalty":0.0,"dry_multiplier":0.0,"dry_base":1.75,"dry_allowed_length":2,"dry_penalty_last_n":4096,"dry_sequence_breakers":["\n",":","\"","*"],"mirostat":0,"mirostat_tau":5.0,"mirostat_eta":0.10000000149011612,"stop":[],"max_tokens":512,"n_keep":0,"n_discard":0,"ignore_eos":false,"stream":true,"logit_bias":[],"n_probs":0,"min_keep":0,"grammar":"","grammar_lazy":false,"grammar_triggers":[],"preserved_tokens":[],"chat_format":"Content-only","reasoning_format":"deepseek","reasoning_in_content":true,"thinking_forced_open":false,"samplers":["penalties","dry","top_n_sigma","top_k","typ_p","top_p","min_p","xtc","temperature"],"speculative.n_max":16,"speculative.n_min":0,"speculative.p_min":0.75,"timings_per_token":false,"post_sampling_probs":false,"lora":[]},"prompt":"<bos>Could you tell me what HTML stands for? Please be concise.","has_new_line":true,"truncated":false,"stop_type":"eos","stopping_word":"","tokens_cached":20,"timings":{"prompt_n":13,"prompt_ms":8553.384,"prompt_per_token_ms":657.9526153846153,"prompt_per_second":1.5198662891786456,"predicted_n":7,"predicted_ms":3150.478,"predicted_per_token_ms":450.0682857142857,"predicted_per_second":2.2218850599813744}}"#,
        ];
        let chunks: Vec<String> = data_json_chunks
            .iter()
            .map(|val| format!("data: {val}\n\n"))
            .collect();
        mocks.mock(|when, then| {
            when.post().path("/completion");
            then.headers(response_headers).bytes_stream(chunks);
        });
        let llamacpp_server = mocktail::server::MockServer::new("llama.cpp")
            .with_config(mocktail::server::MockServerConfig {
                listen_addr: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
                ..Default::default()
            })
            .with_mocks(mocks);
        llamacpp_server.start().await.unwrap();

        // act
        let url = llamacpp_server.url("/");
        let url_str = url.as_str();
        let url = if url_str.ends_with('/') {
            url_str.strip_suffix('/').unwrap_or(url_str)
        } else {
            url_str
        };
        let outcome = llama_cpp_stream(prompt, Some(url)).await.unwrap();

        // assert
        let LlamaCppResponse {
            ref content,
            ref prompt,
            ..
        } = outcome;
        assert_eq!(content, "\n\nHyperText Markup Language\n\n");
        assert_eq!(
            prompt,
            "<bos>Could you tell me what HTML stands for? Please be concise."
        );
        insta::assert_json_snapshot!(outcome);
    }
}
