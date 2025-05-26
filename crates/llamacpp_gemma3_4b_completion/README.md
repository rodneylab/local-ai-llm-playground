<img src="../../images/llamacpp-gemma3-4b-completion.png" alt="Rodney Lab Local A I L L M Playground llama.cpp Gemma 3 4B Completion Git Hub banner" />

<p align="center">
  <a aria-label="Open Rodney Lab site" href="https://rodneylab.com" rel="nofollow noopener noreferrer">
    <img alt="Rodney Lab logo" src="https://rodneylab.com/assets/icon.png" width="60" />
  </a>
</p>
<h1 align="center">
llamacpp-gemma3-4b-completion
</h1>

**Large Language Model chat demo calling local llama.cpp server from Rust code.**

## 📝 Key details

<dl>
  <dt>Server</dt>
  <dd>llama.cpp</dd>

<dt>Model</dt>
  <dd><a href="https://huggingface.co/google/gemma-3-4b-it-qat-q4_0-gguf">Gemma3 4B parameters, instruction-trained (gemma3-4b-it)</a></dd>

<dt>Model download size</dt>
  <dd>3.16GB</dd>
</dl>

## 🖥️ Running the example:

<img src="../../images/llamacpp_gemma3_4b_completion.gif" alt="Terminal animation shows the user entering the following command: 'cargo run --bin gemma-3-4b-it-qat-q4_0-gguf'. The app starts running and a prompt appears. At the prompt, the user types 'Building a website can be done in 10 simple steps:', then hits enter. The app responds with a stream of output ending with a copy of the input prompt, then the text 'Sending prompt to llama.cpp and awaiting response...'. After a short delay, of a few seconds, the model response starts streaming, it reads ' 1. Choose a domain name and web hosting provider. 2. Select a website builder or CMS. 3. Design your website layout. 4. Create your content. 5. Add images and videos. 6. Optimize your website for search engines. 7. Test your website on different devices. 8. Launch your website. 9. Promote your website. 10. Maintain and update your website regularly. Do you want me to elaborate on any of these steps, or perhaps provide links to resources for each?'. The app presents a new prompt, ready for a new question, though the animation restarts."/>

To set up llama.cpp to run locally using Homebrew on macOS run:

```shell
brew install llama.cpp
```

Then, download and serve the model:

```shell
llama-server -hf google/gemma-3-4b-it-qat-q4_0-gguf
```

By default, `llama-server` will listen on port `8080`; the port the example app is
configured to use.

For other operating systems, or more details, see the
[LLaMA.cpp HTTP Server Quick Start Guide](https://github.com/ggml-org/llama.cpp/tree/master/tools/server#quick-start).

Clone this repo and from the repository root folder run:

```shell
cargo run --bin llamacpp_gemma3_4b_completion
```

Code makes use of the server’s REST API.

## 🧐 What’s inside?

Main Rust file is at [./src/main.rs](./src/main.rs).

## Inspiration

- [GitHub ggml-org/llama.cpp - Tools - Server - Testing with CURL](https://github.com/ggml-org/llama.cpp/tree/master/tools/server#testing-with-curl)

## ☎️ Issues

Feel free to jump into the
[Rodney Lab matrix chat room](https://matrix.to/#/%23rodney:matrix.org).
