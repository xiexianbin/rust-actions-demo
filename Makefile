# https://www.xiexianbin.cn/program/tools/2016-01-09-makefile/index.html
.PHONY: all fmt test clippy clean build

CARGOCMD=cargo

help:  ## Show this help.
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {sub("\\\\n",sprintf("\n%22c"," "), $$2);printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)

all: fmt test clippy build  ## Build all

clippy:  ## run clippy
	cargo clippy -- -D warnings

fmt:  ## run fmt
	cargo fmt --all -- --check

test:  ## run test
	cargo test

build:  ## build for current os
	cargo build
