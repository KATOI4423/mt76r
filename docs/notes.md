# Notes

## Rust for Linux のビルド

### 全体の流れ

```
./Makefile
  └ make -C $KDIR M=$PWD Mo=$OUT
    │ -C $KDIR: カーネルのビルドディレクトリの Makefile を実行する
    │ M=$PWD: 「外部モジュールを $PWD でビルドする」ことを指定（KBUILD_EXTMOD）
    │ MO=$OUT: 生成物を出力するディレクトリを指定（Kernel 6.13以降の方法）
    ▼
  $KDIR/Makefile
    │ M=... があると、デフォルトターゲットが modules になる
    ▼
  scripts/Makefile.build obj=. : ここで ./Kbuild が include される
    ▼
  scripts/Makefile.modfinal : .ko の仕上げ
```

 - scripts/Makefile.build が、対象ディレクトリ（`M=$PWD`で指定したディレクトリ）に `Kbuild`
   があればそれを、なければ `Makefile` を include する。 `Kbuild` が優先されるため、 `Makefile`
   はカーネルのビルド中には読まれない。そのため、 `Makefile` は人間用の入り口で、 `Kbuild` は
   カーネルのビルドシステムの入り口という役割分担となる。

 - 成果物は `M=...` で指定したディレクトリに出力される。 `-C` でディレクトリを移動するため、 
   `M=...` には絶対パスを指定する必要がある。

### Kbuild

``` bash
# SPDX-License-Identifier: GPL-2.0

obj-m := rust_out_of_tree.o
```

 - `obj-m` に指定したモジュールが作成される。
 - `obj-m := foo.o` で `foo.ko` を作る。その元は `foo.o` であるという記述方法で、
   `.o` ファイルは `.c` からも `.rs` からも作成できる。 kbuild はファイルの拡張子を見て、
   `.rs` の拡張子であれば rustc のルールを適応する。
 - Rust では `foo.rs` に `mod bar` を記述することで、 `bar.rs` が自動で読み込まれる。
   そのため、 `obj-m` の指定は `foo.o` だけで良い。
 - 別のモジュールを作る場合は、 `obj-m += hoge.o` と追加していく。

#### 複合オブジェクト指定

``` bash
# SPDX-License-Identifier: GPL-2.0

obj-m := mt76r.o        # mt76.ko というモジュールを作る
mt76r-y := src/lib.o    # mt76.o はsrc/lib.o から作る
```

 - `<name>-y` には、そのモジュールを構成するオブジェクトファイルを列挙する。
   kbuild はこれらを順番にビルドし、 `ld -r` で1つのモジュールにリンクして、`.ko` に仕上げる。
 - `mt76r.rs` や `mt76r.c` など、同名のソースコードがある場合は、 `-y` を書かなくても単体でビルドされる。
   `-y` を書くのは、元になるファイルの名前がモジュール名と違う場合や、複数ファイルから作る場合である。

#### Tips

`CONFIG_...` と組み合わせて、特定の `CONFIG_...` が有効（=y）のときだけソースコードを追加するという方法ができる。

``` Makefile
# CONFIG_FOO_DEBUG=y のとき、 foo-y に展開され、 kbuild からリンクされる
foo-$(CONFIG_FOO_DEBUG) += debug.o
```

---

### Rust for Linux out-of-tree モジュールのビルド問題メモ

**環境**: Ubuntu、カーネル `7.0.0-38-generic`
(gcc 15.2 でビルド、rustc 1.93.1、bindgen 0.72.1)

#### 最終的に通ったコマンド

```bash
sudo apt install linux-lib-rust-$(uname -r) rustc-1.93
make KDIR=/lib/modules/$(uname -r)/build RUSTC=/usr/bin/rustc-1.93
```

#### 問題と原因・解決

