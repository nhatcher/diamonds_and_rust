(module
  (type (;0;) (func (param f64) (result f64)))
  (type (;1;) (func))
  (type (;2;) (func (param i32) (result i32)))
  (type (;3;) (func (param i32)))
  (type (;4;) (func (param f64 f64)))
  (func (;0;) (type 1)
    nop)
  (func (;1;) (type 2) (param i32) (result i32)
    i32.const 1024
    local.get 0
    i32.const 1024
    i32.load
    local.tee 0
    i32.add
    i32.store
    local.get 0)
  (func (;2;) (type 3) (param i32)
    i32.const 1024
    i32.const 1024
    i32.load
    local.get 0
    i32.sub
    i32.store)
  (func (;3;) (type 0) (param f64) (result f64)
    local.get 0
    local.get 0
    f64.mul)
  (func (;4;) (type 0) (param f64) (result f64)
    local.get 0
    local.get 0
    f64.add)
  (func (;5;) (type 4) (param f64 f64)
    (local i32 i32)
    i32.const 1024
    i32.load
    local.set 2
    f64.const 0x0p+0 (;=0;)
    local.set 0
    i32.const 3
    local.set 3
    loop  ;; label = @1
      local.get 2
      local.get 0
      f64.const 0x1.47ae147ae147bp-7 (;=0.01;)
      f64.mul
      f64.const 0x1.8p+1 (;=3;)
      f64.add
      local.tee 1
      local.get 1
      f64.mul
      f64.store
      local.get 2
      i32.const 24
      i32.add
      local.get 3
      f64.convert_i32_s
      f64.const 0x1.47ae147ae147bp-7 (;=0.01;)
      f64.mul
      f64.const 0x1.8p+1 (;=3;)
      f64.add
      local.tee 1
      local.get 1
      f64.mul
      f64.store
      local.get 2
      i32.const 16
      i32.add
      local.get 3
      i32.const 1
      i32.sub
      f64.convert_i32_s
      f64.const 0x1.47ae147ae147bp-7 (;=0.01;)
      f64.mul
      f64.const 0x1.8p+1 (;=3;)
      f64.add
      local.tee 1
      local.get 1
      f64.mul
      f64.store
      local.get 2
      i32.const 8
      i32.add
      local.get 3
      i32.const 2
      i32.sub
      f64.convert_i32_s
      f64.const 0x1.47ae147ae147bp-7 (;=0.01;)
      f64.mul
      f64.const 0x1.8p+1 (;=3;)
      f64.add
      local.tee 1
      local.get 1
      f64.mul
      f64.store
      local.get 2
      i32.const 32
      i32.add
      local.set 2
      local.get 0
      f64.const 0x1p+2 (;=4;)
      f64.add
      local.set 0
      local.get 3
      i32.const 4
      i32.add
      local.tee 3
      i32.const 103
      i32.ne
      br_if 0 (;@1;)
    end
    i32.const 1024
    local.get 2
    i32.store)
  (memory (;0;) 2)
  (global (;0;) i32 (i32.const 66576))
  (global (;1;) i32 (i32.const 1024))
  (global (;2;) i32 (i32.const 1024))
  (global (;3;) i32 (i32.const 1028))
  (global (;4;) i32 (i32.const 1024))
  (global (;5;) i32 (i32.const 0))
  (global (;6;) i32 (i32.const 1))
  (export "memory" (memory 0))
  (export "__wasm_call_ctors" (func 0))
  (export "pmalloc" (func 1))
  (export "pfree" (func 2))
  (export "fn1" (func 3))
  (export "fn2" (func 4))
  (export "redraw" (func 5))
  (export "__heap_base" (global 0))
  (export "bump_pointer" (global 1))
  (export "__dso_handle" (global 2))
  (export "__data_end" (global 3))
  (export "__global_base" (global 4))
  (export "__memory_base" (global 5))
  (export "__table_base" (global 6))
  (data (;0;) (i32.const 1024) "\10\04\01"))
