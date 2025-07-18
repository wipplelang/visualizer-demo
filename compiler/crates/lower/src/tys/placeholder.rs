use crate::{Visit, Visitor};
use wipple_compiler_syntax::PlaceholderType;
use wipple_compiler_trace::{NodeId, Rule};
use wipple_compiler_typecheck::nodes::EmptyNode;

pub static PLACEHOLDER_TYPE: Rule = Rule::new("placeholder type");

impl Visit for PlaceholderType {
    fn visit<'a>(&'a self, visitor: &mut Visitor<'a>, parent: (NodeId, Rule)) -> NodeId {
        visitor.node(parent, self.range, |_visitor, _id| {
            (EmptyNode, PLACEHOLDER_TYPE)
        })
    }
}
