# ユーザー概念・認証仕様

このドキュメントは、現行ブランチのコードを基準にしたユーザー概念、認証方式、セッション構造をまとめたものである。

## ユーザー概念

| 項目 | 内容 |
| :--- | :--- |
| 利用主体 | システムを利用する `user` |
| `username` | ログイン識別子。小文字化して保存・照合する |
| `email` | 平文保存せず、ハッシュ値で重複確認に使う |
| `password` | bcrypt でハッシュ化して保存する |
| `display_name` | 画面に表示する名前として使う |
| `terminated_at` | 有効ユーザーとして扱わない目印 |

## DB 仕様

### `users` テーブル

| カラム | 内容 |
| :--- | :--- |
| `id` | 主キー |
| `username` | 一意なログイン識別子 |
| `email_hash` | 一意なハッシュ値 |
| `password_hash` | bcrypt ハッシュ |
| `display_name` | 表示名 |
| `created_at` | 登録日時 |
| `terminated_at` | 退会日時 |
| `updated_at` | 更新日時 |

### 制約

| 制約 | 内容 |
| :--- | :--- |
| `username` UNIQUE | ユーザー名は一意である |
| `email_hash` UNIQUE | メールハッシュは一意である |
| `updated_at` トリガー | UPDATE 時に自動更新される |

### ユーザー単位のデータ管理

| テーブル | 内容 |
| :--- | :--- |
| `honey` | ユーザーごとのはちみつ |
| `beekeeper` | ユーザーごとの養蜂家 |
| `flower` | ユーザーごとの蜜源 |

これらのテーブルには `user_id` が追加されており、ログインユーザーごとにデータを分けて扱う。

## セッション仕様

| 項目 | 内容 |
| :--- | :--- |
| cookie 名 | `honey_note_session` |
| 保存対象 | `SessionData` |

### `SessionData`

| フィールド | 内容 |
| :--- | :--- |
| `version` | 将来互換のためのバージョン番号 |
| `user_id` | ログインユーザー ID |
| `username` | ログインユーザー名 |

## 認証フロー

| 手順 | エンドポイント | 概要 |
| :--- | :--- | :--- |
| 新規登録 | `POST /api/auth/signup` | `username`, `email`, `password`, `display_name` を受け取る。バリデーションや重複で失敗する |
| ログイン | `POST /api/auth/login` | `username`, `password` を受け取り、成功時にセッションを発行する |
| ログアウト | `POST /api/auth/logout` | セッションを破棄する |
| 現在のログイン状態 | `GET /api/auth/me` | 現在のログイン状態と識別子を返す |

## 認証と認可

| ルール | 内容 |
| :--- | :--- |
| 保護された API | `AuthenticatedUser` extractor を使う |
| セッションがない場合 | `401 Unauthorized` になる |
| 所有権確認 | 詳細取得や更新では所有権を確認する |
