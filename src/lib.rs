use safer_ffi::prelude::*;

/// Todoアイテムを表す構造体
///
/// FFIを通じてC/Go言語からも利用可能な形式で、Todo項目のデータを保持します。
///
/// # フィールド
///
/// * `id` - Todo項目の一意識別子
/// * `note` - Todo項目の内容を表す文字列（FFI互換のchar_p::Box型）
///
/// # 使用例
///
/// ```rust
/// use safer_ffi_example::Todo;
///
/// // 新しいTodoアイテムを作成
/// let todo = Todo::new(1, "牛乳を買う");
/// assert_eq!(todo.id, 1);
/// assert_eq!(todo.note.to_str(), "牛乳を買う");
/// ```
#[derive_ReprC]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Todo {
    pub id: i32,
    pub note: char_p::Box,
}

impl Todo {
    /// 新しいTodoアイテムを作成します
    ///
    /// # 引数
    ///
    /// * `id` - Todo項目の一意識別子
    /// * `note` - Todo項目の内容を表す文字列
    ///
    /// # 戻り値
    ///
    /// 初期化されたTodo構造体のインスタンス
    ///
    /// # 使用例
    ///
    /// ```rust
    /// use safer_ffi_example::Todo;
    ///
    /// let todo = Todo::new(42, "重要なタスク");
    /// ```
    pub fn new(id: i32, note: &str) -> Self {
        let c_string = std::ffi::CString::new(note).unwrap();
        Self {
            id,
            note: char_p::Box::from(c_string),
        }
    }
}

/// Todoアプリケーションの状態を管理する構造体
///
/// 複数のTodoアイテムを管理し、FFIを通じてC/Go言語からも利用可能です。
///
/// # フィールド
///
/// * `todos` - Todo項目のコレクション（FFI互換のrepr_c::Vec型）
///
/// # 使用例
///
/// ```rust
/// use safer_ffi_example::{App, add_todo};
/// use safer_ffi::prelude::*;
///
/// // 空のAppインスタンスを作成
/// let mut app = App::default();
/// assert_eq!(app.todos.len(), 0);
///
/// // CStringを作成してchar_p::Refに変換
/// let note = std::ffi::CString::new("牛乳を買う").unwrap();
/// let note_ref = char_p::Ref::from(note.as_ref());
///
/// // Todoを追加
/// add_todo(&mut app, 1, note_ref);
/// ```
#[derive_ReprC]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct App {
    pub todos: repr_c::Vec<Todo>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            todos: Vec::new().into(),
        }
    }
}

/// 新しいAppインスタンスを作成します
///
/// # 戻り値
///
/// 空のTodoリストを持つAppのインスタンスをFFI互換のBoxでラップして返します。
///
/// # 使用例
///
/// ## Rust
///
/// ```rust
/// use safer_ffi_example::app_new;
///
/// let app = app_new();
/// ```
///
/// ## Go
///
/// ```go
/// import "example.com/todo"
///
/// func main() {
///     app := todo.AppNew()
///     defer todo.AppFree(app)
///     // アプリを使用...
/// }
/// ```
#[ffi_export]
pub fn app_new() -> repr_c::Box<App> {
    Box::new(App::default()).into()
}

/// Todoをアプリケーションに追加します
///
/// # 引数
///
/// * `app` - Todoを追加するアプリケーションインスタンスへの可変参照
/// * `id` - 追加するTodoの一意識別子
/// * `note` - Todoの内容を表す文字列（FFI互換のchar_p::Ref型）
///
/// # 戻り値
///
/// 追加が成功した場合は`true`、失敗した場合は`false`を返します。
///
/// # 使用例
///
/// ## Rust
///
/// ```rust
/// use safer_ffi_example::{App, add_todo};
/// use safer_ffi::prelude::*;
/// use std::ffi::CString;
///
/// let mut app = App::default();
/// let note = CString::new("重要なタスク").unwrap();
/// let note_ref = char_p::Ref::from(note.as_ref());
///
/// let success = add_todo(&mut app, 1, note_ref);
/// assert!(success);
/// ```
///
/// ## Go
///
/// ```go
/// import "example.com/todo"
///
/// func main() {
///     app := todo.AppNew()
///     defer todo.AppFree(app)
///
///     todo.AddTodo(app, 1, "重要なタスク")
/// }
/// ```
#[ffi_export]
pub fn add_todo(app: &mut App, id: i32, note: char_p::Ref<'_>) -> bool {
    // 文字列をRustの文字列に変換
    let note_str = note.to_str();

    // Todo構造体を作成
    let todo = Todo::new(id, note_str);

    // repr_c::Vec から std::vec::Vec に変換
    // Note: FFI互換のrepr_c::Vecから標準のVecに変換して操作する必要がある
    let mut native_vec: Vec<Todo> = app.todos.iter().cloned().collect();

    // 値を追加
    native_vec.push(todo);

    // 再び repr_c::Vec に変換して設定
    app.todos = native_vec.into();

    true
}

/// アプリケーション内のTodoの数を取得します
///
/// # 引数
///
/// * `app` - Todoアプリケーションインスタンスへの参照
///
/// # 戻り値
///
/// アプリケーション内のTodoアイテムの数
///
/// # 使用例
///
/// ## Rust
///
/// ```rust
/// use safer_ffi_example::{App, add_todo, get_todo_count};
/// use safer_ffi::prelude::*;
/// use std::ffi::CString;
///
/// let mut app = App::default();
/// assert_eq!(get_todo_count(&app), 0);
///
/// // Todoを追加
/// let note = CString::new("タスク").unwrap();
/// let note_ref = char_p::Ref::from(note.as_ref());
/// add_todo(&mut app, 1, note_ref);
///
/// assert_eq!(get_todo_count(&app), 1);
/// ```
///
/// ## Go
///
/// ```go
/// import (
///     "example.com/todo"
///     "fmt"
/// )
///
/// func main() {
///     app := todo.AppNew()
///     defer todo.AppFree(app)
///
///     todo.AddTodo(app, 1, "重要なタスク")
///     count := todo.GetTodoCount(app)
///     fmt.Printf("Todo数: %d\n", count)
/// }
/// ```
#[ffi_export]
pub fn get_todo_count(app: &App) -> usize {
    app.todos.len()
}

