.PHONY: default
default: test

.PHONY: build_debug
build_debug:
	cargo build

.PHONY: test
test: build_debug
	scripts/test-runner tests/repl.lish
