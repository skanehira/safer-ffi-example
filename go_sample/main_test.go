package main

import (
	"runtime"
	"testing"
)

// TestAddTodo はTodoの追加機能をテストします
func TestAddTodo(t *testing.T) {
	app := NewApp()
	defer app.Free()

	// テスト用のデータを追加
	todos := []struct {
		id   int32
		note string
	}{
		{1, "タスク1"},
		{2, "タスク2"},
		{3, "タスク3"},
	}

	for _, td := range todos {
		if !app.AddTodo(td.id, td.note) {
			t.Errorf("Todoの追加に失敗: ID=%d, Note=%s", td.id, td.note)
		}
	}

	// 追加した件数が正しいことを確認
	count := app.GetTodoCount()
	if count != len(todos) {
		t.Errorf("期待したTodo数: %d, 実際: %d", len(todos), count)
	}
}

// TestGetTodo はTodoの取得機能をテストします
func TestGetTodo(t *testing.T) {
	app := NewApp()
	defer app.Free()

	// テストデータ
	todos := []struct {
		id   int32
		note string
	}{
		{1, "タスク1"},
		{2, "タスク2"},
		{3, "タスク3"},
	}

	for _, td := range todos {
		app.AddTodo(td.id, td.note)
	}

	// 各インデックスでデータが正しく取得できることを確認
	for i, expected := range todos {
		todo := app.GetTodoAt(i)
		if todo == nil {
			t.Errorf("インデックス %d のTodoがnilです", i)
			continue
		}

		if todo.ID != expected.id {
			t.Errorf("インデックス %d で期待したID: %d, 実際: %d", i, expected.id, todo.ID)
		}

		if todo.Note != expected.note {
			t.Errorf("インデックス %d で期待したNote: %s, 実際: %s", i, expected.note, todo.Note)
		}
	}

	// インデックスが範囲外の場合はnilが返ることを確認
	outOfRange := app.GetTodoAt(len(todos))
	if outOfRange != nil {
		t.Errorf("範囲外のインデックスでnilでない値が返された: %+v", outOfRange)
	}
}

func formatBytes(bytes uint64) (float64, string) {
	// 人間が読みやすい単位に変換
	var unit string
	var amount float64
	switch {
	case bytes >= 1024*1024*1024:
		unit = "GB"
		amount = float64(bytes) / (1024 * 1024 * 1024)
	case bytes >= 1024*1024:
		unit = "MB"
		amount = float64(bytes) / (1024 * 1024)
	case bytes >= 1024:
		unit = "KB"
		amount = float64(bytes) / 1024
	default:
		unit = "B"
		amount = float64(bytes)
	}
	return amount, unit
}

// TestMemoryLeak はメモリリークがないことを確認します
func TestMemoryLeak(t *testing.T) {
	runtime.GC()

	// メモリ使用量の初期値を取得
	var m1, m2 runtime.MemStats
	runtime.ReadMemStats(&m1)

	// 大量のAppオブジェクトを作成して解放
	for range 100 {
		app := NewApp()
		for j := range 100 {
			app.AddTodo(int32(j), "テストタスク")
			todo := app.GetTodoAt(j)
			_ = todo.ID
		}
		// 明示的に解放
		app.Free()
	}

	// 強制的にGCを実行
	runtime.GC()

	// メモリ使用量を再度測定
	runtime.ReadMemStats(&m2)

	// Rustオブジェクトのメモリリークがあればヒープ確保が大きく増加するはず
	amount1, unit1 := formatBytes(m1.Alloc)
	t.Logf("初期ヒープ使用量: %.2f%s", amount1, unit1)

	amount2, unit2 := formatBytes(m2.Alloc)
	t.Logf("テスト後ヒープ使用量: %.2f%s", amount2, unit2)

	// メモリ使用量の差分を計算
	memDiff := int64(m2.Alloc) - int64(m1.Alloc)
	amountDiff, unitDiff := formatBytes(uint64(abs(memDiff)))

	if memDiff >= 0 {
		t.Logf("メモリ増加量: %.2f%s", amountDiff, unitDiff)
	} else {
		t.Logf("メモリ減少量: %.2f%s", amountDiff, unitDiff)
	}

	// メモリ使用量が過度に増加していないことを確認
	// 注：この値はシステムによって異なる場合があるため、適切に調整してください
	const maxExpectedIncrease = 1 * 1024 * 1024 // 1MB以上の増加は疑わしい
	if memDiff > maxExpectedIncrease {
		t.Errorf("メモリ使用量が過度に増加: %.2f%s", amountDiff, unitDiff)
	}
}

// 絶対値を計算する関数
func abs(n int64) int64 {
	if n < 0 {
		return -n
	}
	return n
}
