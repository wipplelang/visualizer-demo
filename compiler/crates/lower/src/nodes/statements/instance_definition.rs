use crate::{
    attributes::{AttributeParser, InstanceAttributes},
    definitions::{Definition, InstanceDefinition},
    visitor::{Visit, Visitor},
};
use std::collections::BTreeMap;
use wipple_compiler_syntax::{Constraints, InstanceDefinitionStatement, Range};
use wipple_compiler_trace::{Fact, NodeId};
use wipple_compiler_typecheck::constraints::{Constraint, Instantiation, Substitutions, Ty};

impl Visit for InstanceDefinitionStatement {
    fn name(&self) -> &'static str {
        "instanceDefinition"
    }

    fn range(&self) -> Range {
        self.constraints.range
    }

    fn visit(&self, id: NodeId, visitor: &mut Visitor<'_>) {
        let attributes =
            InstanceAttributes::parse(visitor, &mut AttributeParser::new(id, &self.attributes));

        let Some((trait_node, trait_parameters)) =
            visitor.resolve_name(&self.constraints.bound.r#trait.value, id, |definition| {
                match definition {
                    Definition::Trait(definition) => Some((
                        (definition.node, definition.parameters.clone()),
                        "trait in instance definition",
                    )),
                    _ => None,
                }
            })
        else {
            visitor.fact(id, Fact::marker("unresolvedTraitName"));
            return;
        };

        let (value, constraints, substitutions) = visitor.with_definition(|visitor| {
            visitor.current_definition().implicit_type_parameters = true;

            let parameters = self
                .constraints
                .bound
                .parameters
                .iter()
                .map(|ty| visitor.child(ty, id, "parameterInInstanceDefinition"));

            // TODO: Ensure `parameters` has the right length
            let substitutions = trait_parameters
                .into_iter()
                .zip(parameters)
                .collect::<BTreeMap<_, _>>();

            visitor.current_definition().lazy_constraint({
                let substitutions = substitutions.clone();
                move |node| {
                    Constraint::Instantiation(Instantiation {
                        substitutions: Substitutions::from(substitutions.clone()),
                        constraints: vec![Constraint::Ty(node, Ty::Of(trait_node))],
                    })
                }
            });

            if let Some(Constraints(constraints)) = &self.constraints.constraints {
                for constraint in constraints {
                    visitor.child(constraint, id, "constraintInInstanceDefinition");
                }
            }

            visitor.current_definition().implicit_type_parameters = false;

            let value = visitor.child(&self.value, id, "valueInInstanceDefinition");

            (
                value,
                visitor.current_definition().take_constraints(),
                substitutions,
            )
        });

        visitor.define_instance(InstanceDefinition {
            node: id,
            comments: self.comments.clone(),
            attributes,
            tr: trait_node,
            substitutions: substitutions.clone(),
            constraints,
            value,
        });

        visitor.constraints(vec![
            Constraint::Instantiation(Instantiation {
                substitutions: Substitutions::from(substitutions.clone()),
                constraints: vec![Constraint::Ty(id, Ty::Of(trait_node))],
            }),
            Constraint::Ty(id, Ty::Of(value)),
        ]);
    }

    fn is_typed(&self) -> bool {
        true
    }
}
