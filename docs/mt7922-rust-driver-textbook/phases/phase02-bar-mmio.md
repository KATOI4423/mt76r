# Phase 2 — BAR / MMIO

## ゴール

MT7922 の BAR を取得し、

```text
PCI BAR
  ↓
MMIO mapping
  ↓
register read
  ↓
register write
```

を安全に扱えるようにする。

---

# 2.1 BAR の基礎

PCI BAR はデバイスの MMIO / I/O resource を表す。

まず:

```bash
lspci -vv -s <BDF>
cat /sys/bus/pci/devices/<BDF>/resource
```

で確認。

---

# 2.2 C 実装

読む:

```text
drivers/net/wireless/mediatek/mt76/mt7921/pci.c
drivers/net/wireless/mediatek/mt76/mmio.c
```

probe 内の:

```text
pcim_enable_device()
pci_set_master()
pci_request_region()
pci_iomap()
```

などの流れを追う。

実際の現行コードで使われている API を確認してから実装すること。

---

# 2.3 MMIO と通常メモリを混同しない

これは重要。

```text
RAM:
    *ptr = value

MMIO:
    readl()
    writel()
```

のように意味が違う。

Rust では raw pointer を持っただけでは安全にならない。

---

# 2.4 MT792x register mapping

MT7921/7922 は単純な

```text
BAR + register_offset
```

だけではありません。

`pci.c` の

```text
__mt7921_reg_addr()
```

を読む。

固定 map:

```text
physical address
      ↓
mapped offset
```

や L1 mapping が存在することを理解する。

---

# 2.5 最初の Rust abstraction

自分で:

```rust
struct Registers<'a> {
    bar: ...,
}
```

のような wrapper を作る。

例えば概念上:

```rust
fn read32(&self, offset: usize) -> u32;
fn write32(&self, offset: usize, value: u32);
```

とする。

重要なのは、

```text
register offset
BAR offset
MMIO pointer
```

を driver 全体に直接ばら撒かないこと。

---

# 2.6 volatile

MMIO access は compiler に通常の memory access として最適化されてはいけない。

そのため kernel の MMIO abstraction を使う。

自分で `read_volatile()` を乱用する前に、Rust-for-Linux の MMIO abstraction が存在するかを手元の tree で確認する。

---

# 2.7 最初の register read

安全に読める chip ID / version register を既存 driver から選ぶ。

検索:

```bash
grep -R "MT_HW_CHIPID" drivers/net/wireless/mediatek/mt76/mt7921
grep -R "CHIPID" drivers/net/wireless/mediatek/mt76/mt792x*
```

そして、

```text
probe
  ↓
MMIO init
  ↓
read chip id
  ↓
pr_info()
```

まで実装。

---

# 2.8 write は後

最初は read-only。

理由:

```text
read wrong register
    → crash / wrong value

write wrong register
    → device state corruption
```

だから。

write は既存 driver が初期化で行っている register に限定する。

---

# 2.9 デバッグ

```bash
lspci -vv -s <BDF>
dmesg -w
```

さらに必要なら:

```bash
sudo cat /sys/kernel/debug/... 
```

ただし debugfs の path は kernel/driver version により異なる。

---

# 完了条件

- BAR の意味を説明できる
- MMIO と RAM の違いを説明できる
- register offset と BAR offset の違いを説明できる
- read32 wrapper を作れる
- device の register を1つ Rust から読める
- register access の lifetime を説明できる
