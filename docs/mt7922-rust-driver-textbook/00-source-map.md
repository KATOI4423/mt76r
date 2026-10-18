# ソースコード対応表

## 最初に読むべき場所

### MT7921/MT7922 PCIe

```text
drivers/net/wireless/mediatek/mt76/mt7921/pci.c
drivers/net/wireless/mediatek/mt76/mt7921/pci_mac.c
drivers/net/wireless/mediatek/mt76/mt7921/pci_mcu.c
```

`pci.c` が PCIe driver の入口です。

特に探すシンボル:

```text
mt7921_pci_device_table
mt7921_pci_probe
mt7921_pci_remove
mt7921e_init_reset
mt7921_dma_init
```

---

## 共通 MT792x

```text
drivers/net/wireless/mediatek/mt76/mt792x_core.c
drivers/net/wireless/mediatek/mt76/mt792x_dma.c
drivers/net/wireless/mediatek/mt76/mt792x.h
```

MT7922 固有処理と共通処理を区別するために読む。

---

## mt76 共通層

```text
drivers/net/wireless/mediatek/mt76/dma.c
drivers/net/wireless/mediatek/mt76/mmio.c
drivers/net/wireless/mediatek/mt76/mcu.c
drivers/net/wireless/mediatek/mt76/mac80211.c
drivers/net/wireless/mediatek/mt76/tx.c
drivers/net/wireless/mediatek/mt76/agg-rx.c
drivers/net/wireless/mediatek/mt76/scan.c
```

---

## MT7921 common

```text
drivers/net/wireless/mediatek/mt76/mt7921/mac.c
drivers/net/wireless/mediatek/mt76/mt7921/mcu.c
drivers/net/wireless/mediatek/mt76/mt7921/main.c
drivers/net/wireless/mediatek/mt76/mt7921/init.c
drivers/net/wireless/mediatek/mt76/mt7921/regs.h
```

---

# Phase → source

| Phase | 第一候補 | 第二候補 |
|---|---|---|
| 1 PCI probe | `mt7921/pci.c` | `mt76/pci.c` |
| 2 BAR/MMIO | `mt7921/pci.c` | `mt76/mmio.c` |
| 3 DMA | `mt7921/pci.c` | `mt792x_dma.c`, `dma.c` |
| 4 firmware | `mt7921/mcu.c` | `mt7921/pci_mcu.c` |
| 5 MCU | `mt7921/mcu.c` | `mt7921/pci_mcu.c`, `mt76/mcu.c` |
| 6 mac80211 | `mt7921/main.c` | `mt76/mac80211.c` |
| 7 RX | `mt7921/mac.c` | `mt76/agg-rx.c`, `dma.c` |
| 8 TX | `mt7921/mac.c` | `mt76/tx.c`, `dma.c` |
| 9 scan | `mt7921/main.c` | `mt76/scan.c` |
| 10 association | `mt7921/main.c` | `mt7921/mac.c`, `mcu.c` |

---

# Linux 側で合わせて読む文書

```text
Documentation/PCI/pci.rst
Documentation/core-api/dma-api-howto.rst
Documentation/core-api/dma-api.rst
Documentation/driver-api/firmware/
Documentation/networking/mac80211-injection.rst
Documentation/driver-api/
```

mac80211 の API は source tree の

```text
include/net/mac80211.h
```

を必ず読むこと。

---

# Rust-for-Linux

重要なのは API 名を暗記することではありません。

```text
rust/kernel/
samples/rust/
drivers/
```

から、

```text
Rust abstraction
    ↓
sample driver
    ↓
自分の driver
```

という順番で読むこと。

Rust-for-Linux は API が発展中なので、手元の kernel tree の Rust API を正とします。
