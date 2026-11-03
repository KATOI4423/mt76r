# SPDX-License-Identifier: GPL-2.0

KDIR ?= /lib/modules/$(shell uname -r)/build
KHDR := $(realpath $(KDIR))/rust/
OUTPUT_DIR ?= $(CURDIR)/target

# The major version number of rustc used to build the host environment's kernel
RUSTC_FULL_VER := $(shell sed -n 's/^CONFIG_RUSTC_VERSION_TEXT=rustc \([0-9.]*\).*/\1/p' $(KDIR)/include/config/auto.conf)
RUSTC_VER := $(shell sed -n 's/^CONFIG_RUSTC_VERSION_TEXT=rustc \([0-9]*\.[0-9]*\).*/\1/p' $(KDIR)/include/config/auto.conf)
RUSTC ?= /usr/bin/rustc-$(RUSTC_VER)
PROC_MACRO_SRC ?= /usr/lib/rust-$(RUSTC_VER)/libexec/rust-analyzer-proc-macro-srv

default: check
	$(MAKE) -C $(KDIR) M=$(CURDIR) MO=$(OUTPUT_DIR) RUSTC=$(RUSTC)

check:
	@test -e $(KDIR)/rust/libcore.rmeta || { \
	  echo "error: No pre-build Rust in $(KDIR)/rust."; \
	  echo "  sudo apt install linux-lib-rust-$$(uname -r)"; exit 1; }
	@test -x $(RUSTC) || { \
	  echo "error: $(RUSTC) not found."; \
	  echo "  sudo apt install rustc-$(RUSTC_VER)"; exit 1; }

check-ksrc:
	@test -n "$(KSRC)" || { \
		echo "error: KSRC is not set."; \
		echo "  make KSRC=/path/to/linux rust-project.json"; \
		exit 1; \
	}
	@test -f "$(KSRC)/rust/kernel/lib.rs" || { \
		echo "error: $(KSRC)/rust/kernel/lib.rs not found."; \
		exit 1; \
	}

rust-project.json: check check-ksrc
	$(MAKE) -C $(KDIR) M=$(CURDIR) RUSTC=$(RUSTC) rust-analyzer
	jq --arg hdr "$(KHDR)" --arg src "$(abspath $(KSRC))/rust/" \
		'.crates |= map(.root_module |= if startswith($$hdr) then $$src + ltrimstr($$hdr) else . end)' \
		rust-project.json > rust-project.json.tmp
	mv rust-project.json.tmp rust-project.json
	jq --arg root "$(CURDIR)/src/lib.rs" -f scripts/add_crate.jq \
		rust-project.json > rust-project.json.tmp
	mv rust-project.json.tmp rust-project.json

.PHONY: configure
configure: check rust-project.json
	@test -x $(PROC_MACRO_SRC) || { \
		echo "error: $(PROC_MACRO_SRC) not found." ; \
		echo "  sudo apt install rustc-$(RUSTC_VER)"; \
		exit 1; \
	}
	@python3 scripts/update_settings.py "$(PROC_MACRO_SRC)"

clean:
	$(MAKE) -C $(KDIR) M=$(CURDIR) MO=$(OUTPUT_DIR) clean

.PHONY: default check clean check-ksrc rust-project.json
