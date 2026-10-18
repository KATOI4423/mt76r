# Phase 1 — PCIe probe

## ゴール

まだ MMIO も DMA も firmware も触らず、

```text
MT7922
  ↓
PCI core
  ↓
Rust driver probe()
  ↓
device detected
```

だけを完成させる。

---

# 1.1 C 実装を読む

最初に読む:

```text
drivers/net/wireless/mediatek/mt76/mt7921/pci.c
```

特に:

```c
static const struct pci_device_id mt7921_pci_device_table[]
```

と

```c
static int mt7921_pci_probe(...)
```

を見る。

現在の実装では MT7922 に対応する PCI ID が device table に入っています。

---

# 1.2 まず device table を理解

C:

```c
{
    PCI_DEVICE(PCI_VENDOR_ID_MEDIATEK, 0x7922),
    .driver_data = ...
}
```

ここで、

```text
vendor ID
device ID
driver-specific data
```

が結びつく。

Rust 化する前に、自分のカードについて:

```bash
lspci -nn
```

の結果をノートに保存する。

---

# 1.3 probe の責務を分解

最初の probe は次だけ。

```text
probe()
  |
  +-- PCI device を認識
  |
  +-- vendor/device を確認
  |
  +-- device 名を log
  |
  +-- return success
```

まだ、

```text
pci_enable_device
pci_set_master
BAR mapping
DMA
IRQ
firmware
```

を入れない。

---

# 1.4 Rust driver の骨格

kernel tree の Rust PCI driver sample / Rust driver abstraction を確認して、手元の tree に合わせる。

概念:

```rust
struct Mt7922;

impl kernel::pci::Driver for Mt7922 {
    type IdInfo = ...;

    fn probe(...) -> Result<...> {
        pr_info!("MT7922 detected\n");
        ...
    }
}
```

**ここは kernel tree の API に合わせて書き換える。**

古いブログのコードをそのまま貼らないこと。

---

# 1.5 remove

probe に成功したら remove も実装する。

```text
probe success
    ↓
module unload
    ↓
remove
    ↓
resource release
```

Phase 1 では明示的な resource が少ないので、remove の存在と lifecycle を学ぶことが目的。

---

# 1.6 テスト

既存 driver を止める必要がある。

probe の log:

```bash
dmesg -w
```

または:

```bash
journalctl -kf
```

成功時:

```text
mt7922: probe
mt7922: MediaTek device detected
```

程度を出す。

---

# 1.7 デバッグ課題

### 課題 A

device ID を意図的に間違える。

期待:

```text
probe is never called
```

### 課題 B

ログを増やして、

```text
probe entered
probe succeeded
remove entered
```

を確認。

### 課題 C

既存 mt7921e を bind した状態で自作 driver が probe されない理由を説明。

---

# 完了条件

以下を自分で説明できること。

- PCI ID table
- PCI driver registration
- probe
- remove
- driver binding
- module lifecycle
- `Result` と kernel errno の対応

---

# 次へ進む前に

まだ wireless device として認識されなくてよい。

この段階で:

```text
lspci
  ↓
自作 Rust PCI driver
  ↓
probe log
```

が成立すれば成功。
