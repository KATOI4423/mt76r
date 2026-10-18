# Phase 4 — Firmware loading

## ゴール

Linux firmware loader から firmware を取得し、

```text
filesystem
  ↓
kernel firmware loader
  ↓
Rust driver
  ↓
DMA / MCU
  ↓
MT7922
```

という流れを理解する。

---

# 4.1 まず firmware と driver を分ける

firmware は Rust に書き換える対象ではない。

基本構造:

```text
Host driver = Rust
Device firmware = MediaTek binary
```

この Phase では firmware binary の内部実装を理解する必要はない。

ただし、

```text
header
version
target
payload
checksum
download protocol
```

は調べる。

---

# 4.2 MT7921/MT7922 の C

読む:

```text
drivers/net/wireless/mediatek/mt76/mt7921/mcu.c
drivers/net/wireless/mediatek/mt76/mt7921/pci_mcu.c
drivers/net/wireless/mediatek/mt76/mt7921/init.c
```

検索:

```bash
grep -R "request_firmware" drivers/net/wireless/mediatek/mt76/mt7921
grep -R "FWDL" drivers/net/wireless/mediatek/mt76/mt7921
```

---

# 4.3 firmware loader

Linux の基本 API:

```c
request_firmware()
request_firmware_nowarn()
request_firmware_direct()
release_firmware()
```

を理解する。

firmware loader は driver が勝手に `/lib/firmware` を読む仕組みではない。

kernel firmware subsystem を通して要求する。

---

# 4.4 Rust API

Rust-for-Linux の firmware abstraction が手元の tree に存在するかを確認する。

検索:

```bash
grep -R "struct Firmware" rust/ drivers/ samples/rust/
grep -R "firmware" rust/kernel/ | head -100
```

API が未実装の場合は、

```text
Rust driver
    ↓
small C helper
    ↓
kernel firmware API
```

という bridge を使うことも教材として許容する。

ただし bridge の安全性を明示する。

---

# 4.5 firmware download を分解

MT7922 では概念的に:

```text
load ROM patch
load WM firmware
start firmware
wait event
verify
```

などの段階がある。

C code で状態遷移を追う。

---

# 4.6 最初の課題

実際に download せず、

```text
firmware file exists?
size?
version?
```

だけ確認する test path を作る。

その後:

```text
request firmware
  ↓
validate
  ↓
release
```

まで。

---

# 4.7 次の課題

既存 driver の firmware download packet / descriptor を読む。

まだ Wi-Fi を有効化しない。

---

# 完了条件

- firmware loader と device firmware を区別できる
- firmware binary を「Rust化する対象ではない」と説明できる
- firmware request の lifetime を説明できる
- MT7922 firmware download の状態遷移を図にできる
- firmware failure と PCI failure を区別できる
