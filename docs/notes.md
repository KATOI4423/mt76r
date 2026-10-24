# Notes

## Rust for Linux のビルド

### 全体の流れ

```
./Makefile
  └ make -C $KDIR M=$PWD
    │ -C $KDIR: カーネルのビルドディレクトリの Makefile を実行する
    │ -M=$PWD: 「外部モジュールを $PWD でビルドする」ことを指定（KBUILD_EXTMOD）
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

```
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
