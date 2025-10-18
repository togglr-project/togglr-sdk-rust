.PHONY: help build check test clean examples generate-api docs

help: ## Show this help message
	@echo "Available targets:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2}'

build: ## Build the project
	cargo build

check: ## Check the project without building
	cargo check

test: ## Run tests
	cargo test

clean: ## Clean build artifacts
	cargo clean

examples: ## Build examples
	cargo build --examples

run-simple: ## Run simple example
	cargo run --example simple_example

run-advanced: ## Run advanced example
	cargo run --example advanced_example

run-tls: ## Run TLS example
	cargo run --example tls_example

generate-api: ## Regenerate OpenAPI client
	openapi-generator-cli generate -i specs/sdk.yml -g rust -o generated --package-name togglr_sdk_generated

docs: ## Generate documentation
	cargo doc --open

format: ## Format code
	cargo fmt

clippy: ## Run clippy linter
	cargo clippy -- -D warnings

all: check test examples ## Run all checks and build examples
