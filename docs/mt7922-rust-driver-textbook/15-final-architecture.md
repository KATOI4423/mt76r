# 最終アーキテクチャ案

Phase 10 時点で目指す構造。

```text
                         userspace
                            |
                       nl80211/iw
                            |
                            v
                    +---------------+
                    |   mac80211    |
                    +-------+-------+
                            |
                    +-------v-------+
                    |    mac.rs     |
                    +-------+-------+
                            |
        +-------------------+-------------------+
        |                   |                   |
        v                   v                   v
      tx.rs               rx.rs             scan.rs
        |                   |                   |
        +-------------------+-------------------+
                            |
                          mcu.rs
                            |
                         dma.rs
                            |
                       mmio.rs
                            |
                         pci.rs
                            |
                        MT7922
```

---

# ownership

```text
PciDevice
  |
  +-- Registers
  |
  +-- DmaState
  |     |
  |     +-- TxRing
  |     +-- RxRing
  |
  +-- FirmwareState
  |
  +-- McuState
  |
  +-- MacState
```

---

# lifecycle

```text
New
 |
 v
PCI_READY
 |
 v
MMIO_READY
 |
 v
DMA_READY
 |
 v
FIRMWARE_LOADED
 |
 v
MCU_READY
 |
 v
MAC_REGISTERED
 |
 v
RUNNING
 |
 +------> RESETTING
 |
 +------> STOPPING
 |
 v
REMOVED
```

---

# 競合

```text
                    +--> IRQ
                    |
device ------------+--> NAPI
                    |
                    +--> workqueue
                    |
                    +--> MCU event
                    |
                    +--> scan
                    |
                    +--> association
                    |
                    +--> remove
```

最終的にはこの race を設計できるようになることが目的。

---

# production へ進む場合

Phase 10 の後に、

1. suspend/resume
2. runtime PM
3. ASPM
4. PCI AER
5. device reset
6. firmware crash recovery
7. rfkill
8. regulatory
9. power save
10. WoWLAN
11. P2P
12. AP mode
13. monitor mode
14. debugfs
15. tracepoints
16. statistics
17. thermal
18. upstream quality

へ進む。

ここから先は「教材」ではなく「driver development」になる。
