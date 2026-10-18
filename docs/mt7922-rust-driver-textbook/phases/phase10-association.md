# Phase 10 — Association

## ゴール

最終的に、

```text
iw
  ↓
scan
  ↓
select AP
  ↓
authentication
  ↓
association
  ↓
4-way handshake
  ↓
data
```

まで到達する。

---

# 10.1 重要な注意

association は「最後に callback を1個実装する」だけではない。

以下が連携する。

```text
mac80211
cfg80211
MCU
RX
TX
station state
key management
power management
```

つまり Phase 10 は、それまでの Phase が全部正しく動くことが前提。

---

# 10.2 ソース

読む:

```text
drivers/net/wireless/mediatek/mt76/mt7921/main.c
drivers/net/wireless/mediatek/mt76/mt7921/mac.c
drivers/net/wireless/mediatek/mt76/mt7921/mcu.c
drivers/net/wireless/mediatek/mt76/mt792x_core.c
```

検索:

```bash
grep -R "sta_add" drivers/net/wireless/mediatek/mt76/mt7921
grep -R "sta_state" drivers/net/wireless/mediatek/mt76/mt7921
grep -R "bss_info_changed" drivers/net/wireless/mediatek/mt76/mt7921
```

---

# 10.3 station state

mac80211 から見た station state を理解する。

```text
NOTEXIST
   ↓
NONE
   ↓
AUTH
   ↓
ASSOC
   ↓
AUTHORIZED
```

実際の state transitions は使用する kernel API を確認すること。

---

# 10.4 MCU

association に必要な処理の一部は MCU command になる。

```text
add station
set BSS
set key
set channel
```

など。

それぞれを C driver から一つずつ追う。

---

# 10.5 最終テスト

まず open network。

次に WPA2。

その後 WPA3 などへ進む。

最初から暗号方式を全部対象にしない。

---

# 10.6 最終確認

```bash
iw dev
iw dev wlanX link
ip link
ping <gateway>
```

さらに:

```bash
iperf3
```

で throughput を確認。

---

# 完了条件

最低限:

```text
PCI probe
BAR/MMIO
DMA
firmware
MCU
mac80211
RX
TX
scan
association
```

が一つの driver として接続される。

ただし production quality では、

```text
suspend/resume
runtime PM
reset/recovery
AER
rfkill
regulatory
power management
debugfs
tracepoints
firmware crash
race conditions
```

などが残る。

これらは次の教材テーマ。
