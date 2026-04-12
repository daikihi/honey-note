# honey-note

ハチミツの管理を行うためのプロジェクトです。ハチミツのコレクションを登録・管理することができます。

## プロジェクト構成

このプロジェクトは以下のサブプロジェクトで構成されています。

- `server`: Actix-webを使用したAPIサーバーと静的ファイルの配信
- `front`: Rust + WebAssembly (wasm-pack) によるフロントエンド
- `batchs`: マスターデータや初期データの登録用バッチツール
- `common`: 各サブプロジェクトで共有されるインフラ・リポジトリ層
- `common-type`: 各サブプロジェクトで共有される型定義
- `resources`: データベースマイグレーションファイルとマスターデータ

## 設計概要

- [抽象デザイン](docs/AbustructDesignLogs.md)
- [データモデル設計](docs/design.md)
- [システム構成図](https://github.com/user-attachments/assets/330b914e-1d96-48c4-8480-9a4e344c53a8)
- [イベントストーミング図](https://github.com/user-attachments/assets/fb8d6349-a483-4388-942c-7e41c75982bf)

## 開発環境のセットアップ

### 必須ツール

- Rust (rustupを推奨: [rustup.rs](https://rustup.rs/))
- wasm-pack (フロントエンドのビルドに使用)
- sqlx-cli (データベースマイグレーションに使用)
- SQLite

```bash
# wasm-packのインストール
cargo install wasm-pack

# sqlx-cliのインストール
cargo install sqlx-cli --no-default-features --features sqlite
```

### データベースの準備

```bash
cd resources/db
# データベースファイルの作成とマイグレーションの実行
sqlx migrate run --database-url sqlite:./honey_note.db
```

デフォルトのデータベースファイル名は `common/src/infrastructure/db/sqlx.rs` の `DB_FILE_NAME` で定義されています。

## ビルドと実行

### フロントエンドのビルド

RustコードをWebAssemblyにコンパイルし、サーバーが配信するディレクトリに出力します。

```bash
cd front/
wasm-pack build --target web --out-dir ../server/src/assets/javascript/
```

### フロントエンドのテスト実行

WebAssembly コードのテストには、ブラウザ環境（Chrome 等）と `wasm-pack` が必要です。
ローカル環境に WebDriver (ChromeDriver, Geckodriver 等) がインストールされている必要があります。

```bash
cd front/
# Chrome ヘッドレスモードでのテスト実行
wasm-pack test --chrome --headless -- --test signup_tests
```

**テストが失敗する場合の確認事項:**

1. **ChromeDriver のバージョン**: ブラウザのバージョンと一致しているか確認してください。
2. **接続制限**: セキュリティソフト等で `127.0.0.1` への接続が制限されていないか確認してください。
3. **ヘッドレス設定**: GUI環境がある場合は `--headless` を外して実行し、ブラウザが起動するか確認してください。

※ テストコード本体は `front/tests/signup_tests.rs` にあり、Fetch API や リダイレクトをスタブ化して検証しています。

### サーバーの起動

```bash
# ルートディレクトリから実行
RUST_LOG=info cargo run -p server
```

サーバーはデフォルトで `http://127.0.0.1:8080/honey_note/honeys/lists.html` でアクセス可能です。
設定は `common/config/server/local.toml` で変更できます。

### バッチ処理

各バッチは `batchs` サブプロジェクトに含まれています。トランザクション内で処理されるため、再試行が可能です。

#### 都道府県マスター

```bash
RUST_LOG=info cargo run -p batchs --bin prefecture_loader resources/master_data/japanese_prefectures.csv resources/db/honey_note.db --user-id <USER_ID>
```

#### 花名マスター

```bash
RUST_LOG=info cargo run -p batchs --bin flower_loader resources/master_data/flower.csv resources/db/honey_note.db --user-id <USER_ID>
```

#### 養蜂業者マスター

```bash
RUST_LOG=info cargo run -p batchs --bin beekeeper_loader resources/master_data/beekeeper.csv resources/db/honey_note.db --user-id <USER_ID>
```

#### ハチミツデータ（JSONL形式）

```bash
RUST_LOG=info cargo run -p batchs --bin honey_loader resources/master_data/honey_data.jsonl resources/db/honey_note.db --user-id <USER_ID>
```

## APIエンドポイント

主要なエンドポイントは以下の通りです。ベースURLは `/honey-note/api` です。

### ハチミツ関連

- `GET /honeys`: ハチミツ一覧の取得
- `GET /honey/{id}`: ハチミツ詳細の取得
- `PUT /honey/new`: ハチミツの新規登録
- `PUT /honey/edit`: ハチミツの更新

### マスターデータ関連

- `GET /flowers`: 花一覧の取得
- `GET /flower/{id}`: 花詳細の取得
- `PUT /flower/new`: 花の新規登録
- `PUT /flower/edit/{id}`: 花の更新
- `GET /beekeepers`: 養蜂業者一覧の取得
- `GET /beekeeper/{id}`: 養蜂業者詳細の取得
- `PUT /beekeeper/new`: 養蜂業者の新規登録
- `PUT /beekeeper/edit/{id}`: 養蜂業者の更新
- `GET /prefectures`: 都道府県一覧の取得

### その他

- `GET /health`: ヘルスチェック

## 仕様書

詳細な仕様については以下のドキュメントを参照してください（日本語）。

- [API仕様書](specs/jp/api.md)
- [Webページ仕様書](specs/jp/web-endpoint.md)

## その他

- WSL（Ubuntu）を使用する場合は、`gcc` のインストールが必要です: `sudo apt install gcc`
- 動作ログを表示するには、環境変数 `RUST_LOG=info` を設定してください。

## ライセンス

- 本プロジェクトには "ISO 3166 Countries with Regional Codes" のデータを使用しており、MITライセンスに基づき提供されています。
  <https://github.com/lukes/ISO-3166-Countries-with-Regional-Codes>
