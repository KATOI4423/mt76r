# Cソースを読むための手順

## 1. ファイルを上から読まない

MT7922/mt7921 driver は大きい。

最初から全行を読むと挫折しやすい。

代わりに、

```text
symbol
 ↓
caller
 ↓
callee
 ↓
data structure
 ↓
register
```

で読む。

---

# 2. 例えば probe

最初:

```text
mt7921_pci_probe
```

を探す。

そこから:

```text
pcim_enable_device
pci_set_master
pci_iomap
mt76_alloc_device
mt792x_init
mt7921_dma_init
```

などへ飛ぶ。

---

# 3. data structure を追う

例えば:

```text
struct mt792x_dev
```

を見つけたら、

```text
どこで allocate?
どこで initialize?
どこで free?
どの thread/IRQ が access?
```

を調べる。

---

# 4. register を追う

```text
MT_...
```

を見つけたら、

```bash
grep -R "MT_XXXX" drivers/net/wireless/mediatek/mt76
```

で、

```text
definition
read
write
initialization
reset
```

を追う。

---

# 5. MCU command を追う

```text
command ID
 ↓
packet struct
 ↓
send function
 ↓
DMA queue
 ↓
interrupt
 ↓
event parser
 ↓
waiter
```

まで追う。

---

# 6. mac80211 callback を追う

```text
ieee80211_ops
```

から callback を一つ選ぶ。

例えば:

```text
tx
```

なら、

```text
mac80211 tx
 ↓
mt7921 tx callback
 ↓
mt76 tx
 ↓
DMA
 ↓
hardware
```

を追う。

---

# 7. grep の練習

```bash
rg "mt7921_pci_probe"
rg "MT_HW_CHIPID"
rg "mt76_dma"
rg "mt792x_mcu"
rg "ieee80211_ops"
rg "sta_state"
```

`rg` がなければ `grep -R` でもよい。

---

# 8. git blame

疑問のあるコード:

```bash
git blame -L <start>,<end> path/to/file.c
```

commit を見る:

```bash
git show <commit>
```

driver の変化理由を調べる。

これは特に hardware workaround を理解するのに有効。

---

# 9. git log

```bash
git log --oneline -- drivers/net/wireless/mediatek/mt76/mt7921/pci.c
```

「なぜこの処理が必要なのか」がコメントだけで分からない場合、history を見る。

---

# 10. 一番重要

C driver にある、

```text
if
retry
timeout
reset
workqueue
special case
```

は、かなりの確率で hardware の癖を吸収している。

Rust 化の際に「不要そうだから削除」しない。

まず、

```text
なぜ存在するか
```

を git history / datasheet / firmware behavior から調べる。
