BSP ?= rpi3

ifeq ($(BSP),rpi3)
  TARGET            = aarch64-unknown-none-softfloat
  KERNEL_BIN        = target/bin/kernel8.img
  QEMU_BINARY       = qemu-system-aarch64
  QEMU_MACHINE_TYPE = raspi3b
  QEMU_RELEASE_ARGS = -d in_asm -display none
  OBJDUMP_BINARY    = aarch64-none-elf-objdump
  NM_BINARY         = aarch64-none-elf-nm
  READELF_BINARY    = aarch64-none-elf-readelf
  LD_SCRIPT_PATH    = $(shell pwd)/linker
  RUSTC_MISC_ARGS   = -Ctarget-cpu=cortex-a53
endif

KERNEL_MANIFEST      = Cargo.toml
KERNEL_LINKER_SCRIPT = kernel.ld

KERNEL_ELF      = target/$(TARGET)/release/kernel
# This parses cargo's dep-info file.
# https://doc.rust-lang.org/cargo/guide/build-cache.html#dep-info-files
KERNEL_ELF_DEPS = $(filter-out %: ,$(file < $(KERNEL_ELF).d)) $(KERNEL_MANIFEST)

RUSTFLAGS = $(RUSTC_MISC_ARGS) \
  -Clink-arg=--library-path=$(LD_SCRIPT_PATH) \
  -Clink-arg=--script=$(KERNEL_LINKER_SCRIPT)

FEATURES      = bsp_$(BSP)
COMPILER_ARGS = --target=$(TARGET) --features $(FEATURES) --release

RUSTC_CMD   = cargo rustc $(COMPILER_ARGS)
CLIPPY_CMD  = cargo clippy $(COMPILER_ARGS)
CHECK_CMD   = cargo check $(COMPILER_ARGS)
OBJCOPY_CMD = rust-objcopy --strip-all -O binary

EXEC_QEMU = $(QEMU_BINARY) -M $(QEMU_MACHINE_TYPE)


.PHONY: build qemu clippy clean check readelf objdump nm

build: $(KERNEL_BIN)

$(KERNEL_ELF): $(KERNEL_ELF_DEPS)
	$(call color_header, "Compiling kernel ELF - $(BSP)")
	@RUSTFLAGS="$(RUSTFLAGS)" $(RUSTC_CMD)

$(KERNEL_BIN): $(KERNEL_ELF)
	$(call color_header, "Generating stripped binary")
	@mkdir -p $(dir $(KERNEL_BIN))
	@$(OBJCOPY_CMD) $(KERNEL_ELF) $(KERNEL_BIN)
	$(call color_progress_prefix, "Name")
	@echo $(KERNEL_BIN)
	$(call color_progress_prefix, "Size")
	$(call disk_usage_KiB, $(KERNEL_BIN))

qemu: $(KERNEL_BIN)
	$(call color_header, "Launching QEMU")
	@$(EXEC_QEMU) $(QEMU_RELEASE_ARGS) -kernel $(KERNEL_BIN)

clippy:
	$(call color_header, "Running clippy")
	@$(CLIPPY_CMD)

check:
	$(call color_header, "Running check")
	@$(CHECK_CMD)

clean:
	@cargo clean

readelf: $(KERNEL_ELF)
	$(call color_header, "Launching readelf")
	@$(READELF_BINARY) --headers $(KERNEL_ELF)

objdump: $(KERNEL_ELF)
	$(call color_header, "Launching objdump")
	@$(OBJDUMP_BINARY) --disassemble --demangle --section .text $(KERNEL_ELF)

nm: $(KERNEL_ELF)
	$(call color_header, "Launching nm")
	@$(NM_BINARY) --demangle --print-size $(KERNEL_ELF) | sort
