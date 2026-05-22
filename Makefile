.PHONY: all build install uninstall clean

BINARY_NAME = catc
INSTALL_DIR ?= /usr/local/bin

all: build

build:
	cargo build --release

install: build
	install -d $(DESTDIR)$(INSTALL_DIR)
	install -m 755 target/release/$(BINARY_NAME) $(DESTDIR)$(INSTALL_DIR)/$(BINARY_NAME)
	@echo "Installed $(BINARY_NAME) to $(DESTDIR)$(INSTALL_DIR)/$(BINARY_NAME)"

uninstall:
	rm -f $(DESTDIR)$(INSTALL_DIR)/$(BINARY_NAME)
	@echo "Removed $(BINARY_NAME) from $(DESTDIR)$(INSTALL_DIR)/$(BINARY_NAME)"

clean:
	cargo clean
