# Phase 6 — mac80211 registration

## ゴール

ここで初めて Linux の Wi-Fi stack と接続する。

```text
Rust MT7922
    |
    v
mac80211
    |
    v
cfg80211
    |
    v
userspace
```

---

# 6.1 必読

```text
include/net/mac80211.h
```

さらに:

```text
net/mac80211/
net/wireless/
```

を読む。

---

# 6.2 MT7921 main.c

読む:

```text
drivers/net/wireless/mediatek/mt76/mt7921/main.c
```

特に:

```text
ieee80211_ops
ieee80211_hw
wiphy
```

を探す。

---

# 6.3 callback の考え方

mac80211 が driver に、

```text
start
stop
add_interface
remove_interface
config
configure_filter
bss_info_changed
sta_state
tx
scan
```

などを要求する。

最初から全部実装しない。

---

# 6.4 最小 registration

まず:

```text
allocate ieee80211_hw
set supported bands
set basic capabilities
register
```

だけ。

TX/RX は未実装でもよい段階を作る。

---

# 6.5 Rust type design

C:

```c
struct ieee80211_hw
struct ieee80211_ops
struct wiphy
```

を Rust abstraction がどこまで提供しているか確認する。

検索:

```bash
grep -R "ieee80211" rust/kernel
grep -R "mac80211" rust/kernel
```

もし abstraction が不足している場合、

```text
Rust
  ↓
small C shim
  ↓
mac80211
```

を一時的に使用する設計も検討する。

これは「Rust driver の学習」と「Rust-for-Linux abstraction の不足」を分離するため。

---

# 6.6 最初の成功

目標:

```bash
iw dev
```

で wlan interface が見えること。

まだ、

```bash
iw dev wlanX scan
```

は動かなくてよい。

---

# 6.7 debug

```bash
iw phy
iw dev
```

を使う。

また、

```bash
dmesg -w
```

で callback の呼び出し順を確認する。

---

# 完了条件

- mac80211 と cfg80211 の役割を説明できる
- `ieee80211_hw` の役割を説明できる
- `ieee80211_ops` の callback model を説明できる
- 自作 driver が wlan interface を登録できる
- RX/TX 未実装の状態を正常に扱える
