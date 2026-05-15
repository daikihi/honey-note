# ユーザー概念・認証仕様

このドキュメントは、現行ブランチのコードを基準にしたユーザー概念、認証方式、セッション構造をまとめたものである。

## ユーザー概念

- 本システムの利用主体を `user` とする
- `username` はログイン識別子であり、小文字化して保存・照合する
- `email` は平文保存せず、ハッシュ値で重複確認に使う
- `password` は bcrypt でハッシュ化して保存する
- `display_name` は画面に表示する名前として使う
- `terminated_at` が入っているユーザーは有効ユーザーとして扱わない

## DB 仕様

### `users` テーブル

- `id`
- `username`
- `email_hash`
- `password_hash`
- `display_name`
- `created_at`
- `terminated_at`
- `updated_at`

### 制約

- `username` は UNIQUE
- `email_hash` は UNIQUE
- `updated_at` は UPDATE トリガーで自動更新される

### ユーザー単位のデータ管理

- `honey`
- `beekeeper`
- `flower`

これらのテーブルには `user_id` が追加されており、ログインユーザーごとにデータを分けて扱う。

## セッション仕様

- ログイン状態はセッション cookie `honey_note_session` で管理する
- セッションには `SessionData` を保存する
- `SessionData` は次の項目を持つ
  - `version`
  - `user_id`
  - `username`

## 認証フロー

### 新規登録

- `POST /api/auth/signup`
- `username`, `email`, `password`, `display_name` を受け取る
- バリデーションに通らない場合は失敗する

### ログイン

- `POST /api/auth/login`
- `username`, `password` を受け取る
- 認証成功時にセッションを発行する

### ログアウト

- `POST /api/auth/logout`
- セッションを破棄する

### 現在のログイン状態

- `GET /api/auth/me`
- ログイン中かどうか、`user_id` と `username` を返す

## 認証と認可

- 認証が必要な API は `AuthenticatedUser` extractor を使う
- セッションがなければ `401 Unauthorized` になる
- 養蜂家、蜜源、はちみつの詳細取得や更新は所有者確認を行う
