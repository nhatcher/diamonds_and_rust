(module
  (type (;0;) (func (param f64) (result f64)))
  (type (;1;) (func))
  (func (;0;) (type 1)
    nop)
  (func (;1;) (type 0) (param f64) (result f64)
    (local i32)
    i32.const 66560
    local.tee 1
    local.get 0
    f64.store offset=8
    local.get 1
    f64.load offset=8
    f64.const 0x1.8p+2 (;=6;)
    f64.mul)
  (func (;2;) (type 0) (param f64) (result f64)
    (local i32)
    i32.const 66560
    local.tee 1
    local.get 0
    f64.store offset=8
    local.get 1
    f64.load offset=8
    f64.const 0x1.5p+5 (;=42;)
    f64.mul
    f64.const 0x1p+2 (;=4;)
    f64.add)
  (memory (;0;) 2)
  (global (;0;) i32 (i32.const 1024))
  (global (;1;) i32 (i32.const 1032))
  (global (;2;) i32 (i32.const 1024))
  (global (;3;) i32 (i32.const 1040))
  (global (;4;) i32 (i32.const 1024))
  (global (;5;) i32 (i32.const 66576))
  (global (;6;) i32 (i32.const 0))
  (global (;7;) i32 (i32.const 1))
  (export "memory" (memory 0))
  (export "__wasm_call_ctors" (func 0))
  (export "gh" (func 1))
  (export "fn" (func 2))
  (export "a" (global 0))
  (export "d" (global 1))
  (export "__dso_handle" (global 2))
  (export "__data_end" (global 3))
  (export "__global_base" (global 4))
  (export "__heap_base" (global 5))
  (export "__memory_base" (global 6))
  (export "__table_base" (global 7))
  (data (;0;) (i32.const 1030) "\18@\00\00\00\00\00\00\10@"))
