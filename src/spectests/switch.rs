// Rust test file autogenerated with cargo build (build/spectests.rs).
// Please do NOT modify it by hand, as it will be reseted on next build.
// Test based on spectests/switch.wast
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
      (type (;0;) (func (param i32) (result i32)))
      (type (;1;) (func (param i64) (result i64)))
      (type (;2;) (func (result i32)))
      (func (;0;) (type 0) (param i32) (result i32)
        (local i32)
        i32.const 100
        set_local 1
        block  ;; label = @1
          block  ;; label = @2
            block  ;; label = @3
              block  ;; label = @4
                block  ;; label = @5
                  block  ;; label = @6
                    block  ;; label = @7
                      block  ;; label = @8
                        block  ;; label = @9
                          block  ;; label = @10
                            get_local 0
                            br_table 0 (;@10;) 1 (;@9;) 2 (;@8;) 3 (;@7;) 4 (;@6;) 5 (;@5;) 6 (;@4;) 8 (;@2;) 7 (;@3;)
                          end
                          get_local 0
                          return
                        end
                        nop
                      end
                    end
                    i32.const 0
                    get_local 0
                    i32.sub
                    set_local 1
                    br 5 (;@1;)
                  end
                  br 4 (;@1;)
                end
                i32.const 101
                set_local 1
                br 3 (;@1;)
              end
              i32.const 101
              set_local 1
            end
            i32.const 102
            set_local 1
          end
        end
        get_local 1
        return)
      (func (;1;) (type 1) (param i64) (result i64)
        (local i64)
        i64.const 100
        set_local 1
        block (result i64)  ;; label = @1
          block  ;; label = @2
            block  ;; label = @3
              block  ;; label = @4
                block  ;; label = @5
                  block  ;; label = @6
                    block  ;; label = @7
                      block  ;; label = @8
                        block  ;; label = @9
                          block  ;; label = @10
                            get_local 0
                            i32.wrap/i64
                            br_table 0 (;@10;) 1 (;@9;) 2 (;@8;) 3 (;@7;) 6 (;@4;) 5 (;@5;) 4 (;@6;) 8 (;@2;) 7 (;@3;)
                          end
                          get_local 0
                          return
                        end
                        nop
                      end
                    end
                    i64.const 0
                    get_local 0
                    i64.sub
                    br 5 (;@1;)
                  end
                  i64.const 101
                  set_local 1
                end
              end
            end
            get_local 1
            br 1 (;@1;)
          end
          i64.const -5
        end
        return)
      (func (;2;) (type 0) (param i32) (result i32)
        block (result i32)  ;; label = @1
          i32.const 10
          block (result i32)  ;; label = @2
            i32.const 100
            block (result i32)  ;; label = @3
              i32.const 1000
              block (result i32)  ;; label = @4
                i32.const 2
                get_local 0
                i32.mul
                i32.const 3
                get_local 0
                i32.and
                br_table 1 (;@3;) 2 (;@2;) 3 (;@1;) 0 (;@4;)
              end
              i32.add
            end
            i32.add
          end
          i32.add
        end
        return)
      (func (;3;) (type 2) (result i32)
        block  ;; label = @1
          i32.const 0
          br_table 0 (;@1;)
        end
        i32.const 1)
      (export \"stmt\" (func 0))
      (export \"expr\" (func 1))
      (export \"arg\" (func 2))
      (export \"corner\" (func 3)))
    ";
    let wasm_binary = wat2wasm(module_str.as_bytes()).expect("WAST not valid or malformed");
    instantiate(wasm_binary, spectest_importobject(), None).expect("WASM can't be instantiated")
}

fn start_module_1(result_object: &ResultObject) {
    result_object.instance.start();
}

// Line 120
fn c1_l120_action_invoke(result_object: &ResultObject) {
    println!("Executing function {}", "c1_l120_action_invoke");
    let func_index = match result_object.module.info.exports.get("stmt") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(i32, &Instance) -> i32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(0 as i32, &result_object.instance);
    assert_eq!(result, 0 as i32);
}

// Line 121
fn c2_l121_action_invoke(result_object: &ResultObject) {
    println!("Executing function {}", "c2_l121_action_invoke");
    let func_index = match result_object.module.info.exports.get("stmt") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(i32, &Instance) -> i32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(1 as i32, &result_object.instance);
    assert_eq!(result, -1 as i32);
}

// Line 122
fn c3_l122_action_invoke(result_object: &ResultObject) {
    println!("Executing function {}", "c3_l122_action_invoke");
    let func_index = match result_object.module.info.exports.get("stmt") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(i32, &Instance) -> i32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(2 as i32, &result_object.instance);
    assert_eq!(result, -2 as i32);
}

// Line 123
fn c4_l123_action_invoke(result_object: &ResultObject) {
    println!("Executing function {}", "c4_l123_action_invoke");
    let func_index = match result_object.module.info.exports.get("stmt") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(i32, &Instance) -> i32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(3 as i32, &result_object.instance);
    assert_eq!(result, -3 as i32);
}

// Line 124
fn c5_l124_action_invoke(result_object: &ResultObject) {
    println!("Executing function {}", "c5_l124_action_invoke");
    let func_index = match result_object.module.info.exports.get("stmt") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(i32, &Instance) -> i32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(4 as i32, &result_object.instance);
    assert_eq!(result, 100 as i32);
}

// Line 125
fn c6_l125_action_invoke(result_object: &ResultObject) {
    println!("Executing function {}", "c6_l125_action_invoke");
    let func_index = match result_object.module.info.exports.get("stmt") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(i32, &Instance) -> i32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(5 as i32, &result_object.instance);
    assert_eq!(result, 101 as i32);
}

// Line 126
fn c7_l126_action_invoke(result_object: &ResultObject) {
    println!("Executing function {}", "c7_l126_action_invoke");
    let func_index = match result_object.module.info.exports.get("stmt") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(i32, &Instance) -> i32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(6 as i32, &result_object.instance);
    assert_eq!(result, 102 as i32);
}

// Line 127
fn c8_l127_action_invoke(result_object: &ResultObject) {
    println!("Executing function {}", "c8_l127_action_invoke");
    let func_index = match result_object.module.info.exports.get("stmt") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(i32, &Instance) -> i32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(7 as i32, &result_object.instance);
    assert_eq!(result, 100 as i32);
}

// Line 128
fn c9_l128_action_invoke(result_object: &ResultObject) {
    println!("Executing function {}", "c9_l128_action_invoke");
    let func_index = match result_object.module.info.exports.get("stmt") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(i32, &Instance) -> i32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(-10 as i32, &result_object.instance);
    assert_eq!(result, 102 as i32);
}

// Line 130
fn c10_l130_action_invoke(result_object: &ResultObject) {
    println!("Executing function {}", "c10_l130_action_invoke");
    let func_index = match result_object.module.info.exports.get("expr") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(i64, &Instance) -> i64 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(0 as i64, &result_object.instance);
    assert_eq!(result, 0 as i64);
}

// Line 131
fn c11_l131_action_invoke(result_object: &ResultObject) {
    println!("Executing function {}", "c11_l131_action_invoke");
    let func_index = match result_object.module.info.exports.get("expr") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(i64, &Instance) -> i64 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(1 as i64, &result_object.instance);
    assert_eq!(result, -1 as i64);
}

// Line 132
fn c12_l132_action_invoke(result_object: &ResultObject) {
    println!("Executing function {}", "c12_l132_action_invoke");
    let func_index = match result_object.module.info.exports.get("expr") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(i64, &Instance) -> i64 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(2 as i64, &result_object.instance);
    assert_eq!(result, -2 as i64);
}

// Line 133
fn c13_l133_action_invoke(result_object: &ResultObject) {
    println!("Executing function {}", "c13_l133_action_invoke");
    let func_index = match result_object.module.info.exports.get("expr") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(i64, &Instance) -> i64 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(3 as i64, &result_object.instance);
    assert_eq!(result, -3 as i64);
}

// Line 134
fn c14_l134_action_invoke(result_object: &ResultObject) {
    println!("Executing function {}", "c14_l134_action_invoke");
    let func_index = match result_object.module.info.exports.get("expr") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(i64, &Instance) -> i64 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(6 as i64, &result_object.instance);
    assert_eq!(result, 101 as i64);
}

// Line 135
fn c15_l135_action_invoke(result_object: &ResultObject) {
    println!("Executing function {}", "c15_l135_action_invoke");
    let func_index = match result_object.module.info.exports.get("expr") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(i64, &Instance) -> i64 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(7 as i64, &result_object.instance);
    assert_eq!(result, -5 as i64);
}

// Line 136
fn c16_l136_action_invoke(result_object: &ResultObject) {
    println!("Executing function {}", "c16_l136_action_invoke");
    let func_index = match result_object.module.info.exports.get("expr") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(i64, &Instance) -> i64 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(-10 as i64, &result_object.instance);
    assert_eq!(result, 100 as i64);
}

// Line 138
fn c17_l138_action_invoke(result_object: &ResultObject) {
    println!("Executing function {}", "c17_l138_action_invoke");
    let func_index = match result_object.module.info.exports.get("arg") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(i32, &Instance) -> i32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(0 as i32, &result_object.instance);
    assert_eq!(result, 110 as i32);
}

// Line 139
fn c18_l139_action_invoke(result_object: &ResultObject) {
    println!("Executing function {}", "c18_l139_action_invoke");
    let func_index = match result_object.module.info.exports.get("arg") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(i32, &Instance) -> i32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(1 as i32, &result_object.instance);
    assert_eq!(result, 12 as i32);
}

// Line 140
fn c19_l140_action_invoke(result_object: &ResultObject) {
    println!("Executing function {}", "c19_l140_action_invoke");
    let func_index = match result_object.module.info.exports.get("arg") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(i32, &Instance) -> i32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(2 as i32, &result_object.instance);
    assert_eq!(result, 4 as i32);
}

// Line 141
fn c20_l141_action_invoke(result_object: &ResultObject) {
    println!("Executing function {}", "c20_l141_action_invoke");
    let func_index = match result_object.module.info.exports.get("arg") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(i32, &Instance) -> i32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(3 as i32, &result_object.instance);
    assert_eq!(result, 1116 as i32);
}

// Line 142
fn c21_l142_action_invoke(result_object: &ResultObject) {
    println!("Executing function {}", "c21_l142_action_invoke");
    let func_index = match result_object.module.info.exports.get("arg") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(i32, &Instance) -> i32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(4 as i32, &result_object.instance);
    assert_eq!(result, 118 as i32);
}

// Line 143
fn c22_l143_action_invoke(result_object: &ResultObject) {
    println!("Executing function {}", "c22_l143_action_invoke");
    let func_index = match result_object.module.info.exports.get("arg") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(i32, &Instance) -> i32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(5 as i32, &result_object.instance);
    assert_eq!(result, 20 as i32);
}

// Line 144
fn c23_l144_action_invoke(result_object: &ResultObject) {
    println!("Executing function {}", "c23_l144_action_invoke");
    let func_index = match result_object.module.info.exports.get("arg") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(i32, &Instance) -> i32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(6 as i32, &result_object.instance);
    assert_eq!(result, 12 as i32);
}

// Line 145
fn c24_l145_action_invoke(result_object: &ResultObject) {
    println!("Executing function {}", "c24_l145_action_invoke");
    let func_index = match result_object.module.info.exports.get("arg") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(i32, &Instance) -> i32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(7 as i32, &result_object.instance);
    assert_eq!(result, 1124 as i32);
}

// Line 146
fn c25_l146_action_invoke(result_object: &ResultObject) {
    println!("Executing function {}", "c25_l146_action_invoke");
    let func_index = match result_object.module.info.exports.get("arg") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(i32, &Instance) -> i32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(8 as i32, &result_object.instance);
    assert_eq!(result, 126 as i32);
}

// Line 148
fn c26_l148_action_invoke(result_object: &ResultObject) {
    println!("Executing function {}", "c26_l148_action_invoke");
    let func_index = match result_object.module.info.exports.get("corner") {
        Some(&Export::Function(index)) => index,
        _ => panic!("Function not found"),
    };
    let invoke_fn: fn(&Instance) -> i32 = get_instance_function!(result_object.instance, func_index);
    let result = invoke_fn(&result_object.instance);
    assert_eq!(result, 1 as i32);
}

// Line 150
#[test]
fn c27_l150_assert_invalid() {
    let wasm_binary = [0, 97, 115, 109, 1, 0, 0, 0, 1, 4, 1, 96, 0, 0, 3, 2, 1, 0, 10, 9, 1, 7, 0, 65, 0, 14, 0, 3, 11];
    let compilation = compile(wasm_binary.to_vec());
    assert!(compilation.is_err(), "WASM should not compile as is invalid");
}

#[test]
fn test_module_1() {
    let result_object = create_module_1();
    // We group the calls together
    start_module_1(&result_object);
    c1_l120_action_invoke(&result_object);
    c2_l121_action_invoke(&result_object);
    c3_l122_action_invoke(&result_object);
    c4_l123_action_invoke(&result_object);
    c5_l124_action_invoke(&result_object);
    c6_l125_action_invoke(&result_object);
    c7_l126_action_invoke(&result_object);
    c8_l127_action_invoke(&result_object);
    c9_l128_action_invoke(&result_object);
    c10_l130_action_invoke(&result_object);
    c11_l131_action_invoke(&result_object);
    c12_l132_action_invoke(&result_object);
    c13_l133_action_invoke(&result_object);
    c14_l134_action_invoke(&result_object);
    c15_l135_action_invoke(&result_object);
    c16_l136_action_invoke(&result_object);
    c17_l138_action_invoke(&result_object);
    c18_l139_action_invoke(&result_object);
    c19_l140_action_invoke(&result_object);
    c20_l141_action_invoke(&result_object);
    c21_l142_action_invoke(&result_object);
    c22_l143_action_invoke(&result_object);
    c23_l144_action_invoke(&result_object);
    c24_l145_action_invoke(&result_object);
    c25_l146_action_invoke(&result_object);
    c26_l148_action_invoke(&result_object);
}
