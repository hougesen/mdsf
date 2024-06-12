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

lint-aggressive:
    cargo clean
    cargo clippy --all-targets --all-features -- -Dclippy::style -Dclippy::double_neg -Dclippy::perf -Dclippy::pedantic -Dclippy::all -Dclippy::cargo -Dclippy::complexity -Dclippy::nursery -Dclippy::suspicious -Aclippy::module_name_repetitions -Aclippy::missing_errors_doc -Aclippy::must_use_candidate -Aclippy::multiple_crate_versions
    cargo clean

test:
    just lint
    RUST_BACKTRACE=full cargo test --release

test-coverage:
    cargo llvm-cov clean
    cargo llvm-cov --all-features --open

update-readme:
    cargo run -- schema
    cargo run -- init
    node scripts/update-supported-languages.mjs
    npx --yes prettier@latest --cache --write mdsf.json schemas README.md

precommit:
    cargo clean
    cargo fmt
    just build
    just lint
    just test
    just update-readme
    cargo run -- format tests && git restore tests
    npx --yes prettier@latest --write --cache .
    git restore tests/
    just --fmt --unstable .
    typos .

publish:
    just build
    just lint

    cargo clean
    cargo publish
