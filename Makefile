# SPDX-License-Identifier: GPL-2.0

KDIR ?= /lib/modules/$(shell uname -r)/build
OUTPUT_DIR ?= $(CURDIR)/target

# The major version number of rustc used to build the host environment's kernel
RUSTC_VER := $(shell sed -n 's/^CONFIG_RUSTC_VERSION_TEXT=rustc \([0-9]*\.[0-9]*\).*/\1/p' $(KDIR)/include/config/auto.conf)
RUSTC ?= /usr/bin/rustc-$(RUSTC_VER)

default: check
	$(MAKE) -C $(KDIR) M=$(CURDIR) MO=$(OUTPUT_DIR) RUSTC=$(RUSTC)

check:
	@test -e $(KDIR)/rust/libcore.rmeta || { \
	  echo "error: No pre-build Rust in $(KDIR)/rust."; \
	  echo "  sudo apt install linux-lib-rust-$$(uname -r)"; exit 1; }
	@test -x $(RUSTC) || { \
	  echo "error: $(RUSTC) not found."; \
	  echo "  sudo apt install rustc-$(RUSTC_VER)"; exit 1; }

clean:
	$(MAKE) -C $(KDIR) M=$(CURDIR) MO=$(OUTPUT_DIR) clean

.PHONY: default check clean
