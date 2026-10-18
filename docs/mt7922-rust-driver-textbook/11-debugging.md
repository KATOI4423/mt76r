# デバッグ手帳

## 1. PCI

```bash
lspci -nnk
lspci -vv -s <BDF>
cat /sys/bus/pci/devices/<BDF>/resource
```

見る:

```text
BAR
MSI/MSI-X
Bus Master
IOMMU
Link speed
Link width
ASPM
```

---

## 2. kernel log

```bash
dmesg -w
journalctl -kf
```

Rust driver では probe の入口/出口を必ずログする。

---

## 3. dynamic debug

mt76 側の debug を必要に応じて確認する。

```bash
cat /sys/kernel/debug/dynamic_debug/control
```

---

## 4. debugfs

```bash
mount | grep debugfs
ls /sys/kernel/debug
```

driver が debugfs entry を持つ場合、mt76 の既存情報と比較する。

---

## 5. wireless

```bash
iw phy
iw dev
iw dev wlanX info
iw dev wlanX link
iw dev wlanX scan
```

---

## 6. network

```bash
ip link
ip addr
ip route
```

---

# crash 時

最初に:

```text
Oops
BUG
WARNING
Call Trace
```

を見る。

Rust の panic と C kernel crash を混同しない。

---

# DMA debugging

kernel の DMA API debugging が使える環境では有効化する。

また、

```text
IOMMU
DMA address
descriptor
buffer
ownership
```

をログに出す。

ただし packet path で大量ログを出すと timing が変わる。

---

# lock debugging

以下を意識する。

```text
spinlock
mutex
completion
workqueue
NAPI
IRQ
```

特に:

```text
IRQ context で sleep していないか
atomic context で allocation していないか
lock order が循環していないか
remove と worker が race していないか
```

---

# 典型的な事故

## use-after-free

```text
remove()
  ↓
free device
  ↓
worker executes
  ↓
device access
```

## DMA after free

```text
free buffer
  ↓
hardware still owns DMA
```

## double completion

```text
timeout
  ↓
free request
  ↓
late event
  ↓
same request free again
```

## interrupt after remove

```text
free IRQ state
  ↓
IRQ
```

---

# デバッグ原則

「動かない」ではなく、

```text
どの state まで進んだか
```

をログする。

例えば:

```text
PCI_PROBED
BAR_READY
DMA_READY
FW_LOADED
MCU_READY
MAC_REGISTERED
RX_READY
TX_READY
```

という state marker を持つとよい。
