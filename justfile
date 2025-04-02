build:
    cargo check
    cargo build

build-local:
    cargo build --release
    sudo cp target/release/mdsf /usr/local/bin/mdsf-local

lint:
    cargo fmt -- --check --color always
    cargo clippy --all-targets --all-features

lint-aggressive:
    cargo clean
    cargo clippy --fix --allow-staged --all-targets --all-features -- -Dclippy::style -Dclippy::double_neg -Dclippy::perf -Dclippy::pedantic -Dclippy::all -Dclippy::cargo -Dclippy::complexity -Dclippy::nursery -Dclippy::suspicious -Aclippy::module_name_repetitions -Aclippy::missing_errors_doc -Aclippy::must_use_candidate -Aclippy::multiple_crate_versions -Aclippy::needless_raw_strings -Aclippy::needless_raw_string_hashes -Dclippy::std_instead_of_core
    cargo clean

test:
    RUST_BACKTRACE=full cargo nextest run --all-targets --all-features --no-fail-fast --release

test-coverage:
    cargo llvm-cov clean
    cargo llvm-cov --all-features --all-targets --open

changelog:
    npx auto-changelog --hide-credit -u -l 100 -b 100

codegen:
    just changelog
    cargo fmt 
    cargo run --package mdsf-codegen
    cargo fmt 
    cargo run --package mdsf-codegen
    cargo fmt 
    dist init --yes
    just format

sort-json:
    npx jsonlint -s -i mdsf.json
    find ./tools -type f -name "*.json" -exec npx jsonlint -s -i {} \;

format:
    taplo format
    cargo fmt
    just --fmt --unstable .
    npx --yes prettier@latest --write --cache .
    cargo run -- format .
    dist init --yes

version VERSION:
    cd mdsf-vscode && npm version {{ VERSION }}
    cd github-action && npm version {{ VERSION }}

precommit-vscode:
    cd mdsf-vscode && npm i
    cd mdsf-vscode && npm run lint:biome:fix
    cd mdsf-vscode && npm run lint:eslint:fix

precommit-github-action:
    cd github-action && npm i

precommit-cli:
    just format
    just codegen
    just build
    just lint
    just test
    just format

precommit:
    just precommit-vscode 
    just precommit-github-action
    just precommit-cli 
