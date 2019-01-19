# vet

vet: | vet\:check vet\:format vet\:lint  ## Run all vet targets
.PHONY: vet

vet\:check:  ## Check rust syntax
	cargo check --all -v
.PHONY: vet\:check

vet\:format:  ## Check format without changes [alias: vet:fmt, fmt]
	cargo fmt --all -- --check
.PHONY: vet\:format

vet\:fmt: | vet\:format
.PHONY: vet\:fmt

fmt: | vet\:format
.PHONY: fmt

vet\:lint:  ## Check style using clippy [alias: lint]
	cargo clippy --all-targets
.PHONY: vet\:lint

lint: | vet\:lint
.PHONY: lint

vet\:all: | vet\:check vet\:format vet\:lint  ## Check code using all vet:xxx targets
.PHONY: vet\:all

# test

test\:all:  ## Run unit tests and integration tests [alias: test]
	cargo test --tests
.PHONY: test\:all

test: | test\:all
.PHONY: test

# coverage

coverage:  ## Generate coverage report of unit tests using kcov [alias: cov]
	cargo test --bin overlap --no-run
	./.tools/check-kcov overlap kcov
.PHONY: coverage

# document

document:  ## Generate documentation files [alias: doc]
	cargo rustdoc -- -Z --display-warnings
.PHONY: document

doc: | document
.PHONY: doc

# build

build\:debug:  ## Run debug build [alias: build]
	cargo build
.PHONY: build\:debug

build: | build\:debug
.PHONY: build

build\:release:  ## Create release build
	cargo build --release
.PHONY: build\:release

# other utilities

clean:  ## Tidy up
	cargo clean
.PHONY: clean

help:  ## Display this message
	@grep -E '^[0-9a-z\:\\]+: ' $(MAKEFILE_LIST) | grep -E '  ## ' | \
	  sed -e 's/\(\s|\(\s[0-9a-z\:\\]*\)*\)  /  /' | tr -d \\\\ | \
	  awk 'BEGIN {FS = ":  ## "}; {printf "\033[36m%-14s\033[0m %s\n", $$1, $$2}' | \
	  sort
.PHONY: help

.DEFAULT_GOAL = test\:all
default: test\:all
