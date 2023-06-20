// clang --target=wasm32 --no-standard-libraries -Wl,--export-all -Wl,--no-entry -o test_loop.wasm test_loop.c


double fn(double x) {
    double result = 0;
    for (int i=0; i<x; i++) {
        result += x;
    }
    return result;
}

/*
(module
  (type (;1;) (func (param f64) (result f64)))
  (func (;1;) (type 0) (param f64) (result f64)
    (local i32) ;; i
    (local f64) ;; result
    ;; i = 0
    i32.const 0
    local.set 1
    ;; result = 0
    f64.const 0
    local.set 2
    loop
      ;; i < x 
      local.get 1
      f64.convert_i32_s
      local.get 0
      f64.lt
      if
        ;; result = result + x
        local.get 0
        local.get 2
        f64.add
        local.set 2
        
        ;; i++
        local.get 1
        i32.const 1
        i32.add
        local.set 1
        br 1
      end
    end
    local.get 2)
  (export "fn" (func 0)))


*/