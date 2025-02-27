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

// NewAppはApp_tのインスタンスを作成します
func NewApp() *App {
	return &App{
		ptr: C.app_new(),
	}
}

// AddTodoはTodoリストに新しいTodoを追加します
func (a *App) AddTodo(id int32, note string) bool {
	cNote := C.CString(note)
	defer C.free(unsafe.Pointer(cNote))

	return bool(C.add_todo(a.ptr, C.int32_t(id), cNote))
}

// GetTodoCountはTodoの数を返します
func (a *App) GetTodoCount() int {
	return int(C.get_todo_count(a.ptr))
}

// GetTodoAtは指定されたインデックスのTodoを返します
func (a *App) GetTodoAt(index int) *Todo {
	if index >= a.GetTodoCount() {
		return nil
	}

	id := int32(C.get_todo_id_at(a.ptr, C.size_t(index)))

	// get_todo_note_atはメモリを確保して返すので、Goで解放する必要があります
	cNote := C.get_todo_note_at(a.ptr, C.size_t(index))
	note := C.GoString(cNote)
	C.free(unsafe.Pointer(cNote))

	return &Todo{
		ID:   id,
		Note: note,
	}
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
