# Rust for Linux out-of-tree モジュールのビルド問題メモ

**環境**: Ubuntu、カーネル `7.0.0-38-generic`
(gcc 15.2 でビルド、rustc 1.93.1、bindgen 0.72.1)

## 最終的に通ったコマンド

```bash
sudo apt install linux-lib-rust-$(uname -r) rustc-1.93
make -C ./kernel KDIR=/lib/modules/$(uname -r)/build RUSTC=/usr/bin/rustc-1.93
```

## 問題と原因・解決

| # | エラー | 原因 | 解決 |
|---|---|---|---|
| 1 | `rustc-abi: invalid rustc abi: 'x86-softfloat'` | デフォルトの rustup rustc (1.99.1) が新しすぎる。ヘッダの target.json は旧名 `x86-softfloat` を出力するが、新しい rustc では `softfloat` に変更されている | カーネルをビルドした rustc に合わせる |
| 2 | `E0463: can't find crate for core` | `build/rust` が壊れたシンボリックリンクだった。Ubuntu はビルド済みの `rust/` 成果物 (libcore.rmeta など) を別パッケージに分けている | `sudo apt install linux-lib-rust-$(uname -r)` |
| 3 | `E0514: found crate core compiled by an incompatible version of rustc` | rmeta は rustc のバージョン文字列が完全一致しないと読めない。rustup の 1.93.1 は同じコミットでも `(built from a source tarball)` が付かず不一致 | apt 版 `rustc-1.93` を入れ、`RUSTC=/usr/bin/rustc-1.93` を指定 |
| 4 | clang: `unknown argument: '-mpreferred-stack-boundary=3'` など | カーネルは gcc でビルドされており、CFLAGS に gcc 専用オプションが入っている。`LLVM=1` で clang を使うと拒否される | `LLVM=1` を外す |

## 確認に使ったコマンド

```bash
# カーネルをビルドした rustc / bindgen
grep -E 'RUSTC_VERSION_TEXT|BINDGEN' /lib/modules/$(uname -r)/build/include/config/auto.conf

# rust/ の中身 (リンク切れの確認)
readlink -f /lib/modules/$(uname -r)/build/rust
```

## 教訓

- out-of-tree の Rust ビルドでは、**カーネルをビルドした rustc と完全に同じ rustc** が必要 (rustup 版では不可)。
- **カーネルをビルドしたコンパイラ (gcc/clang) に合わせる**。Ubuntu のカーネルなら gcc なので `LLVM=1` は付けない。
- `RUST_LIB_SRC` や `rust-src` は、`rust/` がビルド済みの out-of-tree では不要。
- `warning: pahole version differs` は無視してよい。
- ロード時は Secure Boot やモジュール署名で `insmod` が失敗することがある。
