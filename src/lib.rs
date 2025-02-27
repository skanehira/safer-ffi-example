use safer_ffi::prelude::*;

// Todoアイテムの構造体定義
#[derive_ReprC]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Todo {
    id: i32,
    note: char_p::Box,
}

impl Todo {
    pub fn new(id: i32, note: &str) -> Self {
        let c_string = std::ffi::CString::new(note).unwrap();
        Self {
            id,
            note: char_p::Box::from(c_string),
        }
    }
}

// アプリケーション構造体
#[derive_ReprC]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct App {
    todos: repr_c::Vec<Todo>,
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

// Todoを追加する関数
#[ffi_export]
fn add_todo(app: &mut App, id: i32, note: char_p::Ref<'_>) -> bool {
    // 文字列をRustの文字列に変換
    let note_str = note.to_str();

    // Todo構造体を作成
    let todo = Todo::new(id, note_str);

    // repr_c::Vec から std::vec::Vec に変換
    let mut native_vec: Vec<Todo> = app.todos.iter().cloned().collect();

    // 値を追加
    native_vec.push(todo);

    // 再び repr_c::Vec に変換して設定
    app.todos = native_vec.into();

    true
}

// Todoの数を取得する関数
#[ffi_export]
fn get_todo_count(app: &App) -> usize {
    app.todos.len()
}

// 指定インデックスのTodoのIDを取得する関数
#[ffi_export]
fn get_todo_id_at(app: &App, index: usize) -> i32 {
    if index < app.todos.len() {
        app.todos[index].id
    } else {
        -1 // エラー値
    }
}

// 指定インデックスのTodoのノートを取得する関数
#[ffi_export]
fn get_todo_note_at(app: &App, index: usize) -> char_p::Box {
    if index < app.todos.len() {
        // 文字列をコピーして返す
        let note_str = app.todos[index].note.to_str();
        let c_string = std::ffi::CString::new(note_str).unwrap();
        char_p::Box::from(c_string)
    } else {
        // エラーの場合は空文字列
        let c_string = std::ffi::CString::new("").unwrap();
        char_p::Box::from(c_string)
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CStr;

    // テスト用にchar_p::Refを作成するヘルパー関数
    // CStringとchar_p::Refの両方を返す理由:
    // 1. char_p::RefはCStringが持つヒープメモリ上の文字列データへのポインタを保持している
    // 2. CStringがスコープを抜けてdropされると、そのヒープメモリは解放される
    // 3. その後にchar_p::Refを使うと解放済みメモリ参照(use-after-free)となり、未定義動作を引き起こす
    // 4. タプルで両方を返すことで、呼び出し側がCStringの寿命を管理でき、ダングリングポインタを防止できる
    // 5. 呼び出し側は `let _ = cstring;` などでCStringを保持し、参照が必要な間メモリが解放されないようにする
    //
    // メモリ構造の図解:
    //
    // CString オブジェクト        ヒープ上の文字列データ
    // +------------------+       +----------------+
    // | ポインタ   --------+-----> | 'こ', 'ん', ... |
    // +------------------+       +----------------+
    //                                 ↑
    // char_p::Ref                     |
    // +------------------+            |
    // | ポインタ  ---------+------------+
    // +------------------+
    //
    // このパターンはテスト用に簡略化していますが、実際のアプリケーションではもっと体系的な
    // 文字列ライフタイム管理方法（例：Arc<CString>など）の検討が必要かもしれません
    fn c_str(s: &str) -> (std::ffi::CString, char_p::Ref<'_>) {
        let cstring = std::ffi::CString::new(s).unwrap();
        let cstr = unsafe { CStr::from_ptr(cstring.as_ptr()) };
        let char_ref = char_p::Ref::from(cstr);
        (cstring, char_ref) // CStringを一緒に返して、ライフタイムを延長
    }

    #[test]
    fn test_todo_new() {
        let todo = Todo::new(42, "テストタスク");
        assert_eq!(todo.id, 42);
        assert_eq!(todo.note.to_str(), "テストタスク");
    }

    #[test]
    fn test_app_new() {
        let app = App::default();
        assert_eq!(app.todos.len(), 0);
    }

    #[test]
    fn test_add_todo() {
        let mut app = App::default();

        // Todoを追加
        let (cstring1, note_ref1) = c_str("タスク1");
        let result = add_todo(&mut app, 1, note_ref1);
        assert!(result);
        assert_eq!(app.todos.len(), 1);

        // 2つ目のTodoを追加
        let (cstring2, note_ref2) = c_str("タスク2");
        add_todo(&mut app, 2, note_ref2);

        assert_eq!(app.todos.len(), 2);
        assert_eq!(app.todos[0].id, 1);
        assert_eq!(app.todos[0].note.to_str(), "タスク1");
        assert_eq!(app.todos[1].id, 2);
        assert_eq!(app.todos[1].note.to_str(), "タスク2");

        // CStringを変数に保持して、関数を抜けるまで生存期間を保証
        let _ = (cstring1, cstring2);
    }

    #[test]
    fn test_get_todo_count() {
        let mut app = App::default();
        assert_eq!(get_todo_count(&app), 0);

        // Todoを追加
        let (cstring, note_ref) = c_str("テスト");
        add_todo(&mut app, 1, note_ref);

        assert_eq!(get_todo_count(&app), 1);

        // CStringを変数に保持
        let _ = cstring;
    }

    #[test]
    fn test_get_todo_id_at() {
        let mut app = App::default();

        // 範囲外のインデックスにアクセス
        assert_eq!(get_todo_id_at(&app, 0), -1);

        // Todoを追加
        let (cstring, note_ref) = c_str("テスト");
        add_todo(&mut app, 42, note_ref);

        assert_eq!(get_todo_id_at(&app, 0), 42);
        // 範囲外のインデックスにアクセス
        assert_eq!(get_todo_id_at(&app, 1), -1);

        // CStringを変数に保持
        let _ = cstring;
    }

    #[test]
    fn test_get_todo_note_at() {
        let mut app = App::default();

        // 範囲外のインデックスにアクセス
        let empty_note = get_todo_note_at(&app, 0);
        assert_eq!(empty_note.to_str(), "");

        // Todoを追加
        let (cstring, note_ref) = c_str("重要なタスク");
        add_todo(&mut app, 1, note_ref);

        let retrieved_note = get_todo_note_at(&app, 0);
        assert_eq!(retrieved_note.to_str(), "重要なタスク");

        // 範囲外のインデックスにアクセス
        let empty_note2 = get_todo_note_at(&app, 1);
        assert_eq!(empty_note2.to_str(), "");

        // CStringを変数に保持
        let _ = cstring;
    }
}
