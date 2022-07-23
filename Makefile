.PHONY: clean

test:
	mkdir test

test/%: test clean
	cd test && \
	CARGO_GENERATE_VALUE_DESCRIPTION="test for $* template" \
	cargo generate -n $* --path=../$* && \
	cd $* && \
	git init . && \
	cargo test

clean:
	rm -rf test/*
