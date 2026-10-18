# MT7922 Rust Driver 学習教本

## 目的

この教材は、MediaTek MT7922 PCIe Wi-Fi デバイスを題材に、

1. PCIe probe
2. BAR / MMIO
3. DMA
4. firmware loading
5. MCU communication
6. mac80211 registration
7. RX
8. TX
9. scan
10. association

の順に、Linux kernel driver と Rust-for-Linux を実際に触りながら学ぶための教材です。

**重要:** これは「mt76 の C コードを機械的に Rust に翻訳する教材」ではありません。
各 Phase でまず「ハードウェアが何を要求しているか」を C 実装から読み取り、その後に Rust 側で最小限の実装を作ります。

最終目標は、既存の mt76/mt7921e と同じ機能を持つことですが、途中では既存ドライバと同時にデバイスを操作しないようにしてください。

---

## 0. 前提

### 想定ハードウェア

- MediaTek MT7922 PCIe
- デスクトップ PC の空き PCIe スロット
- Linux
- x86_64
- IOMMU が有効でも動作できる設計を目標とする

MT7922 の PCI ID は実機によって異なる場合があります。まず必ず以下で確認してください。

```bash
lspci -nn | grep -i -E 'network|wireless|mediatek'
lspci -nn -s <BDF>
```

例:

```text
0000:xx:yy.z Network controller [0280]: MediaTek ... [14c3:7922]
```

**PCI ID をコードへ固定する前に、自分のカードの `vendor:device` を確認すること。**

---

## 1. 重要な方針

### 1.1 既存ドライバは「仕様書」として読む

MT7922 は独立した小さなドライバではありません。

概念的には、

```text
mac80211
   |
   v
mt76
   |
   +-- mt792x common
   |
   +-- mt7921e PCIe
            |
            +-- PCI
            +-- MMIO
            +-- WFDMA
            +-- MCU
            +-- firmware
```

という構造です。

現在の Linux tree では MT7921/MT7922 系 PCIe 実装は主に次のファイルに分かれています。

- `drivers/net/wireless/mediatek/mt76/mt7921/pci.c`
- `drivers/net/wireless/mediatek/mt76/mt7921/pci_mac.c`
- `drivers/net/wireless/mediatek/mt76/mt7921/pci_mcu.c`
- `drivers/net/wireless/mediatek/mt76/mt7921/mac.c`
- `drivers/net/wireless/mediatek/mt76/mt7921/mcu.c`
- `drivers/net/wireless/mediatek/mt76/mt7921/main.c`
- `drivers/net/wireless/mediatek/mt76/mt792x_core.c`
- `drivers/net/wireless/mediatek/mt76/mt792x_dma.c`
- `drivers/net/wireless/mediatek/mt76/dma.c`
- `drivers/net/wireless/mediatek/mt76/mac80211.c`

### 1.2 C → Rust は1:1変換しない

例えば C の

```c
void __iomem *regs;
```

を単純に Rust の raw pointer にするのは学習上も安全性上も良くありません。

Rust-for-Linux では、

```text
C API
  |
  v
bindings
  |
  v
kernel abstraction
  |
  v
Rust driver
```

という層を意識してください。

Rust-for-Linux の設計思想は、kernel C API への直接アクセスを abstraction に閉じ込め、Rust 側の利用者が soundness を保ちやすくすることです。

---

## 2. 開発環境

まず「普段使いの kernel」と「ドライバ開発用 kernel」を分離することを強く推奨します。

最初の段階では VM、別 SSD、または別 kernel entry を用意してください。

推奨:

```text
host Linux
  |
  +-- normal kernel
  |
  +-- development kernel
         |
         +-- Rust enabled
         +-- debug info
         +-- dynamic debug
         +-- lockdep / KASAN 等は必要に応じて
```

### 2.1 kernel source

Rust-for-Linux を学習するなら、ディストリビューションの kernel source だけでなく、Rust-for-Linux の開発 tree も確認します。

```bash
git clone https://github.com/Rust-for-Linux/linux.git
cd linux
```

ただし、**この教材は特定 commit の API を固定していません**。

最初に以下を記録してください。

```bash
git rev-parse HEAD
rustc --version
clang --version
gcc --version
```

この4つを学習ノートに残します。

---

## 3. 教材の進め方

各 Phase は以下の順番です。

```text
A. Linux kernel の基礎を読む
B. mt76 の C 実装を読む
C. ハードウェア上の状態を観測する
D. Rust abstraction を確認する
E. 最小 Rust 実装を書く
F. 失敗させる
G. エラー処理を追加する
H. 完了条件を確認する
```

### 絶対に避けること

- 既存の mt7921e を bind したまま自作ドライバを probe させる
- `/dev/mem` から適当に MMIO を叩く
- firmware を自作 driver と既存 driver の両方からロードする
- DMA buffer を Rust の lifetime だけで安全だと思い込む
- `unsafe` を増やして compiler error を黙らせる
- RX/TX を一度に全部実装する

---

# Phase 一覧

| Phase | 内容 | 主な学習テーマ |
|---|---|---|
| 1 | PCIe probe | module, PCI ID, probe/remove, resource |
| 2 | BAR / MMIO | BAR, ioremap, volatile I/O, register |
| 3 | DMA | DMA address, ring, descriptor, ownership |
| 4 | firmware | request_firmware, firmware format, download |
| 5 | MCU | command/event, interrupt, timeout, state machine |
| 6 | mac80211 | ieee80211_hw, wiphy, callbacks |
| 7 | RX | RX descriptor, DMA ring, NAPI, skb |
| 8 | TX | TX descriptor, DMA mapping, completion |
| 9 | scan | mac80211 scan callback, MCU scan |
| 10 | association | authentication/association, station state |

---

# 完了時の姿

最終的に、

```text
PCIe
  |
  +-- probe
  +-- BAR/MMIO
  +-- DMA
  +-- firmware
  +-- MCU
  +-- mac80211
  +-- RX
  +-- TX
  +-- scan
  +-- association
       |
       v
    wlanX
       |
       v
    normal Linux Wi-Fi stack
```

まで到達することを目標にします。

ただし、**MT7922 の完全な production-quality driver を Rust-for-Linux upstream API だけで完成させること自体が、教材の終了条件ではありません。**

本当に重要なのは、

```text
PCI device
  ↓
MMIO
  ↓
DMA
  ↓
firmware
  ↓
MCU
  ↓
802.11 hardware
  ↓
mac80211
```

という kernel driver の全体像を自分で説明できることです。
