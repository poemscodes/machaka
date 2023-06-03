INSTALL_PATH		:= $(HOME)/usr/bin/
MACHAKA_DEBUG_BIN	:=target/debug/machaka
MACHAKA_RELEASE_BIN	:=target/release/machaka
MACHAKA_BIN		:=$(MACHAKA_DEBUG_BIN)

all: test debug release examples

$(INSTALL_PATH):
	mkdir -p $@

$(MACHAKA_RELEASE_BIN): $(INSTALL_PATH)
	cargo build --release

$(MACHAKA_DEBUG_BIN): $(INSTALL_PATH)
	cargo build

.PHONY: all clean cls release debug fix fmt check build test examples


examples:
	cargo build --examples

release: check fix | $(MACHAKA_RELEASE_BIN)
	install $(MACHAKA_RELEASE_BIN) $(INSTALL_PATH)

debug: check fix | $(MACHAKA_DEBUG_BIN)
	install $(MACHAKA_DEBUG_BIN) $(INSTALL_PATH)

clean: cls
	@rm -rf target

cls:
	-@reset || tput reset

fix:
	cargo fix --allow-dirty --allow-staged

fmt:
	rustfmt --edition 2021 src/*.rs

check:
	cargo check --all-targets

build test: check
	cargo $@
