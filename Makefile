
NAME=lzos
ARCH=x86_64
TARGET=x86_64-lzos
MODE?=debug
RUST_TARGET=target/$(TARGET)/$(MODE)

ifeq ($(GDB),true)
	QEMU_FLAGS += -S -s
endif

ifeq ($(MODE),release)
	CARGO_FLAGS += --release
endif

.PHONY: all kernel run clean clean_iso $(NAME).iso

all: kernel

kernel: clean_iso
	cargo build $(CARGO_FLAGS)

iso: $(NAME).iso

$(RUST_TARGET)/$(NAME): kernel

$(NAME).iso: $(RUST_TARGET)/$(NAME)
	mkdir -p isodir/boot/grub
	cp $(RUST_TARGET)/$(NAME) isodir/boot/$(NAME).bin
	cp grub.cfg isodir/boot/grub/
	grub-mkrescue -o $(NAME).iso isodir
	rm -r isodir


run: $(NAME).iso
	qemu-system-$(ARCH) $(QEMU_FLAGS) -cdrom $(NAME).iso

clean: clean_iso
	cargo clean

clean_iso:
	rm -rf $(NAME).iso isodir
