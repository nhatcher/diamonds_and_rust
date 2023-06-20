(module
  (func (param f64 f64) (result f64)
      f64.const 42
      local.get 0
      f64.mul
      local.get 1
      f64.add
  )
  (export "main" (func 0))
)