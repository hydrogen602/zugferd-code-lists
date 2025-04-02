
gen:
	python -m py_generate all
	cargo fmt
	cd ts && npm run build
	cp readme.md ts/README.md
	cp LICENSE ts/LICENSE

npm-publish: gen
	cd ts && npm publish

cargo-publish: gen
	cargo publish
