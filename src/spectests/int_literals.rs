// Rust test file autogenerated with cargo build (build/spectests.rs).
// Please do NOT modify it by hand, as it will be reseted on next build.
// Test based on spectests/int_literals.wast
#![allow(
    warnings,
    dead_code
)]
use wabt::wat2wasm;

use crate::webassembly::{instantiate, compile, ImportObject, ResultObject, Instance, Export};
use super::_common::{
    spectest_importobject,
    NaNCheck,
};


// Line 1
fn create_module_1() -> ResultObject {
    let module_str = "(module
      (type (;0;) (func (result i32)))
      (type (;1;) (func (result i64)))
      (func (;0;) (type 0) (result i32)
        i32.const 195940365
        return)
      (func (;1;) (type 0) (result i32)
        i32.const -1
        return)
      (func (;2;) (type 0) (result i32)
        i32.const 2147483647
        return)
      (func (;3;) (type 0) (result i32)
        i32.const -2147483647
        return)
      (func (;4;) (type 0) (result i32)
        i32.const -2147483648
        return)
      (func (;5;) (type 0) (result i32)
        i32.const -2147483648
        return)
      (func (;6;) (type 0) (result i32)
        i32.const -2147483648
        i32.const 1
        i32.add
        return)
      (func (;7;) (type 0) (result i32)
        i32.const 0
        return)
      (func (;8;) (type 0) (result i32)
        i32.const 10
        return)
      (func (;9;) (type 0) (result i32)
        i32.const -1
        return)
      (func (;10;) (type 0) (result i32)
        i32.const 42
        return)
      (func (;11;) (type 1) (result i64)
        i64.const 913028331277281902
        return)
      (func (;12;) (type 1) (result i64)
        i64.const -1
        return)
      (func (;13;) (type 1) (result i64)
        i64.const 9223372036854775807
        return)
      (func (;14;) (type 1) (result i64)
        i64.const -9223372036854775807
        return)
      (func (;15;) (type 1) (result i64)
        i64.const -9223372036854775808
        return)
      (func (;16;) (type 1) (result i64)
        i64.const -9223372036854775808
        return)
      (func (;17;) (type 1) (result i64)
        i64.const -9223372036854775808
        i64.const 1
        i64.add
        return)
      (func (;18;) (type 1) (result i64)
        i64.const 0
        return)
      (func (;19;) (type 1) (result i64)
        i64.const 10
        return)
      (func (;20;) (type 1) (result i64)
        i64.const -1
        return)
      (func (;21;) (type 1) (result i64)
        i64.const 42
        return)
      (func (;22;) (type 0) (result i32)
        i32.const 1000000)
      (func (;23;) (type 0) (result i32)
        i32.const 1000)
      (func (;24;) (type 0) (result i32)
        i32.const 168755353)
      (func (;25;) (type 0) (result i32)
        i32.const 109071)
      (func (;26;) (type 1) (result i64)
        i64.const 1000000)
      (func (;27;) (type 1) (result i64)
        i64.const 1000)
      (func (;28;) (type 1) (result i64)
        i64.const 3078696982321561)
      (func (;29;) (type 1) (result i64)
        i64.const 109071)
      (export \"i32.test\" (func 0))
      (export \"i32.umax\" (func 1))
      (export \"i32.smax\" (func 2))
      (export \"i32.neg_smax\" (func 3))
      (export \"i32.smin\" (func 4))
      (export \"i32.alt_smin\" (func 5))
      (export \"i32.inc_smin\" (func 6))
      (export \"i32.neg_zero\" (func 7))
      (export \"i32.not_octal\" (func 8))
      (export \"i32.unsigned_decimal\" (func 9))
      (export \"i32.plus_sign\" (func 10))
      (export \"i64.test\" (func 11))
      (export \"i64.umax\" (func 12))
      (export \"i64.smax\" (func 13))
      (export \"i64.neg_smax\" (func 14))
      (export \"i64.smin\" (func 15))
      (export \"i64.alt_smin\" (func 16))
      (export \"i64.inc_smin\" (func 17))
      (export \"i64.neg_zero\" (func 18))
      (export \"i64.not_octal\" (func 19))
      (export \"i64.unsigned_decimal\" (func 20))
      (export \"i64.plus_sign\" (func 21))
      (export \"i32-dec-sep1\" (func 22))
      (export \"i32-dec-sep2\" (func 23))
      (export \"i32-hex-sep1\" (func 24))
      (export \"i32-hex-sep2\" (func 25))
      (export \"i64-dec-sep1\" (func 26))
      (export \"i64-dec-sep2\" (func 27))
      (export \"i64-hex-sep1\" (func 28))
      (export \"i64-hex-sep2\" (func 29)))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject(), None).expect("WASM can't be instantiated")
}

fn start_module_1(result_object: &ResultObject) {
    result_object.instance.start();
}

// Line 37
fn c1_l37_action_invoke(result_object: &ResultObject) {
    println!("Executing function {}", "c1_l37_action_invoke");
    let func_index = match result_object.module.info.exports.get("i32.test") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&Instance) -> i32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(&result_object.instance);
    assert_eq!(result, 195940365 as i32);
}

// Line 38
fn c2_l38_action_invoke(result_object: &ResultObject) {
    println!("Executing function {}", "c2_l38_action_invoke");
    let func_index = match result_object.module.info.exports.get("i32.umax") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&Instance) -> i32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(&result_object.instance);
    assert_eq!(result, -1 as i32);
}

// Line 39
fn c3_l39_action_invoke(result_object: &ResultObject) {
    println!("Executing function {}", "c3_l39_action_invoke");
    let func_index = match result_object.module.info.exports.get("i32.smax") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&Instance) -> i32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(&result_object.instance);
    assert_eq!(result, 2147483647 as i32);
}

// Line 40
fn c4_l40_action_invoke(result_object: &ResultObject) {
    println!("Executing function {}", "c4_l40_action_invoke");
    let func_index = match result_object.module.info.exports.get("i32.neg_smax") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&Instance) -> i32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(&result_object.instance);
    assert_eq!(result, -2147483647 as i32);
}

// Line 41
fn c5_l41_action_invoke(result_object: &ResultObject) {
    println!("Executing function {}", "c5_l41_action_invoke");
    let func_index = match result_object.module.info.exports.get("i32.smin") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&Instance) -> i32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(&result_object.instance);
    assert_eq!(result, -2147483648 as i32);
}

// Line 42
fn c6_l42_action_invoke(result_object: &ResultObject) {
    println!("Executing function {}", "c6_l42_action_invoke");
    let func_index = match result_object.module.info.exports.get("i32.alt_smin") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&Instance) -> i32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(&result_object.instance);
    assert_eq!(result, -2147483648 as i32);
}

// Line 43
fn c7_l43_action_invoke(result_object: &ResultObject) {
    println!("Executing function {}", "c7_l43_action_invoke");
    let func_index = match result_object.module.info.exports.get("i32.inc_smin") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&Instance) -> i32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(&result_object.instance);
    assert_eq!(result, -2147483647 as i32);
}

// Line 44
fn c8_l44_action_invoke(result_object: &ResultObject) {
    println!("Executing function {}", "c8_l44_action_invoke");
    let func_index = match result_object.module.info.exports.get("i32.neg_zero") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&Instance) -> i32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(&result_object.instance);
    assert_eq!(result, 0 as i32);
}

// Line 45
fn c9_l45_action_invoke(result_object: &ResultObject) {
    println!("Executing function {}", "c9_l45_action_invoke");
    let func_index = match result_object.module.info.exports.get("i32.not_octal") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&Instance) -> i32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(&result_object.instance);
    assert_eq!(result, 10 as i32);
}

// Line 46
fn c10_l46_action_invoke(result_object: &ResultObject) {
    println!("Executing function {}", "c10_l46_action_invoke");
    let func_index = match result_object.module.info.exports.get("i32.unsigned_decimal") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&Instance) -> i32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(&result_object.instance);
    assert_eq!(result, -1 as i32);
}

// Line 47
fn c11_l47_action_invoke(result_object: &ResultObject) {
    println!("Executing function {}", "c11_l47_action_invoke");
    let func_index = match result_object.module.info.exports.get("i32.plus_sign") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&Instance) -> i32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(&result_object.instance);
    assert_eq!(result, 42 as i32);
}

// Line 49
fn c12_l49_action_invoke(result_object: &ResultObject) {
    println!("Executing function {}", "c12_l49_action_invoke");
    let func_index = match result_object.module.info.exports.get("i64.test") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&Instance) -> i64 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(&result_object.instance);
    assert_eq!(result, 913028331277281902 as i64);
}

// Line 50
fn c13_l50_action_invoke(result_object: &ResultObject) {
    println!("Executing function {}", "c13_l50_action_invoke");
    let func_index = match result_object.module.info.exports.get("i64.umax") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&Instance) -> i64 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(&result_object.instance);
    assert_eq!(result, -1 as i64);
}

// Line 51
fn c14_l51_action_invoke(result_object: &ResultObject) {
    println!("Executing function {}", "c14_l51_action_invoke");
    let func_index = match result_object.module.info.exports.get("i64.smax") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&Instance) -> i64 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(&result_object.instance);
    assert_eq!(result, 9223372036854775807 as i64);
}

// Line 52
fn c15_l52_action_invoke(result_object: &ResultObject) {
    println!("Executing function {}", "c15_l52_action_invoke");
    let func_index = match result_object.module.info.exports.get("i64.neg_smax") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&Instance) -> i64 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(&result_object.instance);
    assert_eq!(result, -9223372036854775807 as i64);
}

// Line 53
fn c16_l53_action_invoke(result_object: &ResultObject) {
    println!("Executing function {}", "c16_l53_action_invoke");
    let func_index = match result_object.module.info.exports.get("i64.smin") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&Instance) -> i64 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(&result_object.instance);
    assert_eq!(result, -9223372036854775808 as i64);
}

// Line 54
fn c17_l54_action_invoke(result_object: &ResultObject) {
    println!("Executing function {}", "c17_l54_action_invoke");
    let func_index = match result_object.module.info.exports.get("i64.alt_smin") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&Instance) -> i64 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(&result_object.instance);
    assert_eq!(result, -9223372036854775808 as i64);
}

// Line 55
fn c18_l55_action_invoke(result_object: &ResultObject) {
    println!("Executing function {}", "c18_l55_action_invoke");
    let func_index = match result_object.module.info.exports.get("i64.inc_smin") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&Instance) -> i64 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(&result_object.instance);
    assert_eq!(result, -9223372036854775807 as i64);
}

// Line 56
fn c19_l56_action_invoke(result_object: &ResultObject) {
    println!("Executing function {}", "c19_l56_action_invoke");
    let func_index = match result_object.module.info.exports.get("i64.neg_zero") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&Instance) -> i64 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(&result_object.instance);
    assert_eq!(result, 0 as i64);
}

// Line 57
fn c20_l57_action_invoke(result_object: &ResultObject) {
    println!("Executing function {}", "c20_l57_action_invoke");
    let func_index = match result_object.module.info.exports.get("i64.not_octal") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&Instance) -> i64 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(&result_object.instance);
    assert_eq!(result, 10 as i64);
}

// Line 58
fn c21_l58_action_invoke(result_object: &ResultObject) {
    println!("Executing function {}", "c21_l58_action_invoke");
    let func_index = match result_object.module.info.exports.get("i64.unsigned_decimal") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&Instance) -> i64 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(&result_object.instance);
    assert_eq!(result, -1 as i64);
}

// Line 59
fn c22_l59_action_invoke(result_object: &ResultObject) {
    println!("Executing function {}", "c22_l59_action_invoke");
    let func_index = match result_object.module.info.exports.get("i64.plus_sign") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&Instance) -> i64 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(&result_object.instance);
    assert_eq!(result, 42 as i64);
}

// Line 61
fn c23_l61_action_invoke(result_object: &ResultObject) {
    println!("Executing function {}", "c23_l61_action_invoke");
    let func_index = match result_object.module.info.exports.get("i32-dec-sep1") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&Instance) -> i32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(&result_object.instance);
    assert_eq!(result, 1000000 as i32);
}

// Line 62
fn c24_l62_action_invoke(result_object: &ResultObject) {
    println!("Executing function {}", "c24_l62_action_invoke");
    let func_index = match result_object.module.info.exports.get("i32-dec-sep2") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&Instance) -> i32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(&result_object.instance);
    assert_eq!(result, 1000 as i32);
}

// Line 63
fn c25_l63_action_invoke(result_object: &ResultObject) {
    println!("Executing function {}", "c25_l63_action_invoke");
    let func_index = match result_object.module.info.exports.get("i32-hex-sep1") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&Instance) -> i32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(&result_object.instance);
    assert_eq!(result, 168755353 as i32);
}

// Line 64
fn c26_l64_action_invoke(result_object: &ResultObject) {
    println!("Executing function {}", "c26_l64_action_invoke");
    let func_index = match result_object.module.info.exports.get("i32-hex-sep2") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&Instance) -> i32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(&result_object.instance);
    assert_eq!(result, 109071 as i32);
}

// Line 66
fn c27_l66_action_invoke(result_object: &ResultObject) {
    println!("Executing function {}", "c27_l66_action_invoke");
    let func_index = match result_object.module.info.exports.get("i64-dec-sep1") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&Instance) -> i64 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(&result_object.instance);
    assert_eq!(result, 1000000 as i64);
}

// Line 67
fn c28_l67_action_invoke(result_object: &ResultObject) {
    println!("Executing function {}", "c28_l67_action_invoke");
    let func_index = match result_object.module.info.exports.get("i64-dec-sep2") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&Instance) -> i64 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(&result_object.instance);
    assert_eq!(result, 1000 as i64);
}

// Line 68
fn c29_l68_action_invoke(result_object: &ResultObject) {
    println!("Executing function {}", "c29_l68_action_invoke");
    let func_index = match result_object.module.info.exports.get("i64-hex-sep1") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&Instance) -> i64 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(&result_object.instance);
    assert_eq!(result, 3078696982321561 as i64);
}

// Line 69
fn c30_l69_action_invoke(result_object: &ResultObject) {
    println!("Executing function {}", "c30_l69_action_invoke");
    let func_index = match result_object.module.info.exports.get("i64-hex-sep2") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&Instance) -> i64 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(&result_object.instance);
    assert_eq!(result, 109071 as i64);
}

