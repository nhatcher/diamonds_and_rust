#[derive(Debug)]
pub enum Builtin {
    Sin,
    Cos,
    Tan,

    Asin,
    Acos,
    Atan,

    Sinh,
    Cosh,
    Tanh,

    Asinh,
    Acosh,
    Atanh,

    Log,
    Log10,
    Exp,
    Sqrt,

    Atan2,
    Pow,
}

impl Builtin {
    pub fn arg_count(&self) -> u8 {
        match &self {
            Builtin::Atan2 | Builtin::Pow => 2,
            _ => 1,
        }
    }
    pub fn name(&self) -> &str {
        match &self {
            Builtin::Sin => "Sin",
            Builtin::Cos => "Cos",
            Builtin::Tan => "Tan",
            Builtin::Asin => "Asin",
            Builtin::Acos => "Acos",
            Builtin::Atan => "Atan",
            Builtin::Sinh => "Sinh",
            Builtin::Cosh => "Cosh",
            Builtin::Tanh => "Tanh",
            Builtin::Asinh => "Asinh",
            Builtin::Acosh => "Acosh",
            Builtin::Atanh => "Atanh",
            Builtin::Log => "Log",
            Builtin::Log10 => "Log10",
            Builtin::Exp => "Exp",
            Builtin::Sqrt => "Sqrt",
            Builtin::Atan2 => "Atan2",
            Builtin::Pow => "Pow",
        }
    }
}

pub(crate) fn get_builtin_by_name(name: &str) -> Option<Builtin> {
    match name {
        "Sin" => Some(Builtin::Sin),
        "Cos" => Some(Builtin::Cos),
        "Tan" => Some(Builtin::Tan),

        "Asin" => Some(Builtin::Asin),
        "Acos" => Some(Builtin::Acos),
        "Atan" => Some(Builtin::Atan),

        "Sinh" => Some(Builtin::Sinh),
        "Cosh" => Some(Builtin::Cosh),
        "Tanh" => Some(Builtin::Tanh),

        "Asinh" => Some(Builtin::Asinh),
        "Acosh" => Some(Builtin::Acosh),
        "Atanh" => Some(Builtin::Atanh),

        "Log" => Some(Builtin::Log),
        "Log10" => Some(Builtin::Log10),
        "Exp" => Some(Builtin::Exp),
        "Sqrt" => Some(Builtin::Sqrt),

        "Atan2" => Some(Builtin::Atan2),
        "Pow" => Some(Builtin::Pow),
        _ => None,
    }
}
