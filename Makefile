.PHONY: check clippy check-lock test-crate test-sync test-async expand fmt-expand lock

# make check
check: clippy test-crate expand fmt-expand test-sync test-async check-lock

# make lock
lock:
	for f in doc/usage/*_expand.rs; do \
		cp "$$f" "$$f.lock"; \
	done

# DO NOT RUN THEM

test-crate:
	cargo test

test-sync:
	cargo test -p might_be_async_test_sync --manifest-path test/Cargo.toml

test-async:
	cargo test -p might_be_async_test_async --features async --manifest-path test/Cargo.toml

expand:
	python3 scripts/expand_codes.py

fmt-expand:
	for f in doc/usage/*_expand.rs; do \
		case "$$f" in *.lock) ;; *) rustfmt "$$f" --edition 2024 --config blank_lines_lower_bound=0 ;; esac; \
	done

check-lock:
	for f in doc/usage/*_expand.rs; do \
	    git --no-pager diff "$$f.lock" "$$f" || (echo "ERROR: $$f differs from $$f.lock"; exit 1); \
	done
