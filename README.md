# honey-note

このプロジェクトはハチミツの管理を行うためのものです。ハチミツのコレクションをしている方は、このプロジェクトを使ってハチミツを登録・管理できます。

## 設計概要

![Abstract image of what will we make](docs/AbustructDesignLogs.md)

![data model design](docs/design.md)

![Overview Architecture](https://github.com/user-attachments/assets/330b914e-1d96-48c4-8480-9a4e344c53a8)

イベントストーミング図

![Events](https://github.com/user-attachments/assets/fb8d6349-a483-4388-942c-7e41c75982bf)

## データベースマイグレーション

```bash
$ cd resources/db
$ sqlx migrate run --database-url sqlite:${PATH_TO_YOUR_SQLITE_DB_FILE}

# 例
$ sqlx migrate run --database-url sqlite:./honey_note.db
```

デフォルトのデータベースファイルパスは `common/src/infrastructure/db/sqlx.rs` にある `DB_FILE_NAME` 定数で指定されています。別のファイルを使用したい場合は定数を書き換えるか、バッチなど実行時に引数で指定してください。

SQLiteのデータベースファイルを削除した状態でサーバーが起動していると、古いファイルへのコネクションが残るため、サーバーを再起動してください。

## 実行方法

このプロジェクトは主にRustで書かれています。環境にはRustがインストールされている必要があります。
Rustのインストールにはrustupを推奨します。インストール手順は [rustup.rs](https://rustup.rs/) を参照してください。

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### ビルド

```bash
$ cd $honey_note_path
$ cargo install wasm-bindgen-cli  # frontサブプロジェクト向け
```

### バッチ

バッチ処理では各操作を**トランザクション**内で行うようになりました。途中でエラーが発生した場合はロールバックされるので、同じファイルを何度実行しても重複登録されにくくなっています。

以下ではバッチ処理の実行例を示します。

#### 都道府県マスターローダー

ファイルシステムから都道府県マスターデータを読み込み、DBに存在しない場合は登録します。

```bash
$ cargo run -p batchs --bin prefecture_loader resources/master_data/japanese_prefectures.csv $PATH_TO_DB_FILE
```

ログを表示したい場合:

```bash
$ RUST_LOG=info cargo run -p batchs --bin prefecture_loader resources/master_data/japanese_prefectures.csv $PATH_TO_DB_FILE
```

#### 花名マスターローダー

ファイルシステム上のマスターデータファイル（各行に花の名前）を読み込み、新しいデータのみをDBに保存します。拡張子は問いません。ヘッダー行は無視され、各行は1つの花名だけを含むと仮定します。

```bash
$ RUST_LOG=info cargo run -p batchs --bin flower_loader flower_master_data_directory/file_name.csv database_file.db
```

#### 養蜂業者マスターローダー

CSV形式の入力ファイルを処理し、養蜂業者マスターデータを更新・登録します。各行のフォーマットは次の通りです。

```
name_jp,name_en,prefecture_name,city_name
名前,英語表記,都道府県,都市
```

name_jp以外のフィールドは空でも構いません。

将来的にはフロントページの入力フォームからDBを更新できるようにする予定です。現在はプロトタイプとしての実装です。

実行例:

```bash
RUST_LOG=info cargo run -p batchs --bin beekeeper_loader resources/master_data/beekeeper.csv resources/db/honey_note.db
```

#### ハチミツマスターローダー

```
RUST_LOG=info cargo run -p batchs --bin honey_loader resources/master_data/${honey_master_data_name}.jsonl resources/db/${honey_db_name}.db
```

### Web サーバー

サーバーは `common/config/server/local.toml` などの設定ファイルからホスト名・ポート番号を読み込みます。
デフォルトでは `127.0.0.1:8080` ですが、別の環境を追加したい場合は同ディレクトリに環境名.toml を作成し、`main.rs` の `load_config("<env>")` 引数を変更してください。

```bash
cd $honey_note_top_directory
cargo run -p server
```

`$honey_note_top_directory` はリポジトリのルートディレクトリを指します。

#### JavaScript

RustコードからJavaScriptをビルドする方法です。最初に `wasm-pack` をインストールしてください。

```bash
cargo install wasm-pack
```

```bash
$ cd front/
$ wasm-pack build --target web --out-dir ../server/src/assets/javascript/
$ ls ../server/src/assets/javascript
front_bg.wasm      front_bg.wasm.d.ts front.d.ts         front.js           package.json
```

ビルド後、`server/src/assets/javascript` にファイルが配置されます。

## その他

- バッチやサーバーの動作ログを見たい場合は、`RUST_LOG=info` などで環境変数を設定して実行してください。
- WSL（Ubuntu）上で開発する場合、コンパイル用にgccをインストールしてください。

```bash
sudo apt install gcc
```

## ライセンス・関連情報

- 本プロジェクトには "ISO 3166 Countries with Regional Codes" のデータを使用しており、MITライセンスに基づき提供されています。
  <https://github.com/lukes/ISO-3166-Countries-with-Regional-Codes>