// Line 72
#[test]
fn c31_l72_assert_malformed() {
    let wasm_binary = [40, 103, 108, 111, 98, 97, 108, 32, 105, 51, 50, 32, 40, 105, 51, 50, 46, 99, 111, 110, 115, 116, 32, 95, 49, 48, 48, 41, 41];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 76
#[test]
fn c32_l76_assert_malformed() {
    let wasm_binary = [40, 103, 108, 111, 98, 97, 108, 32, 105, 51, 50, 32, 40, 105, 51, 50, 46, 99, 111, 110, 115, 116, 32, 43, 95, 49, 48, 48, 41, 41];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 80
#[test]
fn c33_l80_assert_malformed() {
    let wasm_binary = [40, 103, 108, 111, 98, 97, 108, 32, 105, 51, 50, 32, 40, 105, 51, 50, 46, 99, 111, 110, 115, 116, 32, 45, 95, 49, 48, 48, 41, 41];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 84
#[test]
fn c34_l84_assert_malformed() {
    let wasm_binary = [40, 103, 108, 111, 98, 97, 108, 32, 105, 51, 50, 32, 40, 105, 51, 50, 46, 99, 111, 110, 115, 116, 32, 57, 57, 95, 41, 41];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 88
#[test]
fn c35_l88_assert_malformed() {
    let wasm_binary = [40, 103, 108, 111, 98, 97, 108, 32, 105, 51, 50, 32, 40, 105, 51, 50, 46, 99, 111, 110, 115, 116, 32, 49, 95, 95, 48, 48, 48, 41, 41];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 92
#[test]
fn c36_l92_assert_malformed() {
    let wasm_binary = [40, 103, 108, 111, 98, 97, 108, 32, 105, 51, 50, 32, 40, 105, 51, 50, 46, 99, 111, 110, 115, 116, 32, 95, 48, 120, 49, 48, 48, 41, 41];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 96
#[test]
fn c37_l96_assert_malformed() {
    let wasm_binary = [40, 103, 108, 111, 98, 97, 108, 32, 105, 51, 50, 32, 40, 105, 51, 50, 46, 99, 111, 110, 115, 116, 32, 48, 95, 120, 49, 48, 48, 41, 41];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 100
#[test]
fn c38_l100_assert_malformed() {
    let wasm_binary = [40, 103, 108, 111, 98, 97, 108, 32, 105, 51, 50, 32, 40, 105, 51, 50, 46, 99, 111, 110, 115, 116, 32, 48, 120, 95, 49, 48, 48, 41, 41];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 104
#[test]
fn c39_l104_assert_malformed() {
    let wasm_binary = [40, 103, 108, 111, 98, 97, 108, 32, 105, 51, 50, 32, 40, 105, 51, 50, 46, 99, 111, 110, 115, 116, 32, 48, 120, 48, 48, 95, 41, 41];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 108
#[test]
fn c40_l108_assert_malformed() {
    let wasm_binary = [40, 103, 108, 111, 98, 97, 108, 32, 105, 51, 50, 32, 40, 105, 51, 50, 46, 99, 111, 110, 115, 116, 32, 48, 120, 102, 102, 95, 95, 102, 102, 102, 102, 41, 41];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 113
#[test]
fn c41_l113_assert_malformed() {
    let wasm_binary = [40, 103, 108, 111, 98, 97, 108, 32, 105, 54, 52, 32, 40, 105, 54, 52, 46, 99, 111, 110, 115, 116, 32, 95, 49, 48, 48, 41, 41];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 117
#[test]
fn c42_l117_assert_malformed() {
    let wasm_binary = [40, 103, 108, 111, 98, 97, 108, 32, 105, 54, 52, 32, 40, 105, 54, 52, 46, 99, 111, 110, 115, 116, 32, 43, 95, 49, 48, 48, 41, 41];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 121
#[test]
fn c43_l121_assert_malformed() {
    let wasm_binary = [40, 103, 108, 111, 98, 97, 108, 32, 105, 54, 52, 32, 40, 105, 54, 52, 46, 99, 111, 110, 115, 116, 32, 45, 95, 49, 48, 48, 41, 41];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 125
#[test]
fn c44_l125_assert_malformed() {
    let wasm_binary = [40, 103, 108, 111, 98, 97, 108, 32, 105, 54, 52, 32, 40, 105, 54, 52, 46, 99, 111, 110, 115, 116, 32, 57, 57, 95, 41, 41];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 129
#[test]
fn c45_l129_assert_malformed() {
    let wasm_binary = [40, 103, 108, 111, 98, 97, 108, 32, 105, 54, 52, 32, 40, 105, 54, 52, 46, 99, 111, 110, 115, 116, 32, 49, 95, 95, 48, 48, 48, 41, 41];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 133
#[test]
fn c46_l133_assert_malformed() {
    let wasm_binary = [40, 103, 108, 111, 98, 97, 108, 32, 105, 54, 52, 32, 40, 105, 54, 52, 46, 99, 111, 110, 115, 116, 32, 95, 48, 120, 49, 48, 48, 41, 41];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 137
#[test]
fn c47_l137_assert_malformed() {
    let wasm_binary = [40, 103, 108, 111, 98, 97, 108, 32, 105, 54, 52, 32, 40, 105, 54, 52, 46, 99, 111, 110, 115, 116, 32, 48, 95, 120, 49, 48, 48, 41, 41];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 141
#[test]
fn c48_l141_assert_malformed() {
    let wasm_binary = [40, 103, 108, 111, 98, 97, 108, 32, 105, 54, 52, 32, 40, 105, 54, 52, 46, 99, 111, 110, 115, 116, 32, 48, 120, 95, 49, 48, 48, 41, 41];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 145
#[test]
fn c49_l145_assert_malformed() {
    let wasm_binary = [40, 103, 108, 111, 98, 97, 108, 32, 105, 54, 52, 32, 40, 105, 54, 52, 46, 99, 111, 110, 115, 116, 32, 48, 120, 48, 48, 95, 41, 41];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 149
#[test]
fn c50_l149_assert_malformed() {
    let wasm_binary = [40, 103, 108, 111, 98, 97, 108, 32, 105, 54, 52, 32, 40, 105, 54, 52, 46, 99, 111, 110, 115, 116, 32, 48, 120, 102, 102, 95, 95, 102, 102, 102, 102, 41, 41];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

#[test]
fn test_module_1() {
    let result_object = create_module_1();
    // We group the calls together
    start_module_1(&result_object);
    c1_l37_action_invoke(&result_object);
    c2_l38_action_invoke(&result_object);
    c3_l39_action_invoke(&result_object);
    c4_l40_action_invoke(&result_object);
    c5_l41_action_invoke(&result_object);
    c6_l42_action_invoke(&result_object);
    c7_l43_action_invoke(&result_object);
    c8_l44_action_invoke(&result_object);
    c9_l45_action_invoke(&result_object);
    c10_l46_action_invoke(&result_object);
    c11_l47_action_invoke(&result_object);
    c12_l49_action_invoke(&result_object);
    c13_l50_action_invoke(&result_object);
    c14_l51_action_invoke(&result_object);
    c15_l52_action_invoke(&result_object);
    c16_l53_action_invoke(&result_object);
    c17_l54_action_invoke(&result_object);
    c18_l55_action_invoke(&result_object);
    c19_l56_action_invoke(&result_object);
    c20_l57_action_invoke(&result_object);
    c21_l58_action_invoke(&result_object);
    c22_l59_action_invoke(&result_object);
    c23_l61_action_invoke(&result_object);
    c24_l62_action_invoke(&result_object);
    c25_l63_action_invoke(&result_object);
    c26_l64_action_invoke(&result_object);
    c27_l66_action_invoke(&result_object);
    c28_l67_action_invoke(&result_object);
    c29_l68_action_invoke(&result_object);
    c30_l69_action_invoke(&result_object);
}
