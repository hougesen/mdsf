alias b := build
alias l := lint
alias t := test
alias tc := test-coverage

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
    cd codegen && cargo clippy --all-targets --all-features

lint-aggressive:
    cargo clean
    cargo clippy --all-targets --all-features -- -Dclippy::style -Dclippy::double_neg -Dclippy::perf -Dclippy::pedantic -Dclippy::all -Dclippy::cargo -Dclippy::complexity -Dclippy::nursery -Dclippy::suspicious -Aclippy::module_name_repetitions -Aclippy::missing_errors_doc -Aclippy::must_use_candidate -Aclippy::multiple_crate_versions
    cargo clean

lint-aggressive-codegen:
    cd codegen && cargo clean
    cd codegen && cargo clippy --all-targets --all-features -- -Dclippy::style -Dclippy::double_neg -Dclippy::perf -Dclippy::pedantic -Dclippy::all -Dclippy::cargo -Dclippy::complexity -Dclippy::nursery -Dclippy::suspicious -Aclippy::module_name_repetitions -Aclippy::missing_errors_doc -Aclippy::must_use_candidate -Aclippy::multiple_crate_versions
    cd codegen && cargo clean

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
    cd codegen && cargo run

format:
    cargo fmt
    (cd codegen && cargo fmt)
    just --fmt --unstable .
    npx --yes prettier@latest --write --cache .
    cargo run -- format tests
    git restore tests/

precommit:
    cargo clean
    just format
    just codegen
    just build
    just lint
    just test
    just format
    typos --exclude src/generated.rs --exclude CHANGELOG.md .

publish:
    just build
    just lint
    cargo clean
    cargo publish
