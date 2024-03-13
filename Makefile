build:
	cargo check
	cargo build
	cargo build --release

build-local:
	make build
	sudo cp target/release/mdsf /usr/local/bin/mdsf-local

lint:
	cargo fmt -- --check --color always
	cargo clippy --all-targets --all-features

lint-aggressive:
	cargo clean
	cargo clippy --all-targets --all-features -- -Dclippy::style -Dclippy::double_neg -Dclippy::perf -Dclippy::pedantic -Dclippy::all -Dclippy::cargo -Dclippy::complexity -Dclippy::nursery -Dclippy::suspicious -Aclippy::module_name_repetitions -Aclippy::missing_errors_doc -Aclippy::must_use_candidate -Aclippy::multiple_crate_versions
	cargo clean

test:
	make lint
	RUST_BACKTRACE=full cargo test --release

test-coverage:
	cargo llvm-cov clean
	cargo llvm-cov --all-features --open

precommit:
	cargo clean
	make build
	make lint
	make test
	cargo run -- schema
	npx --yes prettier --write .
	git restore tests/
	typos .


publish:
	make build
	make lint

	cargo clean
	cargo publish
