# Phase 0 — 環境構築

このファイルは10 Phaseの前提となる準備です。

## 0.1 PCI デバイスを確認

```bash
lspci -nn
lspci -nnk
```

対象を絞る:

```bash
lspci -nn -s 0000:xx:yy.z
```

確認するもの:

```text
Vendor ID
Device ID
Subsystem Vendor ID
Subsystem Device ID
Kernel driver in use
Kernel modules
```

---

## 0.2 PCI resource

```bash
cat /sys/bus/pci/devices/0000:xx:yy.z/resource
```

または:

```bash
sudo lspci -vv -s 0000:xx:yy.z
```

BAR の位置とサイズを確認する。

---

## 0.3 既存ドライバ

MT7922 は通常 mt7921e/mt76 系の driver から扱われます。

```bash
modinfo mt7921e
lsmod | grep mt792
```

デバイスと driver の結びつきを確認:

```bash
readlink /sys/bus/pci/devices/0000:xx:yy.z/driver
```

---

## 0.4 最初の原則

Phase 1 の probe を試す前に、既存 `mt7921e` が対象 device を掴んでいない状態を作る必要があります。

ただし Wi-Fi を実際に使っている PC で module unload を行うとネットワークが切れます。

学習用の別ネットワーク手段:

- Ethernet
- USB Ethernet
- 別 Wi-Fi device
- SSH 用の別 NIC

を用意してください。

---

## 0.5 kernel module 開発

最初は out-of-tree module ではなく、Rust-for-Linux kernel tree に sample を追加する方式を理解します。

まず:

```text
samples/rust/
```

を読む。

目的は、

```text
module_init
module_exit
module metadata
logging
error handling
```

を覚えること。

---

## 0.6 Rust の kernel code

一般的な userspace Rust と違い、

```rust
std::vec::Vec
std::sync::Arc
std::thread
```

などをそのまま使えるとは限りません。

kernel Rust では、

```rust
kernel::prelude::*;
kernel::sync::*;
kernel::alloc::*;
```

などの kernel abstraction を使います。

---

## 完了条件

以下を自分で説明できれば開始。

- PCI device と PCI driver の違い
- probe がいつ呼ばれるか
- remove がなぜ必要か
- module と built-in の違い
- kernel virtual address と physical address の違い
- `unsafe` がなぜ必要になり得るか
