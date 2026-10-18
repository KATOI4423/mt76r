# Phase 7 — RX

## ゴール

MT7922 → host の data path を完成させる。

```text
air
 ↓
MT7922
 ↓
DMA
 ↓
RX ring
 ↓
interrupt/NAPI
 ↓
descriptor parse
 ↓
skb
 ↓
mac80211
```

---

# 7.1 必読

```text
drivers/net/wireless/mediatek/mt76/dma.c
drivers/net/wireless/mediatek/mt76/agg-rx.c
drivers/net/wireless/mediatek/mt76/mt7921/mac.c
```

---

# 7.2 RX descriptor

まず packet payload ではなく、

```text
RX descriptor
```

を理解する。

確認項目:

```text
length
packet type
error flags
channel
RSSI
rate
band
checksum
```

など。

---

# 7.3 NAPI

Linux networking では大量 packet のため、

```text
IRQ
 ↓
NAPI schedule
 ↓
poll()
 ↓
process multiple packets
```

という仕組みを使う。

NAPI を理解せずに RX 実装を進めない。

---

# 7.4 RX ring ownership

```text
driver owns
    ↓
device owns
    ↓
device writes packet
    ↓
driver owns
    ↓
deliver
    ↓
refill
```

という状態を図にする。

---

# 7.5 Rust の unsafe boundary

RX では unsafe が増える。

特に:

```text
DMA memory
descriptor
packet length
hardware-provided fields
```

を扱うため。

原則:

```text
unsafe {
    raw descriptor access
}
```

を狭い範囲に閉じ込め、

```text
safe parsed representation
```

を上位へ渡す。

---

# 7.6 最初の RX

最初:

```text
packet received
 ↓
log length
```

だけ。

次に:

```text
packet
 ↓
802.11 frame
 ↓
mac80211
```

へ。

---

# 7.7 テスト

AP が必要。

別 PC / スマートフォン等を AP にする。

まず beacon を受けられることを確認する。

その後:

```bash
iw dev wlanX scan
```

を Phase 9 で完成させる。

---

# 完了条件

- RX ring の ownership を説明できる
- NAPI の役割を説明できる
- descriptor parsing を Rust wrapper にできる
- 受信 packet を mac80211 へ渡せる
- use-after-free / double recycle の危険を説明できる
