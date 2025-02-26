#![allow(deprecated)]
use safer_ffi::prelude::*;

// 外部から使用するための列挙型
#[derive_ReprC]
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Color {
    Red = 0,
    Green = 1,
    Blue = 2,
}

// 住所を表す構造体
#[derive_ReprC]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Address {
    city: repr_c::String,
    postal_code: repr_c::String,
}

impl Address {
    pub fn new(city: &str, postal_code: &str) -> Self {
        Self {
            city: city.into(),
            postal_code: postal_code.into(),
        }
    }
    
    pub fn get_city(&self) -> &str {
        &self.city
    }
    
    pub fn get_postal_code(&self) -> &str {
        &self.postal_code
    }
}

// 外部から使用するための構造体
#[derive_ReprC]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Person {
    favorite_color: Color,
    scores: repr_c::Vec<i32>,  // 整数値のベクトル
    address: Address,          // 埋め込み構造体
}

// Person構造体のメソッド
impl Person {
    // 新しいPersonインスタンスを作成
    pub fn new(favorite_color: Color, scores: Vec<i32>, address: Address) -> Self {
        Self { 
            favorite_color,
            scores: scores.into(),
            address,
        }
    }

    // お気に入りの色を取得
    pub fn get_favorite_color(&self) -> Color {
        self.favorite_color
    }
    
    // スコアの合計を取得
    pub fn get_total_score(&self) -> i32 {
        self.scores.iter().sum()
    }
    
    // スコアの数を取得
    pub fn get_score_count(&self) -> usize {
        self.scores.len()
    }
    
    // 指定インデックスのスコアを取得
    pub fn get_score_at(&self, index: usize) -> Option<i32> {
        if index < self.scores.len() {
            Some(self.scores[index])
        } else {
            None
        }
    }
    
    // 住所を取得
    pub fn get_address(&self) -> &Address {
        &self.address
    }
}

// FFIで公開する関数群

// 新しいAddressインスタンスを作成するファクトリ関数
#[ffi_export]
pub fn address_create(city: char_p::Ref<'_>, postal_code: char_p::Ref<'_>) -> *mut Address {
    let address = Box::new(Address::new(city.to_str(), postal_code.to_str()));
    Box::into_raw(address)
}

// Addressインスタンスの解放
#[ffi_export]
pub fn address_free(ptr: *mut Address) {
    if !ptr.is_null() {
        unsafe {
            let _ = Box::from_raw(ptr);
        }
    }
}

// 新しいPersonインスタンスを作成するファクトリ関数
#[ffi_export]
pub fn person_create_with_address(
    favorite_color: Color, 
    scores_ptr: *const i32,
    scores_len: usize,
    address: &Address
) -> *mut Person {
    let scores = unsafe {
        if scores_ptr.is_null() || scores_len == 0 {
            Vec::new()
        } else {
            std::slice::from_raw_parts(scores_ptr, scores_len)
                .to_vec()
        }
    };
    
    let person = Box::new(Person::new(favorite_color, scores, address.clone()));
    Box::into_raw(person)
}

// Personインスタンスの解放
#[ffi_export]
pub fn person_free(ptr: *mut Person) {
    if !ptr.is_null() {
        unsafe {
            let _ = Box::from_raw(ptr);
        }
    }
}

// Personのお気に入りの色を取得
#[ffi_export]
pub fn person_get_favorite_color(person: &Person) -> Color {
    person.get_favorite_color()
}

// スコアの合計を取得
#[ffi_export]
pub fn person_get_total_score(person: &Person) -> i32 {
    person.get_total_score()
}

// スコアの数を取得
#[ffi_export]
pub fn person_get_score_count(person: &Person) -> usize {
    person.get_score_count()
}

// 指定インデックスのスコアを取得
#[ffi_export]
pub fn person_get_score_at(person: &Person, index: usize) -> i32 {
    person.get_score_at(index).unwrap_or(-1)
}

// Personの住所を取得
#[ffi_export]
pub fn person_get_address(person: &Person) -> &Address {
    person.get_address()
}

// Addressの都市名を取得
#[ffi_export]
pub fn address_get_city(address: &Address) -> char_p::Box {
    let c_string = std::ffi::CString::new(address.get_city()).unwrap();
    char_p::Box::from(c_string)
}

// Addressの郵便番号を取得
#[ffi_export]
pub fn address_get_postal_code(address: &Address) -> char_p::Box {
    let c_string = std::ffi::CString::new(address.get_postal_code()).unwrap();
    char_p::Box::from(c_string)
}

// 文字列を受け取り、安全に処理して返す関数を追加して文字列処理をテスト
#[ffi_export]
pub fn process_string(input: char_p::Ref<'_>) -> char_p::Box {
    let input_str = input.to_str();
    let processed = format!("Processed: {}", input_str);
    let c_string = std::ffi::CString::new(processed).unwrap();
    char_p::Box::from(c_string)
}

// FFI用のモジュール定義（必須）
#[safer_ffi::cfg_headers]
#[test]
extern "Rust" {
    type Color;
    type Address;
    type Person;
    
    fn address_create(city: char_p::Ref<'_>, postal_code: char_p::Ref<'_>) -> *mut Address;
    fn address_free(ptr: *mut Address);
    fn person_create_with_address(
        favorite_color: Color, 
        scores_ptr: *const i32,
        scores_len: usize,
        address: &Address
    ) -> *mut Person;
    fn person_free(ptr: *mut Person);
    fn person_get_favorite_color(person: &Person) -> Color;
    fn person_get_total_score(person: &Person) -> i32;
    fn person_get_score_count(person: &Person) -> usize;
    fn person_get_score_at(person: &Person, index: usize) -> i32;
    fn person_get_address(person: &Person) -> &Address;
    fn address_get_city(address: &Address) -> char_p::Box;
    fn address_get_postal_code(address: &Address) -> char_p::Box;
    fn process_string(input: char_p::Ref<'_>) -> char_p::Box;
}
