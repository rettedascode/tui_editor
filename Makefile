# Makefile for tui_editor

.PHONY: all build run format lint test install clean

all: build

build:
	cargo build --release

run:
	cargo run --release

format:
	cargo fmt --all

lint:
	cargo clippy --all-targets --all-features -- -D warnings

test:
	cargo test

install:
	@if [ "$$OS" = "Windows_NT" ]; then \
		powershell -ExecutionPolicy Bypass -File ./install.ps1; \
	elif command -v pwsh >/dev/null 2>&1; then \
		pwsh -File ./install.ps1; \
	else \
		bash ./install.sh; \
	fi

clean:
	cargo clean 