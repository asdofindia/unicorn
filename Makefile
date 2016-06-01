clean-build: clean build

deps:
	wget https://github.com/nanomsg/nanomsg/releases/download/0.8-beta/nanomsg-0.8-beta.tar.gz
	tar -xvzf nanomsg-0.8-beta.tar.gz
	cd nanomsg-0.8-beta && ./configure && make && sudo make install

ldconfig:
	sudo ldconfig

deps-linux: deps ldconfig

clean:
	rm -rf target
	rm -rf nanomsg-0.8-beta
	rm -f nanomsg-0.8-beta.tar.gz

build:
	cargo rustc --bin unicorn -- -C lto

build-release:
	cargo rustc --bin unicorn --release -- -C lto

nightly-build:
	rustup run nightly cargo build

test:
	cargo test

test-ignored:
	cargo test -- --ignored

doc:
	cargo doc --no-deps

.PHONY: clean deps ldconfig deps-linux build build-release clean-build test test-ignored doc
