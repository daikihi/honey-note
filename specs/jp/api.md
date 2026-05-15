# API仕様書

このドキュメントは、現行ブランチのコードを基準にした API 仕様をまとめたものである。  
ここでは実装されている振る舞いを整理し、エンドポイント、認証要否、主要な入出力を確認できるようにする。

## 共通事項

- ベースURLは `/honey-note/api` である
- 認証が必要な API はセッションからログインユーザーを取得する
- 未ログインの場合は `401 Unauthorized` になる
- PUT 系のリクエストは JSON 本文を受け取り、本文ログも残す
- レスポンスは主に JSON で返す

## 認証関連

### `POST /api/auth/signup`

- 新規ユーザーを登録する
- リクエスト
  - `username`
  - `email`
  - `password`
  - `display_name`
- 振る舞い
  - `username` は小文字化して保存する
  - `email` は SHA-256 でハッシュ化して重複確認に使う
  - `password` は bcrypt でハッシュ化する
  - `display_name` がなければ `username` を表示名に使う
- 主なレスポンス
  - `200 OK`
  - `400 Bad Request`
  - `500 Internal Server Error`

### `POST /api/auth/login`

- ログインしてセッションを発行する
- リクエスト
  - `username`
  - `password`
- 振る舞い
  - `username` は小文字化して照合する
  - 認証成功時は `SessionData` を `user` キーで保存する
- 主なレスポンス
  - `200 OK`
  - `401 Unauthorized`

### `POST /api/auth/logout`

- セッションを破棄する
- 主なレスポンス
  - `200 OK`

### `GET /api/auth/me`

- 現在のログイン状態を返す
- 主なレスポンス
  - `200 OK`

## マスターデータ

### `GET /honey-note/api/prefectures`

- 都道府県一覧を取得する
- 認証は不要

## 養蜂家

### `GET /honey-note/api/beekeepers`

- ログインユーザーに紐付く養蜂家一覧を返す

### `GET /honey-note/api/beekeeper/{id}`

- 指定 ID の養蜂家詳細を返す
- 所有権がない場合は見つからない扱いになる

### `PUT /honey-note/api/beekeeper/new`

- 養蜂家を新規登録する
- リクエスト
  - `Beekeeper`

### `PUT /honey-note/api/beekeeper/edit/{id}`

- 指定 ID の養蜂家を更新する
- リクエスト
  - `Beekeeper`

## 蜜源

### `GET /honey-note/api/flowers`

- ログインユーザーに紐付く蜜源一覧を返す

### `GET /honey-note/api/flower/{id}`

- 指定 ID の蜜源詳細を返す
- 所有権がない場合は見つからない扱いになる

### `PUT /honey-note/api/flower/new`

- 蜜源を新規登録する
- リクエスト
  - `Flower`

### `PUT /honey-note/api/flower/edit/{id}`

- 指定 ID の蜜源を更新する
- リクエスト
  - `Flower`

## はちみつ

### `GET /honey-note/api/honeys`

- ログインユーザーに紐付くはちみつ一覧を返す

### `GET /honey-note/api/honey/{id}`

- 指定 ID のはちみつ詳細を返す
- 所有権がない場合は見つからない扱いになる

### `PUT /honey-note/api/honey/new`

- はちみつを新規登録する
- リクエスト
  - `HoneyNewRequest`
  - `basic`
  - `dynamic`
  - `created_at`
- 振る舞い
  - 養蜂家名が未登録なら作成して関連付ける
  - 花名が未登録なら作成して関連付ける

### `PUT /honey-note/api/honey/edit`

- 指定 ID のはちみつを更新する
- リクエスト
  - `HoneyEditRequest`
  - `id`
  - `basic`
  - `dynamic`
  - `updated_at`

## その他

### `GET /health`

- サーバーの生存確認を返す
