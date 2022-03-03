target=m5

prj:=$(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))

all: build

bin/$(target): src/main.rs
	mkdir -p bin
	cargo build --release
	mv target/release/m5 bin/$(target)

.PHONY: test
test:
	cat src/test.m5 | m5

.PHONY: build
build:
	mkdir -p bin
	cargo build --release
	mv target/release/m5 bin/$(target)

.PHONY: clean
clean:
	cargo clean
	rm -rf bin

.PHONY: global_link
global_link:
	test $(base)
	ln -sf $(prj)/bin/$(target) $(base)/bin/$(target)
	ln -sf $(prj)/bin/$(target) $(base)/bin/m5
	ln -sf $(prj)/lib/m5 $(base)/lib/m5

.PHONY: global_unlink
global_unlink:
	echo "unimplemented"