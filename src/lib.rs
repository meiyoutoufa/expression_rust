mod expression;
mod stack;
use crate::expression::Expression;
use std::os::raw::c_char;
use std::ffi::{CStr, CString};

#[no_mangle]
pub extern "C" fn evaluate_expression(expr: *const c_char) -> bool {
    // 检查输入是否为 null，防止崩溃
    if expr.is_null() {
        return false;
    }

    // 将 C 字符串转换为 Rust 字符串
    let c_str = unsafe {
        CStr::from_ptr(expr)
    };
    let str_slice = c_str.to_str().unwrap_or(""); // 如果无法转换为有效的 UTF-8，则返回空字符串

    let mut exp = Expression::new();
    let out = exp.evaluate_expression(str_slice);
    out
}


#[no_mangle]
pub extern "C" fn infix_to_postfix(expr: *const c_char) -> *mut *mut c_char {
    // 检查输入是否为 null，防止崩溃
    if expr.is_null() {
        return std::ptr::null_mut();
    }

    // 将 C 字符串转换为 Rust 字符串
    let c_str = unsafe {
        CStr::from_ptr(expr)
    };
    let str_slice = c_str.to_str().unwrap_or(""); // 如果无法转换为有效的 UTF-8，则返回空字符串

    // 将中缀表达式转换为后缀表达式，假设返回一个 Vec<String>
    let exp = Expression::new().infix_to_postfix(str_slice);

    // 将 Rust Vec<String> 转换为 C 风格的字符串数组
    let mut result: Vec<*mut c_char> = exp.iter()
        .map(|s| CString::new(s.clone()).unwrap().into_raw())
        .collect();

    // 返回指向字符串数组的指针
    let ptr = result.as_mut_ptr();
    std::mem::forget(result); // 防止 Rust 释放内存

    ptr
}

#[no_mangle]
pub extern "C" fn free_postfix_result(result: *mut *mut c_char) {
    if result.is_null() {
        return;
    }

    // 假设传入的 result 是一个 C 风格的字符串数组
    let mut index = 0;
    unsafe {
        while !(*result.add(index)).is_null() {
            // 释放每个字符串
            CString::from_raw(*result.add(index));
            index += 1;
        }
        // 最后释放字符串数组本身
        Box::from_raw(result);
    }
}
