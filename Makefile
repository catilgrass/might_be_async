.PHONY: check test-crate test-sync test-async expand fmt-expand

check: expand fmt-expand test-crate test-sync test-async

test-crate:
	cargo test

test-sync:
	cargo test -p might_be_async_test_sync --manifest-path test/Cargo.toml

test-async:
	cargo test -p might_be_async_test_async --features async --manifest-path test/Cargo.toml

expand:
	python3 scripts/expand_codes.py

fmt-expand:
	rustfmt doc/usage/* --edition 2024 --config blank_lines_lower_bound=0
