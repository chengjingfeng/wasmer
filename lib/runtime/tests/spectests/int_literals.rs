// Rust test file autogenerated with cargo build (build/spectests.rs).
// Please do NOT modify it by hand, as it will be reset on next build.
// Test based on spectests/int_literals.wast
#![allow(
    warnings,
    dead_code
)]
use wabt::wat2wasm;
use std::{f32, f64};

use wasmer_runtime::types::Value;
use wasmer_runtime::{Instance, module::Module};
use wasmer_clif_backend::CraneliftCompiler;

use crate::spectests::_common::{
    generate_imports,
    NaNCheck,
};


// Line 1
fn create_module_1() -> Instance {
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
    let module = wasmer_runtime::compile(&wasm_binary[..], &CraneliftCompiler::new()).expect("WASM can't be compiled");
    module.instantiate(generate_imports()).expect("WASM can't be instantiated")
}

fn start_module_1(instance: &mut Instance) {
    // TODO Review is explicit start needed? Start now called in runtime::Instance::new()
    //instance.start();
}

// Line 37
fn c1_l37_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c1_l37_action_invoke");
    let result = instance.call("i32.test", &[]);
    assert_eq!(result, Ok(Some(Value::I32(195940365 as i32))));
    result.map(|_| ())
}

// Line 38
fn c2_l38_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c2_l38_action_invoke");
    let result = instance.call("i32.umax", &[]);
    assert_eq!(result, Ok(Some(Value::I32(-1 as i32))));
    result.map(|_| ())
}

// Line 39
fn c3_l39_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c3_l39_action_invoke");
    let result = instance.call("i32.smax", &[]);
    assert_eq!(result, Ok(Some(Value::I32(2147483647 as i32))));
    result.map(|_| ())
}

// Line 40
fn c4_l40_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c4_l40_action_invoke");
    let result = instance.call("i32.neg_smax", &[]);
    assert_eq!(result, Ok(Some(Value::I32(-2147483647 as i32))));
    result.map(|_| ())
}

// Line 41
fn c5_l41_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c5_l41_action_invoke");
    let result = instance.call("i32.smin", &[]);
    assert_eq!(result, Ok(Some(Value::I32(-2147483648 as i32))));
    result.map(|_| ())
}

// Line 42
fn c6_l42_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c6_l42_action_invoke");
    let result = instance.call("i32.alt_smin", &[]);
    assert_eq!(result, Ok(Some(Value::I32(-2147483648 as i32))));
    result.map(|_| ())
}

// Line 43
fn c7_l43_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c7_l43_action_invoke");
    let result = instance.call("i32.inc_smin", &[]);
    assert_eq!(result, Ok(Some(Value::I32(-2147483647 as i32))));
    result.map(|_| ())
}

// Line 44
fn c8_l44_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c8_l44_action_invoke");
    let result = instance.call("i32.neg_zero", &[]);
    assert_eq!(result, Ok(Some(Value::I32(0 as i32))));
    result.map(|_| ())
}

// Line 45
fn c9_l45_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c9_l45_action_invoke");
    let result = instance.call("i32.not_octal", &[]);
    assert_eq!(result, Ok(Some(Value::I32(10 as i32))));
    result.map(|_| ())
}

// Line 46
fn c10_l46_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c10_l46_action_invoke");
    let result = instance.call("i32.unsigned_decimal", &[]);
    assert_eq!(result, Ok(Some(Value::I32(-1 as i32))));
    result.map(|_| ())
}

// Line 47
fn c11_l47_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c11_l47_action_invoke");
    let result = instance.call("i32.plus_sign", &[]);
    assert_eq!(result, Ok(Some(Value::I32(42 as i32))));
    result.map(|_| ())
}

// Line 49
fn c12_l49_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c12_l49_action_invoke");
    let result = instance.call("i64.test", &[]);
    assert_eq!(result, Ok(Some(Value::I64(913028331277281902 as i64))));
    result.map(|_| ())
}

// Line 50
fn c13_l50_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c13_l50_action_invoke");
    let result = instance.call("i64.umax", &[]);
    assert_eq!(result, Ok(Some(Value::I64(-1 as i64))));
    result.map(|_| ())
}

// Line 51
fn c14_l51_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c14_l51_action_invoke");
    let result = instance.call("i64.smax", &[]);
    assert_eq!(result, Ok(Some(Value::I64(9223372036854775807 as i64))));
    result.map(|_| ())
}

// Line 52
fn c15_l52_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c15_l52_action_invoke");
    let result = instance.call("i64.neg_smax", &[]);
    assert_eq!(result, Ok(Some(Value::I64(-9223372036854775807 as i64))));
    result.map(|_| ())
}

