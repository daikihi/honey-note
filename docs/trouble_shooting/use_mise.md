# mise を利用した Rust 開発環境の構築

## 1. 概要

mise を使用して、Rust および Rust 製の開発ツールを管理する。

本ドキュメントでは、macOS 環境における mise の導入、Rust のセットアップ、wasm-pack の導入、および実際に発生した問題と解決方法をまとめる。

### 動作確認環境

| 項目 | 環境 |
|---|---|
| OS | macOS (Apple Silicon / aarch64) |
| Shell | zsh |
| mise | 2026.10.17 |
| Rust | 1.99.1（stable） |
| rustup | 1.29.3 |
| wasm-pack | 0.15.0 |

確認日：2026-11-05

---

## 2. mise の導入

### 2.1 インストール

Homebrew を利用する場合：

```bash
brew install mise
```

インストール確認：

```bash
mise --version
```

### 2.2 zsh への組み込み

`~/.zshrc` に以下を追加する。

```bash
eval "$(mise activate zsh)"
```

シェルの設定を再読み込みする。

```bash
source ~/.zshrc
```

これにより、カレントディレクトリの `mise.toml` に指定されたツールが自動的に有効になる。

---

## 3. Rust のセットアップ

### 3.1 Rust のバージョン管理

プロジェクトのルートディレクトリで実行する。

```bash
mise use rust@stable
```

`mise.toml` に以下が追加される。

```toml
[tools]
rust = "stable"
```

`stable` を指定すると、特定のバージョン番号に固定せず、Rust の安定版チャネルを使用できる。

バージョンを固定したい場合は、次のように指定する。

```bash
mise use rust@1.99.1
```

### 3.2 インストール

```bash
mise install
```

インストール済みバージョンを確認する。

```bash
mise ls rust
```

実際に利用される Rust のバージョンを確認する。

```bash
mise exec -- rustc --version
mise exec -- cargo --version
```

### 3.3 rustup との関係

mise の Rust バックエンドは内部で rustup を利用する。

- **mise**：プロジェクトごとに使用する Rust のバージョンを決定する。
- **rustup**：Rust のツールチェーンをインストール・管理する。
- **cargo**：Rust のビルドおよびパッケージ管理を担当する。

rustup でインストールされたツールチェーンは以下で確認できる。

```bash
rustup toolchain list
```

mise 経由で選択されるツールチェーンは以下で確認する。

```bash
mise exec -- rustup show active-toolchain
```

mise は `RUSTUP_TOOLCHAIN` 環境変数を利用してツールチェーンを切り替えるため、rustup のデフォルトと mise が選択したバージョンが異なっていても問題ない。

---

## 4. wasm-pack のセットアップ

wasm-pack は、Rust で WebAssembly を開発するためのビルド・パッケージングツールである。

Rust を導入しただけでは wasm-pack はインストールされない。

### 4.1 mise によるインストール

プロジェクトのルートディレクトリで実行する。

```bash
mise use 'cargo:wasm-pack@0.15.0'
```

mise の `cargo:` バックエンドを利用して wasm-pack をインストールする。

`mise.toml` には以下の設定が追加される。

```toml
[tools]
rust = "stable"
"cargo:wasm-pack" = "0.15.0"
```

### 4.2 インストール確認

```bash
mise exec -- wasm-pack --version
```

### 4.3 WebAssembly のビルド

Rust の WebAssembly 対応クレートのディレクトリで実行する。

```bash
mise exec -- wasm-pack build \
  --target web \
  --out-dir ../server/src/assets/javascript/
```

`--target web` は、ブラウザで ES Modules として利用するための出力形式を指定する。

`--out-dir` は生成物の出力先を指定する。相対パスはビルド対象ディレクトリを基準とする。

---

## 5. トラブルシューティング

### 5.1 `mise WARN missing: rust@stable`

#### 症状

```text
✓ installed 1 tool: rust@1.99.1
mise WARN missing: rust@stable
```

`mise.toml` は `rust = "stable"` を指定しているのに、mise は `1.99.1` をインストールし、`stable` を未インストールと判定する。

#### 原因

今回の環境では、mise の `minimum_release_age` によるバージョン解決が影響していた。

`stable` が `1.99.1` に解決され、mise が管理するツールチェーンの識別とインストール状態の判定に不整合が生じていた。

#### 確認方法

通常の状態を確認する。

```bash
mise ls rust --json
```

次に、Rust をリリース保留の対象から一時的に除外して確認する。

```bash
MISE_MINIMUM_RELEASE_AGE_EXCLUDES=rust mise ls rust --json
```

今回の環境では、次のように変化した。

| 設定 | 解決されたバージョン |
|---|---|
| 通常 | `1.99.1` |
| Rust を保留対象から除外 | `stable` |

これによって、リリース保留設定がバージョン解決に影響していることを確認できた。

#### 解決方法

以下を実行する。

```bash
mise settings add minimum_release_age_excludes rust
mise install rust@stable
```

これにより Rust がリリース保留の対象から除外され、mise が `stable` チャネルをそのまま管理できるようになった。

