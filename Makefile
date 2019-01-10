# vet

vet: | vet\:check vet\:format vet\:lint  ## Run all vet targets
.PHONY: vet

vet\:check:  ## Check rust syntax
	cargo check --all -v
.PHONY: vet\:check

vet\:format:  ## Check format without changes (alias: vet:fmt)
	cargo fmt --all -- --check
.PHONY: vet\:format

vet\:fmt: | vet\:format
.PHONY: vet\:fmt

vet\:lint:  ## Check code style using clippy
	cargo clippy --all-targets
.PHONY: vet\:lint

# test

test:  ## Run unit tests and integration tests
	cargo test
.PHONY: test

test\:coverage:  ## Generate coverage report of unit tests using kcov (alias: test:cov)
	cargo test --bin overlap --no-run
	./.tools/check-kcov overlap kcov
.PHONY: test\:coverage

test\:cov: | test\:coverage
.PHONY: test\:cov

# document

document:  ## Generate documentation files (alias: doc)
	cargo rustdoc -- -Z --display-warnings
.PHONY: document

doc: | document
.PHONY: doc

# build

build:  ## Run debug build
	cargo build
.PHONY: build

build\:release:  ## Create release build
	cargo build --release
.PHONY: build\:release

# other utilities

clean:  ## Clean up
	cargo clean
.PHONY: clean

help:  ## Display this message
	@grep -E '^[0-9a-z\:\\]+: ' $(MAKEFILE_LIST) | grep -E '  ## ' | \
	  sed -e 's/\(\s|\(\s[0-9a-z\:\\]*\)*\)  /  /' | tr -d \\\\ | \
	  awk 'BEGIN {FS = ":  ## "}; {printf "\033[36m%-14s\033[0m %s\n", $$1, $$2}' | \
	  sort
.PHONY: help

.DEFAULT_GOAL = help
default: help
