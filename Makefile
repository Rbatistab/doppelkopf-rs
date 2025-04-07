.SILENT:

# Defaults for linux/darwin
PREFIX ?= /usr/local
BIN ?= $(PREFIX)/bin

# More env variables
SHELL := /bin/bash
PACKAGE_BIN_NAME = dppkf
PACKAGE_BIN_PATH = target/release/$(PACKAGE_BIN_NAME)

# Commands
.PHONY: all build install uninstall clean

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
	@source ./.installing_utils/fancy_install_text.sh && info_installing_binary
	install -d $(BIN)
	sudo install -m 755 $(PACKAGE_BIN_PATH) $(BIN)/$(PACKAGE_BIN_NAME)
	@source ./.installing_utils/fancy_install_text.sh && info_installed_files

uninstall:
	@source ./.installing_utils/fancy_uninstall_text.sh && info_uninstalling_binary
	sudo rm -rf $(BIN)/$(PACKAGE_BIN_NAME)
	@source ./.installing_utils/fancy_uninstall_text.sh && info_deleted_files

clean: check
	echo "Cleaning"
	cargo clean