| # | エラー | 原因 | 解決 |
|---|---|---|---|
| 1 | `rustc-abi: invalid rustc abi: 'x86-softfloat'` | デフォルトの rustup rustc (1.99.1) が新しすぎる。ヘッダの target.json は旧名 `x86-softfloat` を出力するが、新しい rustc では `softfloat` に変更されている | カーネルをビルドした rustc に合わせる |
| 2 | `E0463: can't find crate for core` | `build/rust` が壊れたシンボリックリンクだった。Ubuntu はビルド済みの `rust/` 成果物 (libcore.rmeta など) を別パッケージに分けている | `sudo apt install linux-lib-rust-$(uname -r)` |
| 3 | `E0514: found crate core compiled by an incompatible version of rustc` | rmeta は rustc のバージョン文字列が完全一致しないと読めない。rustup の 1.93.1 は同じコミットでも `(built from a source tarball)` が付かず不一致 | apt 版 `rustc-1.93` を入れ、`RUSTC=/usr/bin/rustc-1.93` を指定 |
| 4 | clang: `unknown argument: '-mpreferred-stack-boundary=3'` など | カーネルは gcc でビルドされており、CFLAGS に gcc 専用オプションが入っている。`LLVM=1` で clang を使うと拒否される | `LLVM=1` を外す |

#### 確認に使ったコマンド

```bash
# カーネルをビルドした rustc / bindgen
grep -E 'RUSTC_VERSION_TEXT|BINDGEN' /lib/modules/$(uname -r)/build/include/config/auto.conf

# rust/ の中身 (リンク切れの確認)
readlink -f /lib/modules/$(uname -r)/build/rust
```

#### 教訓

- out-of-tree の Rust ビルドでは、**カーネルをビルドした rustc と完全に同じ rustc** が必要 (rustup 版では不可)。
- **カーネルをビルドしたコンパイラ (gcc/clang) に合わせる**。Ubuntu のカーネルなら gcc なので `LLVM=1` は付けない。
- `RUST_LIB_SRC` や `rust-src` は、`rust/` がビルド済みの out-of-tree では不要。

---

## 01-environment.md

### 0.1 PCI デバイスを確認

```bash
$ lspci -nnk
...
06:00.0 Network controller [0280]: MEDIATEK Corp. MT7922 802.11ax PCI Express Wireless Network Adapter [14c3:0616]
        Subsystem: Foxconn International, Inc. Device [105b:e0db]
        Kernel driver in use: mt7921e
        Kernel modules: mt7921e
...
```

#### バス番号

`06:00.0` の `06` の部分（`:` の前の番号）。16進数で 00~ff までの値を取る。
`バス(Bus)` とは、CPU、メモリ、GPU、ストレージなど、コンピュータ内部の各部品を接続し、
データのやり取りを行う共通の伝送路のこと。

#### デバイス番号

`06:00.0` の `00` の部分（`:` と `.` の間の番号）。16進数で 00~1f までの値を取る。
バス上のスロットやデバイスを示す番号。

#### ファンクション番号

`06:00.0` の `0` の部分（`.` の後の番号）。0~7の値を取る。
1つのデバイスが複数の機能（例えば、サウンドとネットワークの統合カードなど）を持つ場合、それらを区別するための番号。

#### デバイスクラス

`Network controller [0280]` の部分。どんなデバイスなのかの種類を示す。
`[0280]` はデバイスクラスコードと呼ばれ、4桁の16進数で分類される。

例:
 - 0000: Non-VGA unclassified device (非VGA非クラスデバイス)
 - 0108: Non-Volatile memory controller (不揮発性メモリコントローラ)
 - 0200: Ethernet controller (イーサネットコントローラ)

#### ベンダー

`MEDIATEC Corp.` の部分。`[14c3:0616]` の `14c3` (`:` の前の番号) がベンダーコードに当たり、16進数の値となる。

#### 名前

`MT7922 802.11ax PCI Express Wireless Network Adapter` の部分。

#### デバイスID

`[14c3:0616]` の `0616` の部分（`:` の後の番号）。

#### サブシステム

`Subsystem: Foxconn International, Inc. Device [105b:e0db]` の部分。 デバイスが搭載されているマザーボードやノートPCなどのホストシステムのベンダー名とモデル名を示す。

#### Kernel driver in use / Kernel modules

指定された PCI デバイスを制御しているカーネルドライバの名前を示す。

 - Kernel driver in use: 実際にデバイスにバインドされ、使用中のドライバ名
 - Kernel modules: そのデバイスをサポートするためにロード可能、利用可能なカーネルモジュールのリスト

#### 情報まとめ

 - Vendor ID: `14c3`
 - Device ID: `0616`
 - Subsystem Vendor ID: `105b`
 - Subsystem Device ID: `e0db`
 - Kernel driver in use: `mt7921e`
 - Kernel modules: `mt7921e`

---
