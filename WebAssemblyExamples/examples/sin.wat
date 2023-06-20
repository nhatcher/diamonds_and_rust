(module
  (func $sin (import "imports" "sin") (param f64) (result f64))
  (func $cos (import "imports" "cos") (param f64) (result f64))
  (func $tan (import "imports" "tan") (param f64) (result f64))
  (func $log (import "imports" "log") (param f64) (result f64))
  (func $exp (import "imports" "exp") (param f64) (result f64))
  (func $pow (import "imports" "pow") (param f64) (param f64) (result f64))
  (func (result f64)
      f64.const 42
      f64.const 1.5
      f64.mul
      call $sin
  )
  (export "main" (func 6))
)
