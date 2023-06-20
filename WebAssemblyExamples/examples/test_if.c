// clang --target=wasm32 --no-standard-libraries -Wl,--export-all -Wl,--no-entry -o test_if.wasm test_if.c


double fn(double x) {
    if (x>5.5) {
        return 2.0*x;
    } else {
        return 3.0*x;
    }
}


/*
(module
  (type (;0;) (func (param f64) (result f64)))
  (func (;0;) (type 0) (param f64) (result f64)
    block (result f64)
      local.get 0
      f64.const 5.5
      f64.gt
      if
        local.get 0
        f64.const 2.0
        f64.mul
        br 1
      end
      local.get 0
      f64.const 3.0
      f64.mul
    end)
  (export "fn" (func 0)))
*/

/*
(module
  (type (;0;) (func (param f64) (result f64)))
  (func (;0;) (type 0) (param f64) (result f64)
    local.get 0
    f64.const 5.5
    f64.gt
    if (result f64)
      local.get 0
      f64.const 2.0
      f64.mul
    else
      local.get 0
      f64.const 3.0
      f64.mul
    end)
  (export "fn" (func 0)))
*/