# 実験チェックリスト

## Phase 1

- [ ] lspci で BDF を確認
- [ ] vendor/device ID を記録
- [ ] 既存 driver を確認
- [ ] Rust module が build
- [ ] probe log
- [ ] remove log

## Phase 2

- [ ] BAR サイズ確認
- [ ] MMIO abstraction 確認
- [ ] chip ID read
- [ ] register wrapper
- [ ] read-only test
- [ ] safe/unsafe boundary 文書化

## Phase 3

- [ ] DMA API を読む
- [ ] DMA buffer allocation
- [ ] DMA address
- [ ] mapping/unmapping
- [ ] descriptor
- [ ] ring
- [ ] ownership
- [ ] interrupt

## Phase 4

- [ ] firmware name
- [ ] firmware request
- [ ] firmware size/version
- [ ] firmware download
- [ ] completion event
- [ ] failure path

## Phase 5

- [ ] MCU command
- [ ] sequence/token
- [ ] event
- [ ] timeout
- [ ] interrupt
- [ ] command state machine

## Phase 6

- [ ] ieee80211_hw
- [ ] wiphy
- [ ] ieee80211_ops
- [ ] wlan interface
- [ ] start/stop

## Phase 7

- [ ] RX ring
- [ ] NAPI
- [ ] descriptor parse
- [ ] skb
- [ ] RX completion
- [ ] mac80211 delivery

## Phase 8

- [ ] TX callback
- [ ] TX descriptor
- [ ] DMA mapping
- [ ] token
- [ ] completion
- [ ] skb lifetime

## Phase 9

- [ ] scan callback
- [ ] MCU scan
- [ ] scan event
- [ ] result reporting
- [ ] cancellation

## Phase 10

- [ ] station state
- [ ] BSS configuration
- [ ] authentication
- [ ] association
- [ ] keys
- [ ] WPA2 test
- [ ] data traffic

---

# 各 Phase で残すもの

各 Phase の終了時に、

```text
phaseXX/
  README.md
  diagram.md
  notes.md
  source-notes.md
  code/
```

を残す。

特に `source-notes.md` には、

```text
C source:
    function:
    reason:

Rust:
    file:
    abstraction:

Difference:
    ...
```

を書く。

これを続けると、最終的に自分専用の mt7922 driver design document になる。
