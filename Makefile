MODULE_ELF = "target/svsm-target/${TARGET_PATH}/svsm-module"

ifdef RELEASE
TARGET_PATH="release"
CARGO_ARGS="--release"
else
TARGET_PATH="debug"
CARGO_ARGS=
endif

all: target/${TARGET_PATH}/svsm-module

test:
	cd src/
	cargo test --target=x86_64-unknown-linux-gnu -Z build-std

target/${TARGET_PATH}/svsm-module:
	cargo build ${CARGO_ARGS} --bin svsm-module
	objcopy -O elf64-x86-64 --strip-unneeded ${MODULE_ELF} $@

clean:
	cargo clean

