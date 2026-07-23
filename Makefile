.PHONY: check clippy check-lock test-crate test-sync test-async expand fmt-expand lock doc

# make check
check: clippy expand fmt-expand test-crate test-sync test-async check-lock

# make doc
doc:
	cargo doc --no-deps --open

# make lock
lock:
	for f in doc/usage/*_expand.rs; do \
		cp "$$f" "$$f.lock"; \
	done

# DO NOT RUN THEM

clippy:
	cargo clippy -- -D warnings

test-crate:
	cargo test

test-sync:
	cargo test -p might_be_async_test_sync --manifest-path test/Cargo.toml

test-async:
	cargo test -p might_be_async_test_async --features metadata_async --manifest-path test/Cargo.toml

expand:
	python3 scripts/expand_codes.py

fmt-expand:
	for f in doc/usage/*_expand.rs; do \
		case "$$f" in *.lock) ;; *) rustfmt "$$f" --edition 2024 --config blank_lines_lower_bound=0 ;; esac; \
	done

check-lock:
	errors=0; \
	for lock in doc/usage/*_expand.rs.lock; do \
		[ -f "$$lock" ] || continue; \
		base="$${lock%.lock}"; \
		if [ ! -f "$$base" ]; then \
			echo "ERROR: $$lock has no matching source file"; \
			errors=1; \
		elif git --no-pager diff --no-index --quiet "$$lock" "$$base" 2>/dev/null; then \
			:; \
		else \
			git --no-pager diff --no-index "$$lock" "$$base" || true; \
			echo "ERROR: $$base differs from $$lock"; \
			errors=1; \
		fi; \
	done; \
	exit $$errors
