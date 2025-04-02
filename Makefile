
gen:
	python -m py_generate all
	cargo fmt
	cd ts && npm run build
	cp readme.md ts/README.md
	cp LICENSE ts/LICENSE

npm-publish:
	cd ts && npm publish

cargo-publish:
	cargo publish
