gen: gen-rs gen-ts

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
