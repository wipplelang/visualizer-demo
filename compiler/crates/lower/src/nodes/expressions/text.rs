use crate::{
    definitions::Definition,
    visitor::{Visit, Visitor},
};
use std::collections::BTreeMap;
use wipple_compiler_syntax::{Range, TextExpression};
use wipple_compiler_trace::{Fact, NodeId};
use wipple_compiler_typecheck::constraints::{Constraint, Ty};

impl Visit for TextExpression {
    fn name(&self) -> &'static str {
        "text"
    }

    fn range(&self) -> Range {
        self.range
    }

    fn visit(&self, id: NodeId, visitor: &mut Visitor<'_>) {
        let text_ty = visitor.resolve_name("Text", id, |definition| match definition {
            Definition::Type(definition) => Some((definition.node, "text")),
            _ => None,
        });

        if let Some(text_ty) = text_ty {
            visitor.constraint(Constraint::Ty(
                id,
                Ty::Named {
                    name: text_ty,
                    parameters: BTreeMap::new(),
                },
            ));
        } else {
            visitor.fact(id, Fact::marker("missingTextType"));
        }
    }

    fn is_typed(&self) -> bool {
        true
    }
}
