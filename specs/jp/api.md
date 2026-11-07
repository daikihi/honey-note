# API仕様書

このドキュメントは、現行ブランチのコードを基準にした API 仕様をまとめたものである。  
エンドポイント、認証要否、主要な入出力を一覧しやすいように整理する。

## 共通事項

| 項目 | 内容 |
| :--- | :--- |
| ベースURL | 認証は `/api/auth`、その他のデータ API は `/honey-note/api` |
| 認証方式 | セッションからログインユーザーを取得する |
| 認証失敗時 | `401 Unauthorized` |
| PUT リクエスト | JSON 本文を受け取り、本文ログも残す |
| 主なレスポンス形式 | JSON |

## 認証関連

| メソッド | パス | 認証 | 主な入力 | 主な振る舞い | 主なレスポンス |
| :--- | :--- | :--- | :--- | :--- | :--- |
| POST | `/api/auth/signup` | 不要 | `username`, `email`, `password`, `display_name` | `username` を小文字化し、`email` はハッシュ化して重複確認する。`password` は bcrypt でハッシュ化する。`display_name` がなければ `username` を使う | `200 OK`, `400 Bad Request`, `500 Internal Server Error` |
| POST | `/api/auth/login` | 不要 | `username`, `password` | `username` を小文字化して照合し、成功時に `SessionData` を `user` キーで保存する | `200 OK`, `401 Unauthorized` |
| POST | `/api/auth/logout` | 不要 | なし | セッションを破棄する | `200 OK` |
| GET | `/api/auth/me` | 不要 | なし | ログイン状態、ログイン中なら `user_id`、`username`、`display_name` を返す。未ログインなら `logged_in: false` と各値 `null` | `200 OK`, `401 Unauthorized`（セッションのユーザーが存在しない場合）, `500 Internal Server Error` |
| PUT | `/api/auth/me/display-name` | 必要 | JSON: `display_name`（文字列）のみ | セッションのユーザーの表示名を前後空白除去後に更新する。空白のみ・余分なフィールドは拒否。同一オリジンの `Origin` ヘッダーを要求する | `200 OK`（`display_name`）, `400 Bad Request`, `401 Unauthorized`, `403 Forbidden`, `404 Not Found`, `500 Internal Server Error` |

登録・表示名保存の成功時は、復旧用に時刻、イベント名、ユーザーID、保存した表示名を構造化ログへ記録する。登録ログにはユーザー名も含める。

表示名更新時の同一オリジン判定は、`local` では HTTP、`prod` では HTTPS を公開側の通信方式として使用する。転送ヘッダーの通信方式は使用しない。

## マスターデータ

| メソッド | パス | 認証 | 主な入力 | 主な振る舞い | 主なレスポンス |
| :--- | :--- | :--- | :--- | :--- | :--- |
| GET | `/honey-note/api/prefectures` | 不要 | なし | 都道府県一覧を取得する | `200 OK`, `500 Internal Server Error` |

## 養蜂家

| メソッド | パス | 認証 | 主な入力 | 主な振る舞い | 主なレスポンス |
| :--- | :--- | :--- | :--- | :--- | :--- |
| GET | `/honey-note/api/beekeepers` | 必要 | なし | ログインユーザーに紐付く養蜂家一覧を返す | `200 OK`, `500 Internal Server Error` |
| GET | `/honey-note/api/beekeeper/{id}` | 必要 | `id` | 指定 ID の養蜂家詳細を返す。所有権がない場合は見つからない扱いになる | `200 OK`, `404 Not Found` |
| PUT | `/honey-note/api/beekeeper/new` | 必要 | `Beekeeper` | 養蜂家を新規登録する | `200 OK`, `400 Bad Request` |
| PUT | `/honey-note/api/beekeeper/edit/{id}` | 必要 | `id`, `Beekeeper` | 指定 ID の養蜂家を更新する | `200 OK`, `400 Bad Request` |

## 蜜源

| メソッド | パス | 認証 | 主な入力 | 主な振る舞い | 主なレスポンス |
| :--- | :--- | :--- | :--- | :--- | :--- |
| GET | `/honey-note/api/flowers` | 必要 | なし | ログインユーザーに紐付く蜜源一覧を返す | `200 OK`, `500 Internal Server Error` |
| GET | `/honey-note/api/flower/{id}` | 必要 | `id` | 指定 ID の蜜源詳細を返す。所有権がない場合は見つからない扱いになる | `200 OK`, `404 Not Found` |
| PUT | `/honey-note/api/flower/new` | 必要 | `Flower` | 蜜源を新規登録する | `200 OK`, `400 Bad Request` |
| PUT | `/honey-note/api/flower/edit/{id}` | 必要 | `id`, `Flower` | 指定 ID の蜜源を更新する | `200 OK`, `400 Bad Request` |

## はちみつ

| メソッド | パス | 認証 | 主な入力 | 主な振る舞い | 主なレスポンス |
| :--- | :--- | :--- | :--- | :--- | :--- |
| GET | `/honey-note/api/honeys` | 必要 | なし | ログインユーザーに紐付くはちみつ一覧を返す | `200 OK`, `500 Internal Server Error` |
| GET | `/honey-note/api/honey/{id}` | 必要 | `id` | 指定 ID のはちみつ詳細を返す。所有権がない場合は見つからない扱いになる | `200 OK`, `404 Not Found` |
| PUT | `/honey-note/api/honey/new` | 必要 | `HoneyNewRequest` | はちみつを新規登録する。養蜂家名や花名が未登録なら作成して関連付ける | `200 OK`, `400 Bad Request` |
| PUT | `/honey-note/api/honey/edit` | 必要 | `HoneyEditRequest` | 指定 ID のはちみつを更新する | `200 OK`, `400 Bad Request` |

## その他

| メソッド | パス | 認証 | 振る舞い | 主なレスポンス |
| :--- | :--- | :--- | :--- | :--- |
| GET | `/health` | 不要 | サーバーの生存確認を返す | `200 OK` |
