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
	expected := []int32{42, 100, 999}
	for _, val := range expected {
		app.AddTodo(val)
	}

	// 追加した件数が正しいことを確認
	count := app.GetTodoCount()
	if count != len(expected) {
		t.Errorf("期待したTodo数: %d, 実際: %d", len(expected), count)
	}
}

// TestGetTodo はTodoの取得機能をテストします
func TestGetTodo(t *testing.T) {
	app := NewApp()
	defer app.Free()

	// テストデータ
	expected := []int32{42, 100, 999}
	for _, val := range expected {
		app.AddTodo(val)
	}

	// 各インデックスでデータが正しく取得できることを確認
	for i, val := range expected {
		got := app.GetTodoAt(i)
		if got != val {
			t.Errorf("インデックス %d で期待した値: %d, 実際: %d", i, val, got)
		}
	}

	// インデックスが範囲外の場合は-1が返ることを確認
	outOfRange := app.GetTodoAt(len(expected))
	if outOfRange != -1 {
		t.Errorf("範囲外のインデックスでの期待値: -1, 実際: %d", outOfRange)
	}
}

func formatBytes(bytes int64) (float64, string) {
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
	// メモリ使用量の初期値を取得
	var m1, m2 runtime.MemStats
	runtime.ReadMemStats(&m1)

	// 大量のAppオブジェクトを作成して解放
	for range 1000 {
		app := NewApp()
		for j := range 10 {
			app.AddTodo(int32(j))
		}
		// 明示的に解放
		app.Free()
	}

	// 強制的にGCを実行
	runtime.GC()

	// メモリ使用量を再度測定
	runtime.ReadMemStats(&m2)

	// Rustオブジェクトのメモリリークがあればヒープ確保が大きく増加するはず
	amount, unit := formatBytes(int64(m1.TotalAlloc))
	t.Logf("初期ヒープ確保: %.2f%s", amount, unit)
	amount, unit = formatBytes(int64(m2.TotalAlloc))
	t.Logf("テスト後ヒープ確保: %.2f%s", amount, unit)

	// メモリ使用量が過度に増加していないことを確認
	// 注：この値はシステムによって異なる場合があるため、適切に調整してください
	const maxExpectedIncrease = 5 * 1024 * 1024 // 5MB以上の増加は疑わしい

	// メモリ使用量の差分を計算
	memIncrease := m2.TotalAlloc - m1.TotalAlloc

	amount, unit = formatBytes(int64(memIncrease))

	if memIncrease > maxExpectedIncrease {
		t.Errorf("メモリ使用量が過度に増加: %.2f%s", amount, unit)
	}
}
