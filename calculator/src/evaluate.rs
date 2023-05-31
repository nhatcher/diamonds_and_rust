use crate::errors::{EvaluationError, Result};
use crate::{analyzer::Context, parser::ExpressionNode};

pub(crate) fn evaluate_in_context(expr: &ExpressionNode, context: &Context) -> Result<f64> {
    todo!()
}
