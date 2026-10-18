# Phase 9 — Scan

## ゴール

```bash
iw dev wlanX scan
```

を動かす。

---

# 9.1 scan の全体像

```text
iw
 ↓
nl80211
 ↓
cfg80211/mac80211
 ↓
driver scan callback
 ↓
MCU command
 ↓
firmware
 ↓
channel hopping
 ↓
beacon/probe response
 ↓
event
 ↓
cfg80211 results
```

---

# 9.2 ソース

読む:

```text
drivers/net/wireless/mediatek/mt76/scan.c
drivers/net/wireless/mediatek/mt76/mt7921/main.c
drivers/net/wireless/mediatek/mt76/mt7921/mcu.c
```

検索:

```bash
grep -R "scan" drivers/net/wireless/mediatek/mt76/mt7921/main.c
grep -R "scan" drivers/net/wireless/mediatek/mt76/mt7921/mcu.c
```

---

# 9.3 scan state machine

Rust では状態を明示する。

```rust
enum ScanState {
    Idle,
    Requested,
    Running,
    Aborting,
    Completed,
}
```

実際の driver の state model を調べてから必要な状態を追加する。

---

# 9.4 asynchronous operation

scan は synchronous function にしない。

```text
request
 ↓
return
 ↓
hardware scans
 ↓
event
 ↓
results
 ↓
complete
```

という非同期処理。

---

# 9.5 cancellation

重要:

```bash
iw dev wlanX scan abort
```

相当の cancellation を考える。

module remove 中に scan が走っていたらどうするかも考える。

---

# 完了条件

```bash
iw dev wlanX scan
```

で AP list が取得できる。

さらに、

```bash
iw dev wlanX scan dump
```

で結果を確認する。

