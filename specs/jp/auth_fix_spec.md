# 仕様メモ: auth_controller の構文修正

このドキュメントは、`server/src/controllers/auth_controller.rs` の `login` 関数で発生していた構文不整合を整理した修正メモである。  
現行コードでは修正済みであり、ここでは何が問題だったかを記録する。

## 問題

- `login` 関数内に `if verified { ... }` と `match` の枝が混在していた
- `Ok(false) => ...` や `Err(e) => ...` が `if` の閉じ括弧の後ろに残っており、Rust の文法として不正だった

## 原因

- `verify_password` の結果を `?` で外しているため、`verified` は `bool` になる
- そのため `match` ではなく `if / else` で書くのが正しい

## 修正後の考え方

- パスワード検証が成功したらセッションを発行する
- 失敗したら `401 Unauthorized` を返す
- 例外的な分岐を `Ok(false)` / `Err(e)` で書かない

## メモ

- このファイルは設計変更ではなく修正記録として扱う
