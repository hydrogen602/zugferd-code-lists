PY_CODE := $(shell find py_generate -name "*.py")
SPEC := $(shell find spec -name "*.xlsx")

DEPS := $(PY_CODE) Makefile $(SPEC)

gen: build-marker

build-marker: $(DEPS)
	+$(MAKE) gen-rs gen-ts
	touch build-marker

npm-publish: gen
	cd ts && pnpm publish

cargo-publish: gen
	cargo publish --features all

check-rs:
	cargo check
	cargo check --features iso_country
	cargo check --features iso_currency
	cargo check --features all

gen-only:
	python -m py_generate all

gen-rs: gen-only
	cargo fmt
	$(MAKE) check-rs
	cargo build --features all

gen-ts: gen-only
	cd ts && pnpm exec prettier . --write
	cd ts && pnpm run build
	cp readme.md ts/README.md
	cp LICENSE ts/LICENSE

# ZUGFeRD has spaces in the file names which doesn't work well with Make
remove-spaces:
	uv run python remove_spaces.py
