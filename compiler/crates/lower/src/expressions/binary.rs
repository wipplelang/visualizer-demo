use crate::{Definition, Visit, Visitor};
use wipple_compiler_syntax::BinaryExpression;
use wipple_compiler_trace::{NodeId, Rule};
use wipple_compiler_typecheck::{
    constraints::{Constraint, Ty},
    nodes::{CallNode, ConstraintNode, Node, PlaceholderNode},
};

pub const OPERATOR: Rule = Rule::new("operator");

pub const EQUAL: Rule = Rule::new("equal");

pub const EQUAL_OPERATOR_LEFT: Rule = Rule::new("equal operator left");

pub const EQUAL_OPERATOR_RIGHT: Rule = Rule::new("equal operator right");

pub const MISSING_EQUAL_TRAIT: Rule = Rule::new("missing equal trait");

impl Visit for BinaryExpression {
    fn visit<'a>(&'a self, visitor: &mut Visitor<'a>, parent: (NodeId, Rule)) -> NodeId {
        match self.operator.source.as_str() {
            "to" => visit_as_math_expression(self, visitor, "To"),
            "by" => visit_as_math_expression(self, visitor, "By"),
            "^" => visit_as_math_expression(self, visitor, "Power"),
            "*" => visit_as_math_expression(self, visitor, "Multiply"),
            "/" => visit_as_math_expression(self, visitor, "Divide"),
            "%" => visit_as_math_expression(self, visitor, "Remainder"),
            "+" => visit_as_math_expression(self, visitor, "Add"),
            "-" => visit_as_math_expression(self, visitor, "Subtract"),
            "<" => visit_as_comparison_expression(self, visitor, ["Less-Than"]),
            "<=" => visit_as_comparison_expression(self, visitor, ["Less-Than", "Equal"]),
            ">" => visit_as_comparison_expression(self, visitor, ["Greater-Than"]),
            ">=" => visit_as_comparison_expression(self, visitor, ["Greater-Than", "Equal"]),
            "=" => visitor.typed_node(parent, &self.range, |visitor, id| {
                let function =
                    visitor.typed_node((id, OPERATOR), &self.operator.range, |visitor, id| {
                        let equal_function =
                            visitor.resolve_name("Equal", id, |definition| match definition {
                                Definition::Trait(definition) => Some((definition.node, OPERATOR)),
                                _ => None,
                            });

                        match equal_function {
                            Some((equal_function, rule)) => (
                                ConstraintNode {
                                    value: id,
                                    constraints: vec![Constraint::Generic(equal_function)],
                                }
                                .boxed(),
                                rule,
                            ),
                            None => (PlaceholderNode.boxed(), MISSING_EQUAL_TRAIT),
                        }
                    });

                let inputs = [
                    (self.left.as_ref(), EQUAL_OPERATOR_LEFT),
                    (self.right.as_ref(), EQUAL_OPERATOR_RIGHT),
                ]
                .into_iter()
                .map(|(input, rule)| input.visit(visitor, (id, rule)))
                .collect::<Vec<_>>();

                (CallNode { function, inputs }.boxed(), EQUAL)
            }),
            "/=" => {
                // TODO: a = b <=> not (Equal a b)
                todo!()
            }
            "and" => {
                // TODO: a = b <=> And a {b}
                todo!()
            }
            "or" => {
                // TODO: a = b <=> Or a {b}
                todo!()
            }
            operator => panic!("invalid binary operator {operator:?}"),
        }
    }
}

fn visit_as_math_expression(
    expression: &BinaryExpression,
    visitor: &mut Visitor<'_>,
    trait_name: &str,
) -> NodeId {
    todo!()
}

fn visit_as_comparison_expression<const N: usize>(
    expression: &BinaryExpression,
    visitor: &mut Visitor<'_>,
    variant_names: [&str; N],
) -> NodeId {
    todo!()
}
