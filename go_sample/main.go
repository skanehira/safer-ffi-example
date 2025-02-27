package main

/*
#cgo LDFLAGS: -L../target/debug -lsafer_ffi_example
#include "safer_ffi_example.h"
*/
import "C"
import (
	"fmt"
)

// Appはラッパー構造体
type App struct {
	ptr *C.App_t
}

// NewAppはApp_tのインスタンスを作成します
func NewApp() *App {
	return &App{
		ptr: C.app_new(),
	}
}

// AddTodoはTodoリストに値を追加します
func (a *App) AddTodo(value int32) {
	C.add_todo(a.ptr, C.int32_t(value))
}

// GetTodoCountはTodoの数を返します
func (a *App) GetTodoCount() int {
	return int(C.get_todo_count(a.ptr))
}

// GetTodoAtは指定されたインデックスのTodoを返します
func (a *App) GetTodoAt(index int) int32 {
	return int32(C.get_todo_at(a.ptr, C.size_t(index)))
}

// Free はアプリケーションのメモリを解放します
func (a *App) Free() {
	C.app_free(a.ptr)
	a.ptr = nil // ダングリングポインタを防止
}

func main() {
	// 新しいAppインスタンスを作成
	app := NewApp()
	defer app.Free() // 関数終了時にメモリを解放

	// いくつかのTodoを追加
	app.AddTodo(10)
	app.AddTodo(20)
	app.AddTodo(30)

	// Todoの数を取得
	count := app.GetTodoCount()
	fmt.Printf("Todo数: %d\n", count)

	// すべてのTodoを表示
	for i := range count {
		value := app.GetTodoAt(i)
		fmt.Printf("Todo[%d]: %d\n", i, value)
	}

	// メモリリークを防ぐために、適切なメモリ管理が必要です
	// 現在の実装ではこれは含まれていません
	// デモンストレーションのための簡略化されたコードです
}
