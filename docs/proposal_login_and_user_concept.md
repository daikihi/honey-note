### ログイン機能およびユーザー概念の追加提案

HoneyNote にユーザー概念を導入し、ログイン機能を実装するための開発計画を提案します。

---

### 1. ユーザー概念の定義 (User Model)

まず、システムに「誰が利用しているか」を管理するための `users` テーブルを追加します。

#### データベース定義 (Migration)
`resources/db/migrations/` に新しいマイグレーションファイルを追加します。

```sql
CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL,                        -- ログインID
    email_hash TEXT NOT NULL,                      -- 暗号化されたメールアドレス（必須）
    password_hash TEXT NOT NULL,                   -- ハッシュ化されたパスワード
    display_name TEXT,                             -- 表示名
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP, -- 登録日時
    terminated_at DATETIME,                        -- 論理削除用（退会日時）
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP, -- 更新日時
    UNIQUE(username),
    UNIQUE(email_hash)
);
```

> **セキュリティ強化のため、メールアドレスも暗号化して保存します。**
> **username, email_hash にはユニーク制約を付与します。退会は terminated_at で管理し、物理削除は行いません（unitemporal設計）。**

---

#### セッションテーブル定義

`resources/db/migrations/` にセッション管理用テーブルを追加します。

```sql
CREATE TABLE sessions (
    id TEXT PRIMARY KEY,                -- セッションID
    session_data BLOB NOT NULL,         -- セッション情報（シリアライズ済み）
    expires_at DATETIME,                -- 有効期限
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

> **セッションは SQLite 上の sessions テーブルで管理します。**

---

### 2. 認証方式の選定 (Authentication)

Rust (Actix-web) での実装において、以下の構成を提案します。

- **パスワードハッシュ化**: `bcrypt` を使用し、ユーザーのパスワードは常にハッシュ化して保存します。
- **メールアドレスのハッシュ化・復号化**: `cryptography` クレートを利用し、メールアドレスは暗号化して保存します（必要に応じて復号化可能）。
    - メールアドレスの重複チェックは、暗号化前の値を一時的に復号化して比較する、または暗号化値で一意性を担保する設計とします。
- **セッション管理**: `actix-session` を使用
    - 初期実装では、ユーザーデータと同じ SQLite データベース内にセッション情報を保持します（`actix-session-sqlx` 等を利用）。
    - 将来的にユーザー数が増加し、パフォーマンスやスケーラビリティが課題となった場合は、セッション専用のデータベース（Redis や別の SQLite インスタンス等）への分離を検討します。

---

### 3. 追加が必要な API エンドポイント

`server/src/controllers/` に `auth_controller.rs` を新設します。

- `POST /api/auth/signup`: ユーザー新規登録
- `POST /api/auth/login`: ログイン（セッション発行）
- `POST /api/auth/logout`: ログアウト（セッション破棄）
- `GET /api/auth/me`: 現在のログインユーザー情報取得

---

### 4. フロントエンド (WASM) の対応方針

`front/src/` において、以下の変更を行います。

- **ログインページの新設**: `server/src/assets/html/login.html` を作成。
- **認証状態の保持**:
    - 各ページ読み込み時に `GET /api/auth/me` を呼び出し、未ログインならログインページへリダイレクトする処理を共通化（`commons` 等に配置）。
- **ナビゲーションの変更**: ログイン時はユーザー名を表示し、ログアウトボタンを表示するように `common.css` と各 HTML のヘッダーを調整。

---

### 5. 開発ロードマップ (案)

1.  **Phase 1: ユーザー基盤の作成**
    - DBマイグレーション（`users` テーブル作成）。
    - ユーザー登録・ログイン用のサーバーサイドロジック実装。
2.  **Phase 2: ログイン画面の実装**
    - HTML/CSS/WASM によるログインフォームの作成。
    - ログイン成功後のリダイレクト処理。
3.  **Phase 3: 認可（アクセス制限）の導入**
    - ミドルウェアによる API 保護（未ログイン時の 401 返却）。
    - 各 HTML ページでのログインチェックの実装。
4.  **Phase 4: データのユーザー紐付け**
    - `honey`、`beekeeper`、`flower` などの既存テーブルに `user_id` を追加し、自分のデータのみ表示・編集できるように変更。
    - 既存API・Repository・UseCase・Controller で「user_id = ログインユーザー」のみを返すように修正。
    - 新規登録・編集時は、必ずログインユーザーの `user_id` を付与して保存するように修正。
    - 既存の `get_all_*` 系APIは「user_id = ログインユーザー」のみ返すようにSQLやRepository層でWHERE句を追加。
    - 既存の `insert_*`/`update_*` 系APIは、必ず `user_id` を付与して保存するようにController/UseCase層で修正。
    - 既存の `honey` だけでなく、`beekeeper`、`flower` も同様に対応。
    - 既存データの移行については、管理者ユーザー等に一括でuser_idを付与するバッチを用意。

---

### 操作ログへのユーザー情報付与

- 各Controllerで出力している操作ログに「どのユーザーが操作したか（user_id, username等）」を必ず記録するように修正。
    - 例: `info!("user_id={}, username={}, action=insert_honey, ...")` のような形式で出力。
    - これにより、後からDBを復元する際に「誰がどのデータを操作したか」を正確に追跡可能。

---

### バッチ処理のユーザー情報対応

- 既存のバッチ処理（例: データ復元や一括登録等）は、User情報を持たずにデータを復元してしまう問題がある。
- 今後は、バッチ処理でも「どのユーザーのデータとして復元するか」を明示的に指定できるように設計・実装を修正。
    - 例: バッチ起動時に `--user-id` オプションを必須にし、そのユーザーIDで全データを登録する。
    - もしくは、バッチ用の管理ユーザーを用意し、そのユーザーIDで一括登録する。
- これにより、バッチ経由で登録されたデータも「誰のものか」が明確になり、通常のAPI経由と同じくユーザー単位でのデータ管理・復元が可能となる。

---

### 6. アーキテクチャ・API設計方針

#### 採用アーキテクチャ

本プロジェクトは「クリーンアーキテクチャ／DDD（ドメイン駆動設計）」の思想をベースに、以下のレイヤ構成で実装されています。

- **Controller層**（server/src/controllers/）
  - Actix-webのエンドポイント定義。リクエスト/レスポンスのDTO変換、認証・認可、ログ出力などを担当。
  - 各APIごとにController関数を用意し、UseCase層を呼び出す。
- **UseCase層**（server/src/use_case/）
  - アプリケーション固有の業務ロジックを実装。Repositoryを介して永続化層とやりとり。
  - 各APIごとにDTO（Data Transfer Object）を定義し、Controller層とやりとり。
- **Repository層**（common/src/repository/）
  - DBアクセスの抽象化。traitでインターフェースを定義し、Sqlite等の実装を分離。
  - テスト用のMock実装も用意。
- **Entity/Model層**（common-type/src/models/）
  - ドメインモデルやDTO、リクエスト/レスポンス型を定義。
- **Batch層**（batchs/）
  - マスターデータや初期データの一括登録・復元用バッチ。UseCase/Repository/Modelを再利用。

#### API設計・コーディング規約

- **DTO設計**  
  - Controller層とUseCase層の間はDTOでやりとりし、ドメインモデルやDBモデルと明確に分離。
  - リクエスト/レスポンスDTOは`server/src/use_case/xxx_dto.rs`等に定義。
- **認証・認可**  
  - 認証済みユーザーのuser_idをAPIごとに必ず取得し、データの登録・取得・編集時にuser_idを付与・参照。
  - Controller層で認証情報を抽出し、DTOやUseCaseに渡す。
- **ログ出力**  
  - すべてのControllerで「誰が（user_id, username等）」「何を（action, resource, id等）」操作したかをinfoレベルで記録。
  - 例: `info!("user_id={}, username={}, action=put_new_honey, ...")`
- **バッチ処理**  
  - バッチもAPIと同じくuser_idを必須とし、どのユーザーのデータか明確に管理。
  - バッチ起動時に`--user-id`等のオプションで指定、または管理ユーザーで一括登録。

#### 今後の修正・拡張ポイント

- `honey`/`beekeeper`/`flower`等の全テーブルにuser_idを追加し、API/Repository/UseCase/Batchでuser_idによる絞り込み・付与を徹底。
- 既存API（get_all_*, insert_*, update_*等）は「user_id=ログインユーザー」のみを返す/保存するように修正。
- 操作ログには必ずuser_id, username等を記録し、後からDB復元・監査ができるようにする。
- バッチ処理もuser_id必須とし、API経由と同じくユーザー単位でのデータ管理・復元を担保。
