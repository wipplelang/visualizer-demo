use crate::{
    constraints::instantiate_constraints,
    definitions::Definition,
    visitor::{Visit, Visitor},
};
use wipple_compiler_syntax::{Range, VariableExpression};
use wipple_compiler_trace::{Fact, NodeId};
use wipple_compiler_typecheck::constraints::{Constraint, Instantiation, Substitutions, Ty};

impl Visit for VariableExpression {
    fn name(&self) -> &'static str {
        "variable"
    }

    fn range(&self) -> Range {
        self.range
    }

    fn visit(&self, id: NodeId, visitor: &mut Visitor<'_>) {
        let constraint =
            visitor.resolve_name(&self.variable.value, id, |definition| match definition {
                Definition::Variable(definition) => Some((
                    Constraint::Ty(id, Ty::Of(definition.node)),
                    "resolved variable name",
                )),
                Definition::Constant(definition) => Some((
                    Constraint::Instantiation(Instantiation {
                        substitutions: Substitutions::replace_all(),
                        constraints: instantiate_constraints(
                            id,
                            definition.constraints.iter().cloned(),
                        )
                        .collect(),
                    }),
                    "resolved constant name",
                )),
                _ => None,
            });

        if let Some(constraint) = constraint {
            visitor.constraint(constraint);
        } else {
            visitor.fact(id, Fact::marker("unresolvedVariableName"));
        }
    }

    fn is_typed(&self) -> bool {
        true
    }
}
