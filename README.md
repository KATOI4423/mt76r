# mt76r

**MediaTek MT76 Linux Wi-Fi Driver — Rust Learning Project**

## 概要

`mt76r` は、MediaTek MT7922 PCIe Wi-Fi アダプタを題材として、Linux カーネルの Wi-Fi ドライバの仕組みを学び、Rust による実装を段階的に試みる学習用プロジェクトです。

既存の Linux カーネル内の `mt76` ドライバを調査し、ハードウェアの初期化から Wi-Fi の送受信に至るまでの処理を理解することを目的としています。

本リポジトリでは、**日本語の学習用ドキュメントと Rust による実験的な実装を一緒に管理**します。

## プロジェクトの目的

主な目的は以下のとおりです。

- Linux カーネルのデバイスドライバの基本構造を理解する。
- PCIe デバイスの検出、BAR、MMIO、DMA の仕組みを学ぶ。
- ファームウェアのロードと MCU との通信を理解する。
- Linux の無線 LAN サブシステムである `mac80211` とドライバの関係を理解する。
- RX/TX データパスの実装を調査する。
- Rust の所有権、型システム、安全性の仕組みをカーネルドライバ開発に応用する。
- 既存の C 言語による実装と比較しながら、Rust による実装方法を検討する。

## 対象ハードウェア

- **デバイス:** MediaTek MT7922
- **接続方式:** PCIe
- **対象 OS:** Linux
- **実装言語:** Rust
- **既存実装の調査対象:** Linux カーネルの `mt76` ドライバ

MT7922 と関連する既存ドライバの実装を基準として、ハードウェアの動作と Linux カーネル側の処理を段階的に調査します。

## 学習ロードマップ

実装は以下のフェーズに分けて進めます。

| フェーズ | 内容 |
|---|---|
| 1 | PCIe デバイスの検出と probe |
| 2 | BAR と MMIO によるレジスタアクセス |
| 3 | DMA の初期化とメモリ管理 |
| 4 | ファームウェアのロード |
| 5 | MCU との通信 |
| 6 | `mac80211` への登録 |
| 7 | RX データパス |
| 8 | TX データパス |
| 9 | Wi-Fi スキャン |
| 10 | アクセスポイントへの接続（Association） |

各フェーズでは、既存実装の調査、処理の理解、Rust での実装検討、動作確認という順序で進めます。

実装状況や学習内容は、ドキュメントとソースコードに随時反映します。

## ファームウェア

本プロジェクトでは、既存の Linux ファームウェア配布物である `linux-firmware` が提供する MT7922 用ファームウェアを利用します。

**ファームウェアのバイナリは、本リポジトリには同梱しません。** 開発環境にインストールされたファームウェアを利用する方針です。

主に以下のファイルを使用します。

- `mediatek/WIFI_MT7922_patch_mcu_1_1_hdr.bin`
- `mediatek/WIFI_RAM_CODE_MT7922_1.bin`

Ubuntu などの環境では、ファームウェアが `linux-firmware` パッケージによってインストールされ、`.bin.zst` という圧縮形式で配置されている場合があります。

標準的な配置先は以下です。

```text
/lib/firmware/mediatek/
```

開発環境におけるファイルの存在は、次のコマンドで確認できます。

```bash
ls -l /lib/firmware/mediatek/WIFI*MT7922*
```

実装時には、対象カーネルの既存 `mt76` ドライバが要求するファイル名、ファームウェアのロード方法、圧縮形式への対応を確認します。

ファームウェアはドライバの Rust 実装とは別の既存バイナリであり、その内部を本プロジェクトで再実装するものではありません。

ファームウェアの取得元とライセンスについては、以下を参照してください。

- [linux-firmware リポジトリ](https://gitlab.com/kernel-firmware/linux-firmware)
- [linux-firmware のファームウェア情報・ライセンス一覧（WHENCE）](https://gitlab.com/kernel-firmware/linux-firmware/-/blob/main/WHENCE)

## 開発方針

### 既存ドライバを基準にする

Linux カーネルに含まれる既存の `mt76` ドライバを主要な調査対象とします。

既存実装の初期化順序、レジスタアクセス、DMA 管理、ファームウェア通信、送受信処理を調査し、それぞれの役割を理解したうえで Rust による実装を検討します。

### Rust for Linux

Linux カーネルで Rust を使用するための仕組みである Rust for Linux を調査対象とします。

実際の実装方法は、対象カーネルのバージョン、Rust 対応状況、利用可能なカーネル API に依存します。ユーザー空間の Rust プログラムとカーネルモジュールでは、利用できる標準ライブラリや実行環境が異なる点にも注意します。

### 段階的に実装する

最初から完全な Wi-Fi ドライバを実装するのではなく、PCIe デバイスの認識など、個別に検証できる機能から取り組みます。

各フェーズで、既存実装との対応関係、未実装の機能、動作確認の結果を記録します。

## 開発環境

開発環境の詳細や構築手順は、`docs/` 以下のドキュメントに記載します。

- [Github Pages](https://KATOI4423.github.io/mt76r/)

再現性を確保するため、検証時には以下の情報を記録する方針です。

- Linux カーネルのバージョンとソースコードのコミット
- Rust ツールチェーンのバージョン
- Rust for Linux の設定状況
- 使用したファームウェアと取得元
- ビルド結果、テスト結果、動作確認時のログ
- 対象ハードウェアと検証条件

## プロジェクトの状態

**状態：学習・実験段階（Work in Progress）**

本プロジェクトは、Linux カーネルドライバと Rust の学習を目的としたものです。

現時点で、実用可能な Wi-Fi ドライバとしての完成や、すべての MT7922 機能への対応を保証するものではありません。

実装済みの機能、未実装の機能、検証状況は開発の進行に応じて更新します。

本プロジェクトのコードを、実運用環境で使用することは想定していません。

## 参考資料

### Linux カーネルと無線 LAN ドライバ

- [Linux kernel](https://github.com/torvalds/linux)
- [Linux kernel — MediaTek mt76 ドライバ](https://github.com/torvalds/linux/tree/master/drivers/net/wireless/mediatek/mt76)
- [Linux kernel documentation](https://docs.kernel.org/)

### Rust for Linux

- [Rust for Linux](https://github.com/Rust-for-Linux/linux)
- [Rust for Linux documentation](https://docs.kernel.org/rust/)
- [Rust for Linux — out-of-tree module template](https://github.com/Rust-for-Linux/rust-out-of-tree-module)

### ファームウェア

- [linux-firmware](https://gitlab.com/kernel-firmware/linux-firmware)

## ライセンスと著作権

本リポジトリのソースコードとドキュメントのライセンスは、それぞれのライセンス表記およびファイル内の著作権表示に従います。

既存の Linux カーネルや `mt76` ドライバからコードを引用・移植する場合は、元ファイルのライセンス、SPDX 識別子、著作権表示、必要なライセンス条件を維持します。

学習用ドキュメントについても、既存資料から文章や図表を転載する場合は、その資料のライセンスと利用条件を確認します。

ファームウェアは本リポジトリに同梱せず、配布元が定めるライセンス条件に従います。

本プロジェクトは MediaTek、Linux カーネル開発コミュニティ、または `mt76` の開発者による公式プロジェクトではありません。
