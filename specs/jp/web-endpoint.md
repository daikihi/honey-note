# Webページ仕様書

このドキュメントは、現行ブランチの `server/src/assets/html` を基準にした Web ページ仕様をまとめたものである。  
どの HTML が存在し、どの WASM 入口とつながっているかを確認できるようにする。

## 共通事項

- ベースURLは `/honey_note` である
- HTML は `server` から静的配信される
- 各ページは `front.js` を読み込み、対応する WASM 入口関数を呼ぶ
- 画面共通の見た目は `server/src/assets/css` で定義される

## トップ・認証

### `/index.html`

- トップページ
- `top_page_main()` を呼ぶ

### `/login.html`

- ログインフォーム
- `login_main()` を呼ぶ

### `/signup.html`

- 新規登録フォーム
- `signup_main()` を呼ぶ

## はちみつ

### `/honeys/lists.html`

- はちみつ一覧
- `honey_list_main()` を呼ぶ

### `/honeys/show.html?id={id}`

- はちみつ詳細
- `honey_show_main()` を呼ぶ

### `/honeys/new.html`

- はちみつ新規登録
- `honey_edit_and_new_main()` を呼ぶ

### `/honeys/edit.html?id={id}`

- はちみつ編集
- `honey_edit_and_new_main()` を呼ぶ

## 蜜源

### `/flowers/lists.html`

- 蜜源一覧
- `flower_list_main()` を呼ぶ

### `/flowers/show.html?id={id}`

- 蜜源詳細
- 静的な詳細ページとして配置されている

### `/flowers/new.html`

- 蜜源新規登録
- `flower_edit_and_new_main()` を呼ぶ

### `/flowers/edit.html?id={id}`

- 蜜源編集
- `flower_edit_and_new_main()` を呼ぶ

## 養蜂家

### `/beekeepers/lists.html`

- 養蜂家一覧
- `beekeepers_list_main()` を呼ぶ

### `/beekeepers/show.html?id={id}`

- 養蜂家詳細
- `beekeeper_show_main()` を呼ぶ

### `/beekeepers/new.html`

- 養蜂家新規登録
- `beekeeper_edit_and_new_main()` を呼ぶ

### `/beekeepers/edit.html?id={id}`

- 養蜂家編集
- `beekeeper_edit_and_new_main()` を呼ぶ

## 静的資産

- `/javascript/`
  - WebAssembly と関連 JavaScript
- `/css/`
  - スタイルシート
- `/icons/`
  - アイコン素材
