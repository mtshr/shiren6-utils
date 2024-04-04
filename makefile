build:
	wasm-pack build --target web
	rollup ./main.js --format iife --file ./pkg/bundle.js
	wc ./pkg/shiren6_utils_bg.wasm

run:
	python3 -m http.server 8000

watch:
	cargo watch --postpone -- make build
