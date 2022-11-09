This is an example of building & testing an out-of-tree rust module, based on info from:

- https://github.com/Rust-for-Linux/rust-out-of-tree-module
- https://www.jackos.io/rust-kernel/rust-for-linux.html

Docs for the Rust for Linux `kernel` module can be found [here](https://rust-for-linux.github.io/docs/kernel/index.html).

# Setting up Dev environment

Setup Rust-compatible linux clone:

```sh
git clone https://github.com/Rust-for-Linux/linux ./rust_for_linux
# OR
# git clone https://github.com/jackos/linux ./rust_for_linux
```

## Install Rust & set nightly toolchain (for rust building now)

Install rust via [rustup](https://rustup.rs/) if you don't have it already

```sh
rustup component add --toolchain=nightly rust-src rustfmt
rustup override set nightly
```

## Build dependencies
Check out the dependencies here: https://www.jackos.io/rust-kernel/rust-for-linux.html#dependencies

Fedora
```
sudo dnf install qemu-system-x86 lld llvm-dev clang-libs
```

# Building the module

```sh
make KDIR=../rust_for_linux LLVM=1
```

# Installing the module in Qemu vm

Build & run qemu VM:
```
make KDIR=../rust_for_linux LLVM=1 rustvm
```


```sh
# Mounting a local directory:
QEMU_EXTRAS='-virtfs local,path=./,mount_tag=rustmod,security_model=none,id=rustmod' make rustvm
# Make sure .config in KDIR has these options: https://wiki.qemu.org/Documentation/9psetup
# mount:
mkdir -p /tmp/rustmod && mount -t 9p -o trans=virtio rustmod /tmp/rustmod
# install:
insmod /tmp/rustmod/rust_vdev.ko devices=2
lsmod | grep rust_vdev

# test
echo "this works" > /dev/vdev0
cat /dev/vdev0

# remove:
rmmod rust_vdev

# turn off VM
poweroff
```

