<img src="./images/rodneylab-github-local-ai-llm-playground.png" alt="Rodney Lab Local A I L L M Playground Git Hub banner" />

<p align="center">
  <a aria-label="Open Rodney Lab site" href="https://rodneylab.com" rel="nofollow noopener noreferrer">
    <img alt="Rodney Lab logo" src="https://rodneylab.com/assets/icon.png" width="60" />
  </a>
</p>
<h1 align="center">
  local-ai-llm-playground
</h1>

**Experiments running offline LLMs in Python and Rust locally using Ollama and
llama.cpp**

## Introduction

Collection of local AI experiments that should run on a recent home computer.
Makes use of local Ollama and llama.cpp servers for running completion and chat
tasks with Gemma3 and Mistral models. Code is written in Python and Rust and
each example has a short description detailing how you can download the model and
run the code.

**Ollama** is an open-source programming language designed for rapid prototyping, education, and research in the field
of artificial intelligence (AI). Ollama:

- has a simple API;
- does not require a Python environment; and
- has a [model library, making it easy to discover and download new models](https://ollama.com/library).

**llama.cpp** is a C++ implementation of LLaMA. It is:

- extremely memory-efficient through quantisation;
- works well on CPU-only setups; and
- is available as a library for integration with other applications.

Source: [Running Local LLMs](https://github.com/di37/running-llms-locally#advantages)

## Contents

- [Introduction](#introduction)
- [Setup](#setup)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
- [Examples](#examples)
- [Why Run Local LLMs?](#-why-run-local-llms)
- [Issues and Support](#-issues-and-support)
- [Contributions](#contributions)
- [Acknowledgements](#acknowledgements)
- [License](#license)

## Setup

### Prerequisites

Example run on Ollama or llama.cpp. Here’s a quick guide to getting those
setup on macOS with Homebrew. Follow links for more detailed instructions, and
for other operating systems. You will also need Rust or Python set up on your
system (depending on which examples you want to run).

#### Ollama

```shell
brew install ollama
```

For other operating systems, or more details, see the
[Official Ollama Quickstart Guide](https://ollama.readthedocs.io/en/quickstart/).

#### llama.cpp

```shell
brew install llama.cpp
```

For other operating systems, or more details, see the
[LLaMA.cpp HTTP Server Quick Start Guide](https://github.com/ggml-org/llama.cpp/tree/master/tools/server#quick-start).

### Installation

Nothing to install beyond the prerequisites.

## Examples

<dl>
<dt>llamacpp-gemma3-4b-completion</dt>
<dd>
  <img src="./images/llamacpp_gemma3_4b_completion.gif" alt="Terminal animation shows the user entering the following command: 'cargo run --bin gemma-3-4b-it-qat-q4_0-gguf'. The app starts running and a prompt appears. At the prompt, the user types 'Building a website can be done in 10 simple steps:', then hits enter. The app responds with a stream of output ending with a copy of the input prompt, then the text 'Sending prompt to llama.cpp and awaiting response...'. After a short delay, of a few seconds, the model response starts streaming, it reads ' 1. Choose a domain name and web hosting provider. 2. Select a website builder or CMS. 3. Design your website layout. 4. Create your content. 5. Add images and videos. 6. Optimize your website for search engines. 7. Test your website on different devices. 8. Launch your website. 9. Promote your website. 10. Maintain and update your website regularly. Do you want me to elaborate on any of these steps, or perhaps provide links to resources for each?'. The app presents a new prompt, ready for a new question, though the animation restarts."/>
    <a href="./crates/llamacpp_gemma3_4b_completion/README.md">Gemma3 LLM completion demo calling local llama.cpp server from Rust code.</a>
</dd>

<dt>ollama-mistral-instruct-chat</dt>
<dd>
  <img src="./images/ollama_mistral_instruct_chat.gif" alt="Terminal animation shows the user entering the following command: 'uv run python/ollama_mistral_instruct_chat/src/main.py'. The app starts running and a prompt appears.  At the prompt, the user types 'Who was the first Prime Minister of Great Britain?', then hits enter.  After a short delay, of a few seconds, the model response starts streaming, it reads 'The position of Prime Minister in the United Kingdom does not exactly correspond to the Prime Minister you might be thinking of.  The office of Prime Minister as we know it today did not formally exist until the 18th century.  However, if we consider someone who played a role similar to that of a modern Prime Minister, Sir Robert Walpole is often considered the first defacto Ptime Minister, serving from 1721 to 1742 under King George II. He was the dominant figure in British politics during his long tenure and held various positions, including First Lord of the Treasuary and Chancellor of the Exchequer, although these titles were not yet officially linked to the Prime Minister's role.'.  The app presents a new prompt, ready for a new question, though the animation restarts."/>
  <a href="./python/ollama_mistral_instruct_chat/README.md">Mistral LLM chat demo calling local Ollama server from Python code.</a></dd>
</dl>

## 🤔 Why run local LLMs?

- **Data sovereignty**: you have more control over your data.
- **Offline support**: great if you have an unstable connection or are temporarily
  offline.
- **Model fine-tuning**: you also have more control over the model run.

You don't need the latest GPU: with llama.cpp or Ollama, smaller models (up to around 7 billion parameters) can run
comfortably on a typical home computer.

For balance, however, running locally, you pay the one-off cost of downloading
the model you want to run, you might not be able to run the largest models,
depending on your machine’s spec. Also, a cloud service would be more scalable
if you needed to step up model usage.

## ☎️ Issues and Support

Open an issue if something does not work as expected or if you have some
improvements.

Feel free to jump into the
[Rodney Lab matrix chat room](https://matrix.to/#/%23rodney:matrix.org).

## Feature requests

New feature suggestions are always welcome and will be considered, though please keep in mind that some of them may be out of scope for what the project is trying to achieve (or is reasonably capable of). If you have an idea for a new feature and would like to share it, you can do so [here](./issues/new?template=feature_request.yml).

Feature requests are tagged with one of the following:

- [Roadmap](./labels/roadmap) - will be implemented in a future release
- [Backlog](./labels/backlog) - may be implemented in the future but needs further feedback or interest from the community
- [Icebox](./labels/icebox) - no plans to implement as it doesn't currently align with the project's goals or capabilities, may be revised at a later date

## Contributions

Contributions welcome, write a short issue with your idea, before spending to
much time on more involved additions.

### Contributing guidelines

- Before working on a new feature it's preferable to submit a feature request first and state that you'd like to implement it yourself
- Please don't submit PRs for feature requests that are either in the roadmap<sup>[1]</sup>, backlog<sup>[2]</sup> or icebox<sup>[3]</sup>
- Avoid introducing new dependencies
- Avoid making backwards-incompatible configuration changes

<details>
<summary><strong><sup>[1] [2] [3]</sup></strong></summary>

[1] The feature likely already has work put into it that may conflict with your implementation

[2] The demand, implementation or functionality for this feature is not yet clear

[3] No plans to add this feature for the time being

</details>

## Acknowledgements

Inspired by:

- [llama.cpp](https://github.com/ggml-org/llama.cpp); and
- [Ollama](https://ollama.com/);

## License

The project is licensed under BSD 3-Clause License — see the [LICENSE](./LICENSE) file for details.
