(module
  (func (param $lhs f64) (param $rhs f64) (result f64)
      f64.const 42
      local.get $lhs
      f64.mul
      local.get $rhs
      f64.add
  )
  (export "main" (func 0))
)
