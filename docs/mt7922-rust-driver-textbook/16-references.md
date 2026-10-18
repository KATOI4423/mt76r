# References

## Linux kernel

- Linux kernel source:
  https://github.com/torvalds/linux

- Rust for Linux:
  https://github.com/Rust-for-Linux/linux

- Rust for Linux general information:
  https://docs.kernel.org/rust/general-information.html

- DMA API HOWTO:
  https://docs.kernel.org/core-api/dma-api-howto.html

---

## MT76 / MT7921

- MT7921 PCIe driver:
  https://github.com/torvalds/linux/blob/master/drivers/net/wireless/mediatek/mt76/mt7921/pci.c

- MT792x core:
  https://github.com/torvalds/linux/blob/master/drivers/net/wireless/mediatek/mt76/mt792x_core.c

- MT76 Makefile:
  https://github.com/torvalds/linux/blob/master/drivers/net/wireless/mediatek/mt76/Makefile

- MT7921 Makefile:
  https://github.com/torvalds/linux/blob/master/drivers/net/wireless/mediatek/mt76/mt7921/Makefile

- MT7921 Kconfig:
  https://github.com/torvalds/linux/blob/master/drivers/net/wireless/mediatek/mt76/mt7921/Kconfig

---

# 読む順番

```text
README
  ↓
00-source-map
  ↓
01-environment
  ↓
phase01
  ↓
phase02
  ↓
phase03
  ↓
...
  ↓
phase10
  ↓
11-debugging
  ↓
12-rust-design
  ↓
13-lab-checklist
  ↓
14-reading-strategy
  ↓
15-final-architecture
```

---

# 注意

Linux/Rust-for-Linux は開発が継続しているため、Rust API の具体的なシグネチャは教材作成時点と将来の kernel tree で変わる可能性がある。

**実装時には、手元の kernel tree の `rust/kernel/` と `samples/rust/` を一次資料として扱うこと。**
