
(module
	(type $Point 
		(struct 
			(field $x f64)
			(field $y f64)))
	(func $create (export "create_point") (param $x f64) (param $y f64) (result (ref $Point))
		(struct.new $Point (local.get $x) (local.get $y))
	)
	(func $length (export "length") (param $p (ref $Point)) (result f64)
		(f64.add
			(f64.mul
				(struct.get $Point $x (local.get $p))
				(struct.get $Point $x (local.get $p)))
			(f64.mul
				(struct.get $Point $y (local.get $p))
				(struct.get $Point $y (local.get $p))))
	)
)