/// 指定インデックスのTodoのIDを取得します
///
/// # 引数
///
/// * `app` - Todoアプリケーションインスタンスへの参照
/// * `index` - 取得するTodoのインデックス（0から始まる）
///
/// # 戻り値
///
/// 成功した場合はTodoのID、インデックスが範囲外の場合は-1を返します
///
/// # 使用例
///
/// ## Rust
///
/// ```rust
/// use safer_ffi_example::{App, add_todo, get_todo_id_at};
/// use safer_ffi::prelude::*;
/// use std::ffi::CString;
///
/// let mut app = App::default();
///
/// // インデックスが範囲外の場合は-1を返す
/// assert_eq!(get_todo_id_at(&app, 0), -1);
///
/// // Todoを追加
/// let note = CString::new("タスク").unwrap();
/// let note_ref = char_p::Ref::from(note.as_ref());
/// add_todo(&mut app, 42, note_ref);
///
/// // 追加したTodoのIDを取得
/// assert_eq!(get_todo_id_at(&app, 0), 42);
/// ```
///
/// ## Go
///
/// ```go
/// import (
///     "example.com/todo"
///     "fmt"
/// )
///
/// func main() {
///     app := todo.AppNew()
///     defer todo.AppFree(app)
///
///     todo.AddTodo(app, 42, "重要なタスク")
///     id := todo.GetTodoIdAt(app, 0)
///     fmt.Printf("最初のTodoのID: %d\n", id)
/// }
/// ```
#[ffi_export]
pub fn get_todo_id_at(app: &App, index: usize) -> i32 {
    if index < app.todos.len() {
        app.todos[index].id
    } else {
        -1 // エラー値
    }
}

/// 指定インデックスのTodoのノート（内容）を取得します
///
/// # 引数
///
/// * `app` - Todoアプリケーションインスタンスへの参照
/// * `index` - 取得するTodoのインデックス（0から始まる）
///
/// # 戻り値
///
/// 成功した場合はTodoのノート、インデックスが範囲外の場合は空文字列を返します
///
/// # 使用例
///
/// ## Rust
///
/// ```rust
/// use safer_ffi_example::{App, add_todo, get_todo_note_at};
/// use safer_ffi::prelude::*;
/// use std::ffi::CString;
///
/// let mut app = App::default();
///
/// // インデックスが範囲外の場合は空文字列を返す
/// let empty = get_todo_note_at(&app, 0);
/// assert_eq!(empty.to_str(), "");
///
/// // Todoを追加
/// let note = CString::new("重要なタスク").unwrap();
/// let note_ref = char_p::Ref::from(note.as_ref());
/// add_todo(&mut app, 1, note_ref);
///
/// // 追加したTodoのノートを取得
/// let retrieved = get_todo_note_at(&app, 0);
/// assert_eq!(retrieved.to_str(), "重要なタスク");
/// ```
///
/// ## Go
///
/// ```go
/// import (
///     "example.com/todo"
///     "fmt"
/// )
///
/// func main() {
///     app := todo.AppNew()
///     defer todo.AppFree(app)
///
///     todo.AddTodo(app, 1, "買い物リスト")
///     note := todo.GetTodoNoteAt(app, 0)
///     fmt.Printf("Todo内容: %s\n", note)
/// }
/// ```
#[ffi_export]
pub fn get_todo_note_at(app: &App, index: usize) -> char_p::Box {
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

/// アプリケーションのメモリを解放します
///
/// この関数を呼び出すことで、アプリケーションが使用していたメモリリソースが
/// 適切に解放されます。Go言語からの利用時には、defer文を使用して確実に呼び出すことが推奨されます。
///
/// # 引数
///
/// * `_app` - 解放するアプリケーションインスタンス
///
/// # 注意
///
/// この関数内では特別な処理は行われず、Rustの所有権システムによって自動的にメモリが解放されます。
/// repr_c::Box はドロップ時に自動的にメモリを解放します。
///
/// # 使用例
///
/// ## Go
///
/// ```go
/// import "example.com/todo"
///
/// func main() {
///     app := todo.AppNew()
///     defer todo.AppFree(app) // 確実にメモリを解放
///
///     // アプリの操作...
/// }
/// ```
#[ffi_export]
pub fn app_free(_app: repr_c::Box<App>) {
    // repr_c::Box はドロップ時に自動的にメモリを解放します
    // この関数内で何もする必要はありません
    // app は関数終了時に自動的にドロップされます
}

/// FFIヘッダーファイルを生成します
///
/// このプロジェクトのRust関数とデータ構造をC/C++/Go等から利用するための
/// ヘッダーファイルを生成します。ビルド時に`headers`機能が有効な場合のみ利用可能です。
///
/// # 戻り値
///
/// ヘッダーファイルの生成結果を表すResult
///
/// # 使用例
///
/// ```rust,no_run
/// #[cfg(feature = "headers")]
/// fn main() -> std::io::Result<()> {
///     safer_ffi_example::generate_headers()
/// }
///
/// #[cfg(not(feature = "headers"))]
/// fn main() {
///     println!("headers機能が有効ではありません");
/// }
/// ```
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