// Line 53
fn c16_l53_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c16_l53_action_invoke");
    let result = instance.call("i64.smin", &[]);
    assert_eq!(result, Ok(Some(Value::I64(-9223372036854775808 as i64))));
    result.map(|_| ())
}

// Line 54
fn c17_l54_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c17_l54_action_invoke");
    let result = instance.call("i64.alt_smin", &[]);
    assert_eq!(result, Ok(Some(Value::I64(-9223372036854775808 as i64))));
    result.map(|_| ())
}

// Line 55
fn c18_l55_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c18_l55_action_invoke");
    let result = instance.call("i64.inc_smin", &[]);
    assert_eq!(result, Ok(Some(Value::I64(-9223372036854775807 as i64))));
    result.map(|_| ())
}

// Line 56
fn c19_l56_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c19_l56_action_invoke");
    let result = instance.call("i64.neg_zero", &[]);
    assert_eq!(result, Ok(Some(Value::I64(0 as i64))));
    result.map(|_| ())
}

// Line 57
fn c20_l57_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c20_l57_action_invoke");
    let result = instance.call("i64.not_octal", &[]);
    assert_eq!(result, Ok(Some(Value::I64(10 as i64))));
    result.map(|_| ())
}

// Line 58
fn c21_l58_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c21_l58_action_invoke");
    let result = instance.call("i64.unsigned_decimal", &[]);
    assert_eq!(result, Ok(Some(Value::I64(-1 as i64))));
    result.map(|_| ())
}

// Line 59
fn c22_l59_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c22_l59_action_invoke");
    let result = instance.call("i64.plus_sign", &[]);
    assert_eq!(result, Ok(Some(Value::I64(42 as i64))));
    result.map(|_| ())
}

// Line 61
fn c23_l61_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c23_l61_action_invoke");
    let result = instance.call("i32-dec-sep1", &[]);
    assert_eq!(result, Ok(Some(Value::I32(1000000 as i32))));
    result.map(|_| ())
}

// Line 62
fn c24_l62_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c24_l62_action_invoke");
    let result = instance.call("i32-dec-sep2", &[]);
    assert_eq!(result, Ok(Some(Value::I32(1000 as i32))));
    result.map(|_| ())
}

// Line 63
fn c25_l63_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c25_l63_action_invoke");
    let result = instance.call("i32-hex-sep1", &[]);
    assert_eq!(result, Ok(Some(Value::I32(168755353 as i32))));
    result.map(|_| ())
}

// Line 64
fn c26_l64_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c26_l64_action_invoke");
    let result = instance.call("i32-hex-sep2", &[]);
    assert_eq!(result, Ok(Some(Value::I32(109071 as i32))));
    result.map(|_| ())
}

// Line 66
fn c27_l66_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c27_l66_action_invoke");
    let result = instance.call("i64-dec-sep1", &[]);
    assert_eq!(result, Ok(Some(Value::I64(1000000 as i64))));
    result.map(|_| ())
}

// Line 67
fn c28_l67_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c28_l67_action_invoke");
    let result = instance.call("i64-dec-sep2", &[]);
    assert_eq!(result, Ok(Some(Value::I64(1000 as i64))));
    result.map(|_| ())
}

// Line 68
fn c29_l68_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c29_l68_action_invoke");
    let result = instance.call("i64-hex-sep1", &[]);
    assert_eq!(result, Ok(Some(Value::I64(3078696982321561 as i64))));
    result.map(|_| ())
}

// Line 69
fn c30_l69_action_invoke(instance: &mut Instance) -> Result<(), String> {
    println!("Executing function {}", "c30_l69_action_invoke");
    let result = instance.call("i64-hex-sep2", &[]);
    assert_eq!(result, Ok(Some(Value::I64(109071 as i64))));
    result.map(|_| ())
}

