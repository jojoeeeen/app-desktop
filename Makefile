.DEFAULT_GOAL := help
.PHONY: help
help:  ## show this help
	@grep -E '^[a-zA-Z_\/-]+:.*?## .*$$' $(MAKEFILE_LIST) \
		| awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'


.PHONY: lint
lint: ## lint && fmt
	yarn lint && cargo fmt --manifest-path ./src-tauri/Cargo.toml


.PHONY: serve 
serve: ## serve
    yarn tauri:serve
