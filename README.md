# safer-ffi-example

[![Rust CI](https://github.com/skanehira/safer-ffi-example/actions/workflows/rust-ci.yml/badge.svg)](https://github.com/skanehira/safer-ffi-example/actions/workflows/rust-ci.yml)
[![セキュリティ監査](https://github.com/skanehira/safer-ffi-example/actions/workflows/security-audit.yml/badge.svg)](https://github.com/skanehira/safer-ffi-example/actions/workflows/security-audit.yml)

RustのFFI（Foreign Function
Interface）を安全に実装するための[safer-ffi](https://github.com/getditto/safer_ffi)クレートを使用した実装例です。このプロジェクトでは、簡単なTodoアプリケーションのFFIバインディングを作成しています。

## 概要

このプロジェクトは以下の特徴を持ちます：

- Rustで実装されたTodoアプリケーションのコア機能
- [safer-ffi](https://github.com/getditto/safer_ffi)を使用した安全なFFIバインディング
- GoからRustの関数を呼び出すサンプル実装

## 機能

- Todoアイテムの作成と管理
- アプリケーションインスタンスの作成と破棄
- Todoの追加
- Todoの数、ID、内容の取得

## 必要環境

- Rust 1.85.0
- Go 1.24.0（Go側のサンプルを試す場合）
- C/C++コンパイラ（ヘッダー生成時に必要）

## インストール

```bash
# リポジトリのクローン
git clone https://github.com/skanehira/safer-ffi-example.git
cd safer-ffi-example

# Rustの依存関係のインストール
cargo build
```

## ヘッダーファイル生成

C/Go言語からの利用に必要なヘッダーファイルを生成するには：

```bash
make headers
```

## Go言語からの利用例

```go
package main

/*
#cgo LDFLAGS: -L../target/debug -lsafer_ffi_example
#include <stdlib.h>
#include "safer_ffi_example.h"
*/
import "C"
import (
	"fmt"
	"unsafe"
)

// Todoは単一のタスク項目を表します
type Todo struct {
	ID   int32
	Note string
}

// Appはラッパー構造体
type App struct {
	ptr *C.App_t
}

// ...

func main() {
	// 新しいAppインスタンスを作成
	app := NewApp()
	defer app.Free() // 関数終了時にメモリを解放

	// いくつかのTodoを追加
	app.AddTodo(1, "牛乳を買う")
	app.AddTodo(2, "レポートを書く")
	app.AddTodo(3, "友達に電話する")

	// Todoの数を取得
	count := app.GetTodoCount()
	fmt.Printf("Todo数: %d\n", count)

	// すべてのTodoを表示
	for i := 0; i < count; i++ {
		todo := app.GetTodoAt(i)
		if todo != nil {
			fmt.Printf("Todo[%d]: ID=%d, Note=%s\n", i, todo.ID, todo.Note)
		}
	}
}
```

## APIドキュメント

Rustのドキュメントを生成して閲覧するには：

```bash
cargo doc --open
```

## ライセンス

MIT

## 参考資料

- [safer-ffi GitHub](https://github.com/getditto/safer_ffi)
- [safer-ffi ドキュメント](https://docs.rs/safer-ffi/latest/safer_ffi/)
