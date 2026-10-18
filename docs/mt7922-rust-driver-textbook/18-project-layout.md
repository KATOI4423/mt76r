# 推奨プロジェクト構成

最初は極小構成から始める。

## Phase 1

```text
mt7922-rs/
├── Makefile / kernel integration
├── Kconfig
└── src/
    ├── lib.rs
    └── pci.rs
```

## Phase 2

```text
src/
├── lib.rs
├── pci.rs
├── mmio.rs
└── regs.rs
```

## Phase 3

```text
src/
├── dma.rs
├── dma/
│   ├── ring.rs
│   ├── descriptor.rs
│   └── buffer.rs
```

## Phase 5

```text
src/
├── mcu.rs
├── mcu/
│   ├── command.rs
│   ├── event.rs
│   └── protocol.rs
```

## Phase 7+

```text
src/
├── rx.rs
├── tx.rs
├── scan.rs
├── station.rs
└── mac.rs
```

---

# 分割ルール

ファイルを分割する基準は「C driver と同じファイル数」ではない。

以下の責務で分ける。

```text
PCI lifecycle
MMIO
register definitions
DMA memory
DMA ring
MCU protocol
MCU transport
RX
TX
mac80211
scan
station
```

一つの module が hardware register / DMA / mac80211 の全てを直接触らないようにする。
