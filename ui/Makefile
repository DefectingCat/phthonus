RUSTFLAGS := "-Zthreads=8"

CARGO = RUSTFLAGS=$(RUSTFLAGS) cargo
RUSTC = rustc
CROSS = cross

all: build

build:
	trunk build

dev:
	trunk serve

release:
	trunk build --release

run:
	$(CARGO) run -p venus

test:
	$(CARGO) test

clean:
	$(CARGO) clean

clean-release:
	rm -rf ./venus/target/release/
	rm -rf ./venus/target/debug/

check:
	$(CARGO) check

format:
	$(CARGO) +nightly fmt

lint:
	$(CARGO) +nightly clippy

fix:
	$(CARGO) +nightly fix --allow-dirty --all-features && $(CARGO) +nightly fmt

linux-musl: clean-release
	$(CROSS) build -p venus --release --target x86_64-unknown-linux-musl

linux-gnu: clean-release
	$(CROSS) build -p venus --release --target x86_64-unknown-linux-gnu

windows-gnu: clean-release
	$(CROSS) build -p venus --release --target x86_64-pc-windows-gnu

freebsd: clean-release
	$(CROSS) build -p venus --release --target x86_64-unknown-freebsd

loongarch: clean-release
	$(CROSS) build -p venus --release --target loongarch64-unknown-linux-gnu

.PHONY: all
