use crate::visitor::{Visit, Visitor};
use wipple_compiler_syntax::{FormattedTextExpression, Range};
use wipple_compiler_trace::NodeId;
// TODO

impl Visit for FormattedTextExpression {
    fn name(&self) -> &'static str {
        "formattedText"
    }

    fn range(&self) -> Range {
        self.range
    }

    fn visit(&self, id: NodeId, visitor: &mut Visitor<'_>) {
        todo!()
    }

    fn is_typed(&self) -> bool {
        true
    }
}
