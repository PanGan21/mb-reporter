.PHONY: test clean

test:
	cargo test

clean-results:
	rm -rf results