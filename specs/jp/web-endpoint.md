# Webページ仕様書

このドキュメントでは、本プロジェクトで提供されるWebページのエンドポイント仕様について説明します。

ベースURL: `/honey-note`

## 画面一覧

### ハチミツ (Honey)

| パス | 説明 |
| :--- | :--- |
| `/honeys/lists.html` | 登録されているハチミツの一覧を表示します。 |
| `/honeys/show.html?id={id}` | 指定したIDのハチミツ詳細を表示します。 |
| `/honeys/new.html` | 新しいハチミツを登録する画面です。 |
| `/honeys/edit.html?id={id}` | 登録済みのハチミツ情報を編集する画面です。 |

### 蜜源植物 (Flower)

| パス | 説明 |
| :--- | :--- |
| `/flowers/lists.html` | 登録されている花の一覧を表示します。 |
| `/flowers/show.html?id={id}` | 指定したIDの花詳細を表示します。 |
| `/flowers/new.html` | 新しい花を登録する画面です。 |
| `/flowers/edit.html?id={id}` | 登録済みの花情報を編集する画面です。 |

### 養蜂業者 (Beekeeper)

| パス | 説明 |
| :--- | :--- |
| `/beekeepers/lists.html` | 登録されている養蜂業者の一覧を表示します。 |
| `/beekeepers/new.html` | 新しい養蜂業者を登録する画面です。 |
| `/beekeepers/edit.html?id={id}` | 登録済みの養蜂業者情報を編集する画面です。 |

## 静的資産 (Assets)

| パス | 説明 |
| :--- | :--- |
| `/javascript/` | WebAssemblyおよび関連するJavaScriptファイル |
| `/css/` | スタイルシート |
| `/icons/` | アイコン素材 |
