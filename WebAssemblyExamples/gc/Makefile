all: module.wasm

.PHONEY: run

%.wasm: %.wat
	wasm-as --enable-gc --enable-reference-types -o $@ $<

run:
	v8  --experimental-wasm-gc --module ./runner.js
