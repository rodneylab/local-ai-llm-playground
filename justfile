default:
    just --list

# clean build and test artefacts
clean:
    rm -f ./**/local-ai-llm-playground-*.profraw 2>/dev/null
    rm -f local-ai-llm-playground-*.profraw 2>/dev/null
    rm -f crates/llamacpp_gemma3_4b_completion/local-ai-llm-playground-*.profraw 2>/dev/null
    rm -rf target 2>/dev/null

# find comments in Rust source
comments:
    rg --pcre2 -t rust '(^|\s+)(\/\/|\/\*)\s+(?!(act|arrange|assert))' .

# find expects and unwraps in Rust source
expects:
    rg --pcre2 -t rust '\.(expect\(.*\)|unwrap\(\))' .

# run python unit tests using unittest
test:
    # start Ollama server with `ollama serve` before running
    uv run -m unittest -v \
        ./python/ollama_mistral_instruct_chat/test/test_ollama_mistral_instruct_chat.py

# run coverage using grcov
coverage:
    # Python coverage
    uv run -m coverage run -m unittest -v \
        ./python/ollama_mistral_instruct_chat/test/test_ollama_mistral_instruct_chat.py
    uv run -m coverage report
    uv run -m coverage html
    uv run -m coverage lcov
    uv run -m coverage xml

    # Rust coverage
    rm -f local-ai-llm-playground-*.profraw 2>/dev/null
    cargo clean
    cargo build
    C_COMPILER=$(brew --prefix llvm)/bin/clang RUSTFLAGS="-Cinstrument-coverage" \
        LLVM_PROFILE_FILE="local-ai-llm-playground-%p-%m.profraw" cargo test
    grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing \
        -o ./target/debug/coverage/
    open --reveal ./target/debug/coverage
    sed -i '' "s|href=\"https://cdn.jsdelivr.net/npm/bulma@0.9.1/css/bulma.min.css\"|href=\"file://`pwd`/.cache/bulma.min.css\"|g" ./target/debug/coverage/**/*.html
    mkdir -p .cache
    curl --time-cond .cache/bulma.min.css -C - -Lo .cache/bulma.min.css \
      https://cdn.jsdelivr.net/npm/bulma/css/bulma.min.css

# generate docs for a crate and copy link to clipboard
doc crate:
    cargo doc -p {{ crate }}
    @echo "`pwd`/target/doc/`echo \"{{ crate }}\" | tr - _`/index.html" | pbcopy

# review (accept/reject/...) insta snapshots
insta-snapshot-review:
    cargo insta review

# check links are valid
linkcheck:
    lychee --cache --max-cache-age 1d --exclude-path "deny.toml" . "**/*.toml" "**/*.rs" "**/*.yml"

# copy URL for Rust std docs to clipboard
std:
    @rustup doc --std --path | pbcopy
