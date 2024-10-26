build:
    cargo check
    cargo build
    cargo build --release

build-local:
    just build
    sudo cp target/release/mdsf /usr/local/bin/mdsf-local

lint:
    cargo fmt -- --check --color always
    cargo clippy --all-targets --all-features

lint-aggressive:
    cargo clean
    cargo clippy --fix --allow-staged --all-targets --all-features -- -Dclippy::style -Dclippy::double_neg -Dclippy::perf -Dclippy::pedantic -Dclippy::all -Dclippy::cargo -Dclippy::complexity -Dclippy::nursery -Dclippy::suspicious -Aclippy::module_name_repetitions -Aclippy::missing_errors_doc -Aclippy::must_use_candidate -Aclippy::multiple_crate_versions
    cargo clean

test:
    just lint
    RUST_BACKTRACE=full cargo test --release

test-coverage:
    cargo llvm-cov clean
    cargo llvm-cov --all-features --open

changelog:
    npx auto-changelog -u

codegen:
    just changelog
    cargo run --package mdsf-codegen
    cargo dist init --yes
    npx --yes prettier@latest --write --cache schemas/ README.md tools/tool.schema.json
    just format

format:
    taplo format
    cargo fmt
    just --fmt --unstable .
    npx --yes prettier@latest --write --cache .
    cargo run -- format tests
    cargo dist init --yes
    git restore tests/

precommit:
    cargo clean
    just format
    just codegen
    just build
    just lint
    just test
    just format

publish:
    just build
    just lint
    cargo clean
    cargo publish -p mdsf
