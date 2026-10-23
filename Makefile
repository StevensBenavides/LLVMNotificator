SHELL := /bin/bash

PREFIX ?= $(HOME)/.local
BINDIR ?= $(PREFIX)/bin
CONFIG_HOME ?= $(or $(XDG_CONFIG_HOME),$(HOME)/.config)
DATA_HOME ?= $(or $(XDG_DATA_HOME),$(HOME)/.local/share)
STATE_HOME ?= $(or $(XDG_STATE_HOME),$(HOME)/.local/state)
CONFIG_DIR ?= $(CONFIG_HOME)/LLVMNotificator
STATE_DIR ?= $(STATE_HOME)/LLVMNotificator
DATA_DIR ?= $(DATA_HOME)/LLVMNotificator
SYSTEMD_USER_DIR ?= $(CONFIG_HOME)/systemd/user
BINARY ?= $(BINDIR)/LLVMNotificator

.PHONY: all build test lint install uninstall enable disable native size run clean

all: build

build:
	cargo build --release

test:
	cargo test

lint:
	cargo clippy --all-targets -- -D warnings

install: build
	install -Dm755 target/release/LLVMNotificator "$(BINARY)"
	install -d -m755 "$(STATE_DIR)" "$(DATA_DIR)"
	install -Dm644 systemd/LLVMNotificator.service.in "$(SYSTEMD_USER_DIR)/LLVMNotificator.service"
	sed -i -e 's|@BINARY@|$(BINARY)|g' "$(SYSTEMD_USER_DIR)/LLVMNotificator.service"
	if [ ! -f "$(CONFIG_DIR)/config.toml" ]; then install -Dm600 config/config.toml "$(CONFIG_DIR)/config.toml"; fi

uninstall:
	rm -f "$(BINARY)" "$(SYSTEMD_USER_DIR)/LLVMNotificator.service"
	systemctl --user daemon-reload

enable:
	systemctl --user daemon-reload
	systemctl --user reenable --now LLVMNotificator.service

disable:
	systemctl --user disable --now LLVMNotificator.service

run:
	cargo run --release -- --check --simulate

clean:
	cargo clean
