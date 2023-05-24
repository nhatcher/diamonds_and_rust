use crate::errors::{EvaluationError, Result};
use crate::{analyzer::Context, parser::ExpressionNode};

pub(crate) fn evaluate_with_globals(expr: &ExpressionNode, context: &Context) -> Result<f64> {
    todo!()
}
