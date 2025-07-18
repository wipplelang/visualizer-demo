use crate::{Definition, Visit, Visitor};
use wipple_compiler_syntax::TraitExpression;
use wipple_compiler_trace::{NodeId, Rule};
use wipple_compiler_typecheck::nodes::{AnnotateNode, Annotation, EmptyNode, Node};

pub static TRAIT_NAME: Rule = Rule::new("trait name");
pub static RESOLVED_TRAIT_NAME: Rule = Rule::new("resolved trait name");
pub static UNRESOLVED_TRAIT_NAME: Rule = Rule::new("unresolved trait name");

impl Visit for TraitExpression {
    fn visit<'a>(&'a self, visitor: &mut Visitor<'a>, parent: (NodeId, Rule)) -> NodeId {
        visitor.typed_node(parent, self.range, |visitor, id| {
            visitor.push_scope(id);

            let constraints =
                visitor.resolve_name(&self.r#type.value, id, |definition| match definition {
                    Definition::Type(_) => todo!(),
                    Definition::Trait(definition) => Some((definition.node, RESOLVED_TRAIT_NAME)),
                    _ => None,
                });

            visitor.pop_scope();

            if let Some((definition, rule)) = constraints {
                (
                    AnnotateNode {
                        value: id,
                        definition: Annotation::Constant(definition),
                    }
                    .boxed(),
                    rule,
                )
            } else {
                (EmptyNode.boxed(), UNRESOLVED_TRAIT_NAME)
            }
        })
    }
}
