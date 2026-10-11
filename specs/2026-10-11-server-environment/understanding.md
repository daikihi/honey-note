# server の環境別設定対応：仕様理解

## 目的

同一コード・同一ブランチの `server` を、起動時の `--env local|prod` で切り替える。環境ごとの Host、Port、SQLite 接続先、セッション Cookie の本番判定を設定と指定環境に連動させる。

## 成功条件

- 引数なしは `local`。`--env local` と `--env prod` を受け付け、不正値・値なしは理由を表示して非ゼロ終了する。`--help` は使用方法を表示する。
- `common/config/server/local.toml` は `127.0.0.1:8080` と既存 DB `sqlite://resources/db/honey_note.db` を使う。新規 `prod.toml` は `127.0.0.1:8081` と `sqlite:///var/lib/honey-note/honey_note.db` を初期値とする。
- `Server` に `database_url` を追加し、指定した環境の TOML を既存の `load_config` で読み込む。失敗時は環境名と原因を確認できる。
- `server` は設定の URL を既存の `get_sqlite_pool` に渡し、HTTP 起動前に DB 接続失敗を検出する。既存 DB ファイルは移動・削除・初期化しない。本番 DB の実体やディレクトリも作らない。
- `prod` はビルド種別を問わず本番扱いとし、64 バイトの `SESSION_SECRET_KEY` を必須にし、Cookie に `Secure` を付ける。`local` はビルド種別を問わず開発扱いとし、鍵未設定ならランダム生成を許す。既存の鍵長検証を維持する。
- API、認証・セッション、静的ファイル配信、既存 SQLite データ、`batchs` の DB 接続、引数なしでの起動を維持する。
- README に起動方法、設定位置、DB URL、秘密鍵、本番 DB とマイグレーション、リポジトリルートからの実行を記載する。
- 指定された CLI、環境判定、秘密鍵、DB エラーのケースを確認し、`cargo fmt --all -- --check`、`cargo check -p server`、`cargo test -p common -p server` を実行する。既存 DB を破壊する試験はしない。

## 入力・出力・振る舞い

- 入力：CLI の `--env`、環境別 TOML、必要に応じた `SESSION_SECRET_KEY`、配置済み SQLite DB。
- 出力：選択環境の Host/Port で HTTP サーバーを起動する。引数・設定・鍵・DB に問題があれば起動前に理由を表示して終了する。
- `Cors::permissive()` は今回維持する。本番はしばらく IP ベースで運用し、前段に Nginx などのリバースプロキシを置く予定。`prod` の初期値 `127.0.0.1:8081` はその背後で使う想定。デプロイ前に公開側のスキーム・IP・ポートと TLS 終端を決め、CORS の許可元を制限する必要がある旨を報告する。大規模な認証・CORS 設計変更は対象外とする。

## 着手前の実装との関係

- 着手前の `server` は `load_config("local")` と `DB_FILE_NAME` を固定使用し、鍵と Cookie の判定に `debug_assertions` を使っていた。
- `get_sqlite_pool` は `connect_lazy` を使うため、プール生成だけでは実接続を確認できない。
- `DB_FILE_NAME` のコード上の参照は着手前には `server/src/main.rs` のみ。バッチは各自の DB 引数を `get_sqlite_pool` に渡している。
- マイグレーションは `resources/db/migrations/` にある。既存 DB は `resources/db/honey_note.db` に存在する。
