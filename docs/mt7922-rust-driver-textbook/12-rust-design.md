# Rust 化の設計指針

## 目的

C driver のコード量を Rust に移すことではなく、

```text
hardware invariants
lifetime
ownership
state machine
```

を Rust type system に反映する。

---

# 1. resource ownership

C:

```text
probe()
  |
  +-- allocate
  +-- map
  +-- enable
```

Rust:

```text
Driver
 ├── PciResources
 ├── Registers
 ├── DmaRings
 ├── FirmwareState
 └── Mcu
```

のように所有関係を表す。

---

# 2. lifetime

例えば:

```text
Registers<'a>
```

が device より長生きしてはいけない。

---

# 3. unsafe

unsafe は、

```text
hardware contract
```

をコードで証明できない境界に閉じ込める。

悪い例:

```rust
unsafe {
    // entire driver
}
```

良い方向:

```rust
fn read_descriptor(...) -> Result<Descriptor> {
    // narrow unsafe boundary
}
```

---

# 4. hardware state

enum を使う。

```rust
enum DeviceState {
    New,
    PciReady,
    MmioReady,
    DmaReady,
    FirmwareLoaded,
    McuReady,
    MacRegistered,
}
```

ただし本当に state transition を compiler で制約するなら typestate も検討できる。

---

# 5. descriptor

wire format と logical representation を分ける。

```text
RawDescriptor
      |
      v
parse()
      |
      v
RxDescriptor
```

上位コードは raw bitfield を直接触らない。

---

# 6. error

C:

```c
return -ENOMEM;
return -ETIMEDOUT;
```

Rust:

```rust
Result<T>
```

へ。

ただし kernel error code と Rust error type の変換方法は手元の Rust-for-Linux API を確認する。

---

# 7. concurrency

MT7922 driver は、

```text
IRQ
NAPI
workqueue
MCU response
TX completion
RX
scan
association
remove
```

が同時に進む。

Rust compiler は race を全部防ぐわけではない。

特に hardware race / protocol race は driver 設計側の責任。

---

# 8. C shim

Rust-for-Linux で abstraction が不足している場合、

```text
Rust
 |
 +-- safe wrapper
       |
       +-- C helper
             |
             +-- kernel API
```

を作ることは学習上合理的。

ただし:

```text
unsafe boundary
ownership
nullability
lifetime
```

を wrapper のコメントに明記する。

---

# 9. コメント

特に unsafe には、

```text
Safety:
- ...
- ...
```

のように、なぜ安全なのかを書く。

「C driver と同じだから」は安全性の説明にならない。

---

# 10. 最終的な module 構造

例:

```text
mt7922-rs/
├── lib.rs
├── pci.rs
├── regs.rs
├── mmio.rs
├── dma.rs
├── firmware.rs
├── mcu.rs
├── mac.rs
├── rx.rs
├── tx.rs
├── scan.rs
├── station.rs
└── debug.rs
```

ただし最初からこの構造にしない。

Phase 1:

```text
lib.rs
pci.rs
```

Phase 2:

```text
mmio.rs
regs.rs
```

のように段階的に増やす。
