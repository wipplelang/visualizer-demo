mod block;
mod function;
mod named;
mod parameter;
mod parameterized;
mod placeholder;
mod tuple;
mod unit;

use crate::{Visit, Visitor};
use wipple_compiler_syntax::Type;
use wipple_compiler_trace::{NodeId, Rule};

impl Visit for Type {
    fn visit<'a>(&'a self, visitor: &mut Visitor<'a>, parent: (NodeId, Rule)) -> NodeId {
        match self {
            Type::Placeholder(ty) => ty.visit(visitor, parent),
            Type::Unit(ty) => ty.visit(visitor, parent),
            Type::Named(ty) => ty.visit(visitor, parent),
            Type::Parameterized(ty) => ty.visit(visitor, parent),
            Type::Block(ty) => ty.visit(visitor, parent),
            Type::Function(ty) => ty.visit(visitor, parent),
            Type::Parameter(ty) => ty.visit(visitor, parent),
            Type::Tuple(ty) => ty.visit(visitor, parent),
        }
    }
}
