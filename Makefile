.PHONY: all install-pre-commit setup-hooks run-hooks format check clean help

all: build

help:
	@echo "Available targets:"
	@echo "    install-pre-commit - Install pre-commit"
	@echo "    setup-hooks - Set up pre-commit hooks"
	@echo "    run-hooks - Run pre-commit hooks"
	@echo "    build - Builds the application in release mode"
	@echo "    build-dev - Builds the application with debug symbols enabled. (release-mode off)"
	@echo "    setup - Setup dev enironment"
	@echo "    format - Format code"

# TODO: Check if Cargo and Rustup are already installed. If not, install them
# using the system's package manager.
install-rustup:
	@echo "Installing rustup..."
	@if! command -v zypper &> /dev/null; then \
		sudo zypper in rustup -y; \
	else \
		if! command -v apt-get &> /dev/null; then \
			sudo apt-get install rustup; \
		if! command -v dnf dnf &> /dev/null; then \
			sudo dnf install rustup; \
		fi; \
	fi;

# Might fail when pip does not allow system wide installation
install-pre-commit:
	@echo "Installing pre-commit..."
	pip install pre-commit || { echo 'Error installing pre-commit'; exit 1; }

setup-hooks:
	@echo "Setting up pre-commit hooks..."
	pre-commit install

run-hooks:
	@echo "Running hooks..."
	pre-commit run --all-files

setup: install-pre-commit setup-hooks run-hooks
	@echo "pre-commit setup completed."

build:
	cargo build --lib --release

build-dev:
	cargo build --lib

docs:
	cargo docs --lib --no-deps --document-private-items --open

format:
	@echo "Formatting code with 'cargo fmt'..."
	cargo fmt

check:
	@echo "Running clippy check..."
	cargo clippy

clean:
	@echo "Cleaning build artifacts..."
	cargo clean