// Line 72
#[test]
fn c31_l72_assert_malformed() {
    let wasm_binary = [40, 103, 108, 111, 98, 97, 108, 32, 105, 51, 50, 32, 40, 105, 51, 50, 46, 99, 111, 110, 115, 116, 32, 95, 49, 48, 48, 41, 41];
    let compilation = wasmer_runtime::compile(&wasm_binary, &CraneliftCompiler::new());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 76
#[test]
fn c32_l76_assert_malformed() {
    let wasm_binary = [40, 103, 108, 111, 98, 97, 108, 32, 105, 51, 50, 32, 40, 105, 51, 50, 46, 99, 111, 110, 115, 116, 32, 43, 95, 49, 48, 48, 41, 41];
    let compilation = wasmer_runtime::compile(&wasm_binary, &CraneliftCompiler::new());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 80
#[test]
fn c33_l80_assert_malformed() {
    let wasm_binary = [40, 103, 108, 111, 98, 97, 108, 32, 105, 51, 50, 32, 40, 105, 51, 50, 46, 99, 111, 110, 115, 116, 32, 45, 95, 49, 48, 48, 41, 41];
    let compilation = wasmer_runtime::compile(&wasm_binary, &CraneliftCompiler::new());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 84
#[test]
fn c34_l84_assert_malformed() {
    let wasm_binary = [40, 103, 108, 111, 98, 97, 108, 32, 105, 51, 50, 32, 40, 105, 51, 50, 46, 99, 111, 110, 115, 116, 32, 57, 57, 95, 41, 41];
    let compilation = wasmer_runtime::compile(&wasm_binary, &CraneliftCompiler::new());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 88
#[test]
fn c35_l88_assert_malformed() {
    let wasm_binary = [40, 103, 108, 111, 98, 97, 108, 32, 105, 51, 50, 32, 40, 105, 51, 50, 46, 99, 111, 110, 115, 116, 32, 49, 95, 95, 48, 48, 48, 41, 41];
    let compilation = wasmer_runtime::compile(&wasm_binary, &CraneliftCompiler::new());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 92
#[test]
fn c36_l92_assert_malformed() {
    let wasm_binary = [40, 103, 108, 111, 98, 97, 108, 32, 105, 51, 50, 32, 40, 105, 51, 50, 46, 99, 111, 110, 115, 116, 32, 95, 48, 120, 49, 48, 48, 41, 41];
    let compilation = wasmer_runtime::compile(&wasm_binary, &CraneliftCompiler::new());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 96
#[test]
fn c37_l96_assert_malformed() {
    let wasm_binary = [40, 103, 108, 111, 98, 97, 108, 32, 105, 51, 50, 32, 40, 105, 51, 50, 46, 99, 111, 110, 115, 116, 32, 48, 95, 120, 49, 48, 48, 41, 41];
    let compilation = wasmer_runtime::compile(&wasm_binary, &CraneliftCompiler::new());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 100
#[test]
fn c38_l100_assert_malformed() {
    let wasm_binary = [40, 103, 108, 111, 98, 97, 108, 32, 105, 51, 50, 32, 40, 105, 51, 50, 46, 99, 111, 110, 115, 116, 32, 48, 120, 95, 49, 48, 48, 41, 41];
    let compilation = wasmer_runtime::compile(&wasm_binary, &CraneliftCompiler::new());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 104
#[test]
fn c39_l104_assert_malformed() {
    let wasm_binary = [40, 103, 108, 111, 98, 97, 108, 32, 105, 51, 50, 32, 40, 105, 51, 50, 46, 99, 111, 110, 115, 116, 32, 48, 120, 48, 48, 95, 41, 41];
    let compilation = wasmer_runtime::compile(&wasm_binary, &CraneliftCompiler::new());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 108
#[test]
fn c40_l108_assert_malformed() {
    let wasm_binary = [40, 103, 108, 111, 98, 97, 108, 32, 105, 51, 50, 32, 40, 105, 51, 50, 46, 99, 111, 110, 115, 116, 32, 48, 120, 102, 102, 95, 95, 102, 102, 102, 102, 41, 41];
    let compilation = wasmer_runtime::compile(&wasm_binary, &CraneliftCompiler::new());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 113
#[test]
fn c41_l113_assert_malformed() {
    let wasm_binary = [40, 103, 108, 111, 98, 97, 108, 32, 105, 54, 52, 32, 40, 105, 54, 52, 46, 99, 111, 110, 115, 116, 32, 95, 49, 48, 48, 41, 41];
    let compilation = wasmer_runtime::compile(&wasm_binary, &CraneliftCompiler::new());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 117
#[test]
fn c42_l117_assert_malformed() {
    let wasm_binary = [40, 103, 108, 111, 98, 97, 108, 32, 105, 54, 52, 32, 40, 105, 54, 52, 46, 99, 111, 110, 115, 116, 32, 43, 95, 49, 48, 48, 41, 41];
    let compilation = wasmer_runtime::compile(&wasm_binary, &CraneliftCompiler::new());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 121
#[test]
fn c43_l121_assert_malformed() {
    let wasm_binary = [40, 103, 108, 111, 98, 97, 108, 32, 105, 54, 52, 32, 40, 105, 54, 52, 46, 99, 111, 110, 115, 116, 32, 45, 95, 49, 48, 48, 41, 41];
    let compilation = wasmer_runtime::compile(&wasm_binary, &CraneliftCompiler::new());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 125
#[test]
fn c44_l125_assert_malformed() {
    let wasm_binary = [40, 103, 108, 111, 98, 97, 108, 32, 105, 54, 52, 32, 40, 105, 54, 52, 46, 99, 111, 110, 115, 116, 32, 57, 57, 95, 41, 41];
    let compilation = wasmer_runtime::compile(&wasm_binary, &CraneliftCompiler::new());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 129
#[test]
fn c45_l129_assert_malformed() {
    let wasm_binary = [40, 103, 108, 111, 98, 97, 108, 32, 105, 54, 52, 32, 40, 105, 54, 52, 46, 99, 111, 110, 115, 116, 32, 49, 95, 95, 48, 48, 48, 41, 41];
    let compilation = wasmer_runtime::compile(&wasm_binary, &CraneliftCompiler::new());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 133
#[test]
fn c46_l133_assert_malformed() {
    let wasm_binary = [40, 103, 108, 111, 98, 97, 108, 32, 105, 54, 52, 32, 40, 105, 54, 52, 46, 99, 111, 110, 115, 116, 32, 95, 48, 120, 49, 48, 48, 41, 41];
    let compilation = wasmer_runtime::compile(&wasm_binary, &CraneliftCompiler::new());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 137
#[test]
fn c47_l137_assert_malformed() {
    let wasm_binary = [40, 103, 108, 111, 98, 97, 108, 32, 105, 54, 52, 32, 40, 105, 54, 52, 46, 99, 111, 110, 115, 116, 32, 48, 95, 120, 49, 48, 48, 41, 41];
    let compilation = wasmer_runtime::compile(&wasm_binary, &CraneliftCompiler::new());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 141
#[test]
fn c48_l141_assert_malformed() {
    let wasm_binary = [40, 103, 108, 111, 98, 97, 108, 32, 105, 54, 52, 32, 40, 105, 54, 52, 46, 99, 111, 110, 115, 116, 32, 48, 120, 95, 49, 48, 48, 41, 41];
    let compilation = wasmer_runtime::compile(&wasm_binary, &CraneliftCompiler::new());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 145
#[test]
fn c49_l145_assert_malformed() {
    let wasm_binary = [40, 103, 108, 111, 98, 97, 108, 32, 105, 54, 52, 32, 40, 105, 54, 52, 46, 99, 111, 110, 115, 116, 32, 48, 120, 48, 48, 95, 41, 41];
    let compilation = wasmer_runtime::compile(&wasm_binary, &CraneliftCompiler::new());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

// Line 149
#[test]
fn c50_l149_assert_malformed() {
    let wasm_binary = [40, 103, 108, 111, 98, 97, 108, 32, 105, 54, 52, 32, 40, 105, 54, 52, 46, 99, 111, 110, 115, 116, 32, 48, 120, 102, 102, 95, 95, 102, 102, 102, 102, 41, 41];
    let compilation = wasmer_runtime::compile(&wasm_binary, &CraneliftCompiler::new());
    assert!(compilation.is_err(), "WASM should not compile as is malformed");
}

#[test]
fn test_module_1() {
    let mut instance = create_module_1();
    // We group the calls together
    start_module_1(&mut instance);
    c1_l37_action_invoke(&mut instance);
    c2_l38_action_invoke(&mut instance);
    c3_l39_action_invoke(&mut instance);
    c4_l40_action_invoke(&mut instance);
    c5_l41_action_invoke(&mut instance);
    c6_l42_action_invoke(&mut instance);
    c7_l43_action_invoke(&mut instance);
    c8_l44_action_invoke(&mut instance);
    c9_l45_action_invoke(&mut instance);
    c10_l46_action_invoke(&mut instance);
    c11_l47_action_invoke(&mut instance);
    c12_l49_action_invoke(&mut instance);
    c13_l50_action_invoke(&mut instance);
    c14_l51_action_invoke(&mut instance);
    c15_l52_action_invoke(&mut instance);
    c16_l53_action_invoke(&mut instance);
    c17_l54_action_invoke(&mut instance);
    c18_l55_action_invoke(&mut instance);
    c19_l56_action_invoke(&mut instance);
    c20_l57_action_invoke(&mut instance);
    c21_l58_action_invoke(&mut instance);
    c22_l59_action_invoke(&mut instance);
    c23_l61_action_invoke(&mut instance);
    c24_l62_action_invoke(&mut instance);
    c25_l63_action_invoke(&mut instance);
    c26_l64_action_invoke(&mut instance);
    c27_l66_action_invoke(&mut instance);
    c28_l67_action_invoke(&mut instance);
    c29_l68_action_invoke(&mut instance);
    c30_l69_action_invoke(&mut instance);
}