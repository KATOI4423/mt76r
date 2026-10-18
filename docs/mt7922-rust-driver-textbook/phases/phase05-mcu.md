# Phase 5 — MCU communication

## ゴール

MT7922 の host ↔ firmware communication を理解し、

```text
Rust host driver
      |
      | command
      v
   WFDMA
      |
      v
    MCU FW
      |
      | event
      v
   WFDMA
      |
      v
Rust interrupt/event handler
```

を実装する。

---

# 5.1 最重要ソース

```text
drivers/net/wireless/mediatek/mt76/mt7921/mcu.c
drivers/net/wireless/mediatek/mt76/mt7921/pci_mcu.c
drivers/net/wireless/mediatek/mt76/mcu.c
```

---

# 5.2 MCU command を探す

検索:

```bash
grep -R "MCU_CMD" drivers/net/wireless/mediatek/mt76/mt7921
grep -R "mt76_mcu_send" drivers/net/wireless/mediatek/mt76
```

1 command を選ぶ。

最初は:

```text
get version
query capability
set driver ownership
```

のような read/query 系を優先。

---

# 5.3 command packet

command は単なる Rust struct ではない。

例えば:

```text
header
sequence
command ID
length
flags
payload
```

などの wire format を持つ。

Rust:

```rust
#[repr(C)]
struct McuHeader {
    ...
}
```

とする場合は、

- size
- alignment
- endian
- padding
- reserved bits

を必ず確認する。

---

# 5.4 event queue

MCU communication では、

```text
send command
     ↓
wait event
     ↓
match sequence/token
     ↓
complete request
```

という非同期処理が必要になる。

ここで初めて、

```text
Mutex
SpinLock
completion
wait queue
timer
workqueue
```

などの kernel concurrency を学ぶ。

---

# 5.5 timeout

必ず timeout を入れる。

```text
send
 ↓
wait 1 sec
 ↓
timeout
```

とする。

永遠に待つ driver は作らない。

---

# 5.6 interrupt と MCU

PCI interrupt:

```text
IRQ
 ↓
read interrupt status
 ↓
ack/clear
 ↓
schedule/poll
 ↓
process event
```

という流れを確認する。

RX/TX を本格的に扱う前に、MCU event のみ処理する構成を作る。

---

# 5.7 Rust の状態機械

例えば:

```rust
enum CommandState {
    Idle,
    Submitted { seq: u8 },
    Completed,
    TimedOut,
}
```

のように状態を明示する。

これは Rust 化の大きなメリットの一つ。

---

# 完了条件

- MCU command/event の違いを説明できる
- sequence/token の必要性を説明できる
- timeout を実装できる
- interrupt → event processing の流れを説明できる
- 1 command の送受信を Rust 側から完結できる
