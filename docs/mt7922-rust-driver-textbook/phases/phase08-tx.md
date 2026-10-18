# Phase 8 — TX

## ゴール

```text
mac80211
 ↓
Rust driver tx()
 ↓
802.11 frame
 ↓
TX descriptor
 ↓
DMA
 ↓
MT7922
 ↓
air
```

を完成させる。

---

# 8.1 必読

```text
drivers/net/wireless/mediatek/mt76/tx.c
drivers/net/wireless/mediatek/mt76/dma.c
drivers/net/wireless/mediatek/mt76/mt7921/mac.c
```

---

# 8.2 TX の特徴

TX は RX と逆方向だが、同じではない。

```text
software packet
 ↓
metadata
 ↓
TXWI / TX descriptor
 ↓
DMA map
 ↓
queue
 ↓
hardware
```

という処理が必要。

---

# 8.3 skb

Linux networking では `struct sk_buff` が中心。

Rust 側の abstraction がどこまであるか確認する。

検索:

```bash
grep -R "SkBuff" rust/kernel
grep -R "sk_buff" rust/kernel
```

不足していれば C shim を検討する。

---

# 8.4 token

mt76 の TX completion では token が重要。

```text
token
 ↓
TX buffer
 ↓
DMA
 ↓
hardware completion
 ↓
token lookup
 ↓
free/unmap
```

という関係を追う。

---

# 8.5 最初の TX

最初は packet を air に出さず、

```text
mac80211 tx callback
 ↓
parse skb
 ↓
build descriptor
 ↓
enqueue
```

まで。

次に hardware へ。

---

# 8.6 completion

TX 完了後に、

```text
DMA unmap
free skb
update stats
```

を正しく行う。

特に double free を警戒する。

---

# 8.7 RX + TX

Phase 8 完了時には、

```text
RX
TX
```

が独立して動く。

次に:

```text
ping
iperf3
```

等で data path を確認。

---

# 完了条件

- skb lifetime を説明できる
- TX DMA mapping の lifetime を説明できる
- TX descriptor と packet の関係を説明できる
- completion で buffer を回収できる
- RX/TX 同時動作で破綻しない
