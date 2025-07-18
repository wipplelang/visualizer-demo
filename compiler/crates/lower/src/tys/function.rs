use crate::{Visit, Visitor};
use wipple_compiler_syntax::FunctionType;
use wipple_compiler_trace::{NodeId, Rule};
use wipple_compiler_typecheck::nodes::FunctionNode;

pub static FUNCTION_TYPE: Rule = Rule::new("function type");
pub static FUNCTION_TYPE_INPUT: Rule = Rule::new("function type input");
pub static FUNCTION_TYPE_OUTPUT: Rule = Rule::new("function type output");

impl Visit for FunctionType {
    fn visit<'a>(&'a self, visitor: &mut Visitor<'a>, parent: (NodeId, Rule)) -> NodeId {
        visitor.node(parent, self.range, |visitor, id| {
            let inputs = self
                .inputs
                .0
                .iter()
                .map(|input| input.visit(visitor, (id, FUNCTION_TYPE_INPUT)))
                .collect::<Vec<_>>();

            let output = self.output.visit(visitor, (id, FUNCTION_TYPE_OUTPUT));

            (FunctionNode { inputs, output }, FUNCTION_TYPE)
        })
    }
}
