# Win11 Tweaks

自分用 Windows11 設定ツール

![ui](https://raw.githubusercontent.com/niumlaque/i/refs/heads/master/i/ec98107f7683a0a47a66ca3c9216800f.png)

## 機能
* エクスプローラ関連
  * 右クリックメニューを Windows10 と同じものに
  * エクスプローラ起動時の画面をいわゆる `マイコンピュータ` に(古い)
  * ファイルの拡張子表示
* タスクバー関連
  * スタートメニュー位置変更
  * 検索ボックス
  * タスクビュー
  * チャット
  * ウィジェット非表示 (現在は何故か Access Denied になる)
  * Windows Copilot (Preview) 非表示

## ビルド方法

初回のみ
```ps
> cargo install create-tauri-app --locked
> cargo install tauri-cli
```
```ps
> cargo tauri build
```
バイナリは `src-tauri/target/release` に生成される。

`cargo tauri build` は `.cargo/config.toml` の設定を見てくれないみたいなので、シングルバイナリを作成したい場合は以下を実行する。

```ps
> cd src-tauri
> cargo build --release
```
