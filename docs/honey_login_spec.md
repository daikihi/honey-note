
# HoneyNote ログイン機能仕様書（改訂版）

実装は、最低限で行ってください。また、勝手に考えた機能は増やさないでください。
レビューも有るのでコメントはこまめに入れてください。
疑問があれば、開発をストップして、一旦オペレータと議論しましょう。

以下は、前回の仕様書に追記内容を反映した最新版です。

---

## 1. ユーザー概念の定義 (User Model)

### users テーブル
HoneyNote にユーザー概念を導入し、ログイン機能を実現するため `users` テーブルを追加します。

```sql
CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL,                        -- ログインID（lowercase に正規化して保存）
    email_hash TEXT NOT NULL,                      -- ハッシュ化したメールアドレス（比較専用）
    password_hash TEXT NOT NULL,                   -- ハッシュ化パスワード
    display_name TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    terminated_at DATETIME,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(username),
    UNIQUE(email_hash)
);
```

### ✔ username は lowercase に統一
ユーザーの混乱を防ぎ、比較の安定性を担保するため、サーバ側で常に lowercase へ正規化して保存します。

### ✔ email_hash の方針
HoneyNote ではメール送信機能を持たないため、**メールアドレスは平文保存せず、ハッシュのみ保存します。**

- ハッシュは比較専用
- 平文メールは保持しないため漏洩リスクが低い
- パスワードと同じ扱いで安全

### ✔ メールアドレス再設定の方針
メールアドレスの再設定・変更は **username を用いて本人確認** して行います。
（メール送信による本人確認を行わないため。）

---

## 2. updated_at の自動更新（SQLite の仕様対策）
SQLite は `DEFAULT CURRENT_TIMESTAMP` だけでは **UPDATE 時に更新されません**。
そのため、自動更新のためにトリガーを定義します。

```sql
CREATE TRIGGER users_updated_at
AFTER UPDATE ON users
FOR EACH ROW
BEGIN
    UPDATE users SET updated_at = CURRENT_TIMESTAMP WHERE id = OLD.id;
END;
```

これにより、`UPDATE users ...` 実行後に自動で `updated_at` が新しくなります。

---

## 3. セッション設計

### sessions テーブル
セッション情報は **アプリ本体とは別の SQLite データベースファイル** に保存します。

```sql
CREATE TABLE sessions (
    id TEXT PRIMARY KEY,
    session_data BLOB NOT NULL,   -- バージョン管理を含める
    expires_at DATETIME,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

### ✔ session_data にバージョンを持たせる
将来の構造変更に備え、セッション JSON は以下のようにバージョン番号を持たせる。

```json
{
  "version": 1,
  "user_id": 1,
  "username": "daiki"
}
```

---

## 4. 認証方式（Actix Web ミドルウェアベース）

### ✔ 認証処理はミドルウェアで行う
Controller が user_id を毎回取得する設計は冗長なため、以下をミドルウェアで統合します。

- セッション読み込み
- user_id / username 抽出
- ユーザーが存在しない場合は 401 を返す

Controller では `AuthenticatedUser` を受け取るだけにしてシンプルにします。

### ✔ 認証エラー型の共通化
API 全体で `AuthError` を共通化します：

```rust
enum AuthError {
    UserNotFound,
    InvalidPassword,
    SessionExpired,
    Unauthorized,
}
```

---

## 5. ログインログの記録

ログイン成功・失敗時には必ずログを出力します。

- 成功
```
info!("login success username={}, ip={}, ua={}");
```

- 失敗
```
warn!("login failed username={}, ip={}, ua={}");
```

これによりセキュリティ監査が可能になります。

---

## 6. 未ログイン時のリダイレクト（フロント/WASM）

すべての HTML/WASM ページはロード時に `GET /api/auth/me` を実行し、
未ログインであれば：

```
window.location.assign("/login.html");
```

を実行します。これを共通関数にまとめます。

---

## 7. Rate Limit の必須化

Actix Web の rate limit ミドルウェアを使い：

- `/api/auth/login` は特に厳しく制限
- DoS／ブルートフォース耐性を強化

---

## 8. API Endpoints
- `POST /api/auth/signup`
- `POST /api/auth/login`
- `POST /api/auth/logout`
- `GET /api/auth/me`

---

## 9. Domain Model と DBModel の分離

### ✔ Domain Model（Entity）
- 役割：DDD の Domain Model
- 配置：`common-type/src/models/`

### ✔ DBModel（SQLx 用構造体）
- 役割：DB 行をマッピングするだけ
- 配置：`common/src/infrastructure/db/sqlx/`

この2層を明確に分けることで保守性を向上させます。

---

## 10. データのユーザー紐付け

既存テーブル（honey, beekeeper, flower など）には `user_id` を追加し：

- 自分のデータのみ表示
- 登録・編集時は `user_id` を付与

---

## 11. 操作ログへのユーザー情報付与

例：
```
info!("user_id={}, username={}, action=insert_honey, honey_id={}");
```

---

## 12. バッチ処理のユーザー対応

- `--user-id` を必須にする
- すべての登録データに user_id を付与

---
