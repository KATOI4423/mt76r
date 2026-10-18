# Phase 3 — DMA

## ゴール

MT7922 の WFDMA を理解し、

```text
CPU
 |
 +-- descriptor ring
 |
 +-- DMA buffer
 |
 v
MT7922
```

という ownership を Rust で表現する。

---

# 3.1 DMA の概念

CPU virtual address:

```text
0xffff...
```

と device が使う DMA address:

```text
0x....
```

は同じとは限らない。

IOMMU が存在すると、

```text
device DMA address
        |
        v
      IOMMU
        |
        v
physical RAM
```

になる。

Linux DMA API はこの差を driver から隠す。

---

# 3.2 必読

```text
Documentation/core-api/dma-api-howto.rst
```

特に:

```text
dma_map_single()
dma_unmap_single()
DMA_TO_DEVICE
DMA_FROM_DEVICE
DMA_BIDIRECTIONAL
```

を理解する。

---

# 3.3 mt76 の DMA

読む:

```text
drivers/net/wireless/mediatek/mt76/dma.c
drivers/net/wireless/mediatek/mt76/mt792x_dma.c
drivers/net/wireless/mediatek/mt76/mt7921/pci.c
```

検索:

```bash
grep -R "mt76_dma" drivers/net/wireless/mediatek/mt76
grep -R "mt792x_dma" drivers/net/wireless/mediatek/mt76
```

---

# 3.4 MT7922 の ring

現行 mt7921 PCIe driver の DMA 初期化には、

```text
MCU TX queue
firmware download queue
MCU RX queue
main RX queue
TX queues
```

など複数の queue が登場する。

いきなり全部実装しない。

---

# 3.5 最初の DMA 実験

まず、

```text
1 buffer
1 descriptor
1 DMA address
```

だけを扱う。

device に実際の packet を送らなくてよい。

目的:

```text
DMA coherent / streaming
CPU ownership
device ownership
mapping lifetime
```

を理解すること。

---

# 3.6 Rust の設計

理想的には、

```rust
struct DmaBuffer<T> {
    cpu: ...,
    dma: ...,
}
```

のように、

```text
CPU側参照
DMA address
length
direction
```

をひとまとめにする。

さらに ring:

```rust
struct Ring<D> {
    descriptors: ...,
    buffers: ...,
    head: usize,
    tail: usize,
}
```

を作る。

---

# 3.7 ownership を明示する

descriptor は例えば:

```text
FREE
  ↓
CPU_PREPARED
  ↓
DEVICE_OWNED
  ↓
DEVICE_DONE
  ↓
CPU_OWNED
```

のような状態を持つ。

Rust の enum で表現すると学習効果が高い。

---

# 3.8 MT7922 では descriptor format が重要

`regs.h` だけでなく、

```text
mt76/dma.h
mt76/mt76.h
mt76/mt7921/*
```

を読む。

ビットフィールドを Rust の struct に移す場合、

```text
#[repr(C)]
```

だけでは不十分。

hardware format と endian / alignment / bitfield を確認する。

---

# 3.9 最初は device に DMA させない

Phase 3 前半:

```text
allocate
map
write descriptor
unmap
free
```

まで。

Phase 3 後半で初めて、

```text
ring register
DMA enable
DMA interrupt
```

へ進む。

---

# 完了条件

- DMA address と CPU virtual address を区別できる
- IOMMU の役割を説明できる
- DMA mapping lifetime を説明できる
- descriptor ownership を説明できる
- 1つの DMA buffer を Rust wrapper で管理できる
- MT7922 の ring が何のために複数あるか説明できる