#### 動作確認

```bash
mise ls rust
mise exec -- rustup show active-toolchain
```

正常時の出力例：

```text
rust  stable (symlink)  .../mise.toml  stable
rust  1.99.1 (symlink)
```

```text
stable-aarch64-apple-darwin
(overridden by environment variable RUSTUP_TOOLCHAIN)
```

`mise WARN missing: rust@stable` が表示されなくなれば解決。

**注意：** この設定では Rust に対するリリース保留が適用されなくなる。自動更新を有効にする設定ではない。また、今回の対処は mise 2026.10.17 で確認したものであり、他のバージョンでも必須とは限らない。

### 5.2 `wasm-pack couldn't exec process`

#### 症状

```text
mise ERROR "wasm-pack" couldn't exec process:
No such file or directory
```

#### 原因

Rust はインストールされているが、wasm-pack が利用可能な状態になっていない。

#### 解決方法

```bash
mise use 'cargo:wasm-pack@0.15.0'
```

導入確認：

```bash
mise exec -- wasm-pack --version
```

これにより、mise が管理するツールとして wasm-pack を実行できるようになった。

### 5.3 Rust が既にインストールされているという警告

#### 症状

```text
It looks like you have an existing installation of Rust
cannot install while Rust is installed
```

#### 原因

rustup が、既存の Rust コマンドまたは mise の shim を検出している。

#### 対応

まずインストール処理の最終結果を確認する。

```bash
mise ls rust
rustup toolchain list
```

今回の環境では、警告後もインストールが続行され、正常に完了した。

そのため、このメッセージだけを理由に既存の Rust や rustup をアンインストールする必要はない。

なお、途中の `cannot install` が最終的な失敗になっている場合には、別途原因調査が必要。

---

## 6. 開発時によく使うコマンド

| 目的 | コマンド |
|---|---|
| mise のバージョン確認 | `mise --version` |
| プロジェクトのツールをインストール | `mise install` |
| インストール状況の確認 | `mise ls` |
| 設定ファイルの確認 | `mise config ls` |
| Rust stable の指定 | `mise use rust@stable` |
| Rust のバージョン確認 | `mise exec -- rustc --version` |
| rustup のツールチェーン一覧 | `rustup toolchain list` |
| 現在の Rust ツールチェーン確認 | `mise exec -- rustup show active-toolchain` |
| wasm-pack のバージョン確認 | `mise exec -- wasm-pack --version` |
| mise の設定確認 | `mise settings ls` |

`mise exec -- <command>` は、mise が選択したツールと環境変数を利用して任意のコマンドを実行する。

---

## 7. 新しい開発環境の構築手順

既に mise が導入されている環境では、以下の順序でセットアップする。

### 7.1 プロジェクトのツールを定義

`mise.toml`：

```toml
[tools]
rust = "stable"
"cargo:wasm-pack" = "0.15.0"
```

### 7.2 ツールをインストール

```bash
mise install
```

`rust@stable` の警告が発生した場合のみ、5.1 の手順で原因を確認する。

### 7.3 正常性を確認

```bash
mise ls
mise exec -- rustc --version
mise exec -- cargo --version
mise exec -- wasm-pack --version
mise exec -- rustup show active-toolchain
```

### 7.4 WebAssembly をビルド

対象クレートのディレクトリに移動して実行する。

```bash
mise exec -- wasm-pack build \
  --target web \
  --out-dir ../server/src/assets/javascript/
```

---

## 8. 運用方針

プロジェクトでは、開発ツールのバージョンを可能な限り `mise.toml` で管理する。

- Rust は `stable` を利用する。
- wasm-pack は動作確認済みの `0.15.0` に固定する。
- プロジェクトの `mise.toml` を Git で管理する。
- 新しい環境では `mise install` を利用する。
- mise に起因する問題が発生した場合は、先に設定とツールチェーンの解決結果を確認する。
- 既存の rustup や Rust のインストールを安易に削除しない。

これにより、複数の Mac や新しい開発環境でも、同じツール構成を再現しやすくなる。

## 9. 参考資料

- [mise 公式ドキュメント](https://mise.jdx.dev/)
- [mise Rust バックエンド](https://mise.jdx.dev/lang/rust.html)
- [mise Cargo バックエンド](https://mise.jdx.dev/dev-tools/backends/cargo.html)
- [rustup 公式ドキュメント](https://rust-lang.github.io/rustup/)
- [wasm-pack 公式リポジトリ](https://github.com/rustwasm/wasm-pack)

---

## 10. 今回の解決結果

2026-11-05、HoneyNote の開発環境において以下を確認した。

- `rust@stable` のインストールに成功。
- `minimum_release_age_excludes` の設定後、mise の missing 警告が解消。
- mise 経由で `stable-aarch64-apple-darwin` を選択できることを確認。
- `cargo:wasm-pack@0.15.0` を導入。
- wasm-pack を利用した WebAssembly ビルドを実行可能になった。

最終的に、Rust と wasm-pack の双方を mise で管理できる状態になった。