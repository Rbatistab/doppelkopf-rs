.SILENT:
.PHONY: all build install unistall clean

# Config variables
-include config.mk

# Defaults (linux/darwin)
PREFIX ?= /usr/local
BIN ?= $(PREFIX)/bin
BINARY_NAME = dppkf

# 1. Set env vars (configure)
# 2. Install (make)

all: build

build: check
	chmod +x ./.installing_utils/build.sh
	./.installing_utils/build.sh || { \
		echo "Error while building cargo."; \
		exit 2; \
	}

check:
	chmod +x ./.installing_utils/check_dependencies.sh
	./.installing_utils/check_dependencies.sh || { \
  		echo "Dependencies are missing."; \
		exit 1; \
	}
	chmod +x ./.installing_utils/check_os_support.sh
	./.installing_utils/check_os_support.sh || { \
  		echo "Error while searching for OS compatibility with installer."; \
		exit 1; \
	}


install: build
	echo "Running Doppelkopf installer..."
#	chmod +x ./install.sh
#	install -d $(BIN)
#	./install.sh

uninstall:
	echo "Uninstalling"

clean: check
	echo "Cleaning"
	cargo clean
