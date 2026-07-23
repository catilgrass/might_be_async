.PHONY: test test-sync test-async expand

test: expand test-sync test-async

test-sync:
	cargo test -p might_be_async_test_sync --manifest-path test/Cargo.toml

test-async:
	cargo test -p might_be_async_test_async --features async --manifest-path test/Cargo.toml

expand:
	python3 scripts/expand_codes.py
