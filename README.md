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
- [Why Run Local LLMs?](#why-run-local-llms)
- [Issues and Support](#issues-and-support)
- [Contributions](#contributions)
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
<dd><a href="./crates/llamacpp_gemma3_4b_completion/README.md">Gemma3 LLM completion demo calling local llama.cpp server from Rust code.</a></dd>

<dt>ollama-mistral-instruct-chat</dt>
<dd><a href="./python/ollama_mistral_instruct_chat/README.md">Mistral LLM chat demo calling local Ollama server from Python code.</a></dd>
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

New feature suggestions are always welcome and will be considered, though please keep in mind that some of them may be out of scope for what the project is trying to achieve (or is reasonably capable of). If you have an idea for a new feature and would like to share it, you can do so [here](/issues/new?template=feature_request.yml).

Feature requests are tagged with one of the following:

- [Roadmap](/labels/roadmap) - will be implemented in a future release
- [Backlog](/labels/backlog) - may be implemented in the future but needs further feedback or interest from the community
- [Icebox](/labels/icebox) - no plans to implement as it doesn't currently align with the project's goals or capabilities, may be revised at a later date

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

## License

The project is licensed under BSD 3-Clause License — see the [LICENSE](/LICENSE) file for details.
