# verify {{{
verify\:check:  ## Check rust syntax [alias: check]
	@cargo check --all -v
.PHONY: verify\:check

check: | verify\:check
.PHONY: check

verify\:format:  ## Check format without changes [alias: verify:fmt, format, fmt]
	@cargo fmt --all -- --check
.PHONY: verify\:format

verify\:fmt: | verify\:format
.PHONY: verify\:fmt

format: | verify\:format
.PHONY: format

fmt: | verify\:format
.PHONY: fmt

verify\:lint:  ## Check style using clippy [alias: lint]
	@cargo clippy --all-targets
.PHONY: verify\:lint

lint: | verify\:lint
.PHONY: lint

verify\:all: | verify\:check verify\:format verify\:lint  ## Check code using all verify:xxx targets [alias: verify]
.PHONY: verify\:all

verify: | verify\:all
.PHONY: verify
# }}}

# test {{{
test\:unit:  ## Run unit tests
	@cargo test --lib overlap
.PHONY: test\:unit

test\:integration:  ## Run integration tests
	@cargo test --test integration_test
.PHONY: test\:integration

test\:all:  ## Run unit tests and integration tests [alias: test]
	@cargo test --tests
.PHONY: test\:all

test: | test\:all
.PHONY: test
# }}}

# coverage -- {{{
coverage:  ## Generate coverage report of unit tests only for lib using kcov [alias: cov]
	@cargo test --lib overlap --no-run
	@./.tool/setup-kcov
	./.tool/get-covered overlap
.PHONY: coverage

cov: | coverage
.PHONY: cov
# }}}

# document -- {{{
document:  ## Generate documentation files [alias: doc]
	@cargo rustdoc -- -Z --display-warnings
.PHONY: document

doc: | document
.PHONY: doc
# }}}

# build -- {{{
build\:debug:  ## Run debug build [alias: build]
	cargo build
.PHONY: build\:debug

build: | build\:debug
.PHONY: build

build\:release:  ## Create release build
	cargo build --release
.PHONY: build\:release
# }}}

# other utilities -- {{{
clean:  ## Tidy up
	@cargo clean
.PHONY: clean

package:  ## Create package
	@cargo package
.PHONY: package

install:  ## Install overlap command into the directory same with cargo
	@cargo install --path . --force
.PHONY: install

help:  ## Display this message
	@grep -E '^[0-9a-z\:\\]+: ' $(MAKEFILE_LIST) | \
	  grep -E '  ## ' | \
	  sed -e 's/\(\s|\(\s[0-9a-z\:\\]*\)*\)  /  /' | \
	  tr -d \\\\ | \
	  awk 'BEGIN {FS = ":  ## "};  \
	       {printf "\033[38;05;222m%-17s\033[0m %s\n", $$1, $$2}' | \
	  sort
.PHONY: help
# }}}

.DEFAULT_GOAL = test:all
default: verify\:check verify\:format verify\:lint test\:all
