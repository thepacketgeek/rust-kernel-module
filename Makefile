# SPDX-License-Identifier: GPL-2.0

KDIR ?= /lib/modules/`uname -r`/build

default:
	$(MAKE) -C $(KDIR) M=$$PWD

rustvm: default
	$(Q) $(MAKE) -C $(KDIR) && qemu-system-x86_64 -nographic -kernel $(KDIR)/vmlinux -initrd $(KDIR)/initrd.img -nic user,model=rtl8139,hostfwd=tcp::5555-:23 $(QEMU_EXTRAS)

clean:
	$(MAKE) -C $(KDIR) M=$$PWD clean
