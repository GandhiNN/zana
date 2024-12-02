RUSTOS:=
RUSTARCH:=

ifeq ($(OS),Windows_NT)
	OSFLAG+=pc-windows-gnu
	ifeq ($(PROCESSOR_ARCHITECTURE),AMD64)
		RUSTARCH+=x86_64
	endif
	ifeq ($(PROCESSOR_ARCHITECTURE),x86)
		RUSTARCH+=386
	endif
else
	UNAME_S := $(shell uname -s)
	ifeq ($(UNAME_S),Linux)
		RUSTOS+=linux-musl
	endif
	ifeq ($(UNAME_S),Darwin)
		RUSTOS+=darwin
	endif
		UNAME_P := $(shell uname -p)
	ifeq ($(UNAME_P),x86_64)
		RUSTARCH+=x86_64-unknown
	endif
		ifneq ($(filter %86,$(UNAME_P)),)
	RUSTARCH+=386
		endif
	ifneq ($(filter arm%,$(UNAME_P)),)
		RUSTOS+=linux-musleabihf
		RUSTARCH+=armv7-unknown
	endif
endif

OS := $(strip $(RUSTOS))
ARCH := $(strip $(RUSTARCH))

os:
	@echo $(OS) $(ARCH)

run:
	@cargo run

check:
	@cargo check
	@cargo clippy

build:	
	@echo "Building binary for ${OS} os with ${ARCH} architecture"
	@cargo build --release

all: build

