.PHONY: all
all: dist/alfred-checkvist.alfredworkflow

.PHONY: build
build: target/release/alfred-checkvist

.PHONY: clean
clean:
	@rm -rf dist
	@rm -rf target/release

target/release/alfred-checkvist: src/main.rs
	cargo build --release
	strip $@

dist/alfred-checkvist.alfredworkflow: target/release/alfred-checkvist workflow/info.plist workflow/icon.png
	mkdir -p dist
	zip $@ -j $^
