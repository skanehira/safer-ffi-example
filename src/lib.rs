use safer_ffi::prelude::*;

// 最もシンプルな解決策
#[derive_ReprC]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct App {
    todos: repr_c::Vec<i32>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            todos: Vec::new().into(),
        }
    }
}

#[ffi_export]
fn app_new() -> repr_c::Box<App> {
    Box::new(App::default()).into()
}

#[ffi_export]
fn add_todo(app: &mut App, value: i32) {
    // repr_c::Vec から std::vec::Vec に変換
    let mut native_vec: Vec<i32> = app.todos.iter().copied().collect();

    // 値を追加
    native_vec.push(value);

    // 再び repr_c::Vec に変換して設定
    app.todos = native_vec.into();
}

// 追加のユーティリティ関数
#[ffi_export]
fn get_todo_count(app: &App) -> usize {
    app.todos.len()
}

#[ffi_export]
fn get_todo_at(app: &App, index: usize) -> i32 {
    if index < app.todos.len() {
        app.todos[index]
    } else {
        -1 // エラー値
    }
}

// アプリケーションのメモリを解放するための関数
#[ffi_export]
fn app_free(_app: repr_c::Box<App>) {
    // repr_c::Box はドロップ時に自動的にメモリを解放します
    // この関数内で何もする必要はありません
    // app は関数終了時に自動的にドロップされます
}

#[cfg(feature = "headers")]
pub fn generate_headers() -> ::std::io::Result<()> {
    ::safer_ffi::headers::builder()
        .to_file("./go_sample/safer_ffi_example.h")?
        .generate()
}
