#![feature(test)]

use bumpalo::collections::Vec;
use oxc_allocator::Allocator;

mod ast;
mod bench;
mod print;
mod visit;
use ast::{
    AsAstRef, AstRef, BinaryOperator, Expression, IdentifierReference, NodeId, Statement,
    StringLiteral, UnaryExpression, UnaryOperator,
};
use print::Printer;
use visit::Visit;

use crate::ast::BinaryExpression;

struct Nodes<'a>(Vec<'a, AstRef<'a>>);

impl<'a> Nodes<'a> {
    pub fn get_node(&self, id: NodeId<'a>) -> &AstRef<'a> {
        &self.0[id.as_index()]
    }

    pub fn get_node_mut(&mut self, id: NodeId<'a>) -> &mut AstRef<'a> {
        &mut self.0[id.as_index()]
    }
}

fn main() {
    test();
}

pub fn test() {
    let alloc = Allocator::default();
    let (mut nodes, stmt) = parse(&alloc);
    #[cfg(not(feature = "test"))]
    println!("before: {}", Printer::print(stmt, &mut nodes));
    TransformTypeof.build(stmt, &mut nodes);
    #[cfg(not(feature = "test"))]
    println!("after: {}", Printer::print(stmt, &mut nodes));
}

/// Create AST for `typeof foo === 'object'`.
/// Hard-coded here, but these are the steps actual parser would take to create the AST
/// with "back-links" to parents on each node.
fn parse<'a, 't>(alloc: &'a Allocator) -> (Nodes<'a>, NodeId<'a>) {
    let mut nodes = Vec::with_capacity_in(5, alloc);
    macro_rules! push {
        ($expr:expr) => {{
            nodes.push(alloc.alloc($expr).as_ast_ref());
            NodeId::new(nodes.len() - 1)
        }};
    }

    // `foo`
    let ident = IdentifierReference {
        name: "foo",
        parent: NodeId::default(),
    };

    let ident_id = push!(ident);
    let ident_expr = Expression::Identifier(ident_id);
    let ident_expr_id = push!(ident_expr);

    // `typeof foo`
    let unary_expr = UnaryExpression {
        operator: UnaryOperator::Typeof,
        argument: ident_expr_id,
        parent: NodeId::default(),
    };

    let unary_id = push!(unary_expr);
    let unary_expr_id = push!(Expression::UnaryExpression(unary_id));
    (&mut nodes[ident_id.as_index()])
        .as_ident_mut_unchecked()
        .parent = unary_id;

    // `'object'`
    let str_lit = StringLiteral {
        value: "object",
        parent: NodeId::default(),
    };
    let str_id = push!(str_lit);
    let str_expr_id = push!(Expression::StringLiteral(str_id));

    // `typeof foo === 'object'` (as expression)
    let binary_expr = BinaryExpression {
        operator: BinaryOperator::StrictEquality,
        left: unary_expr_id,
        right: str_expr_id,
        parent: NodeId::default(),
    };
    let binary_id = push!(binary_expr);

    nodes[unary_id.as_index()].as_unary_mut_unchecked().parent = binary_id;
    nodes[str_id.as_index()].as_str_mut_unchecked().parent = binary_id;

    // `typeof foo === 'object'` (as expression)
    let expr_id = push!(Expression::BinaryExpression(binary_id));
    nodes[binary_id.as_index()].as_binary_mut_unchecked().parent = expr_id;

    // `typeof foo === 'object'` (as statement)
    let stmt_id = push!(Statement::ExpressionStatement(expr_id));

    (Nodes(nodes), stmt_id)
}

/// Transformer for `typeof x === 'y'` to `'y' === typeof x`
struct TransformTypeof;

impl TransformTypeof {
    pub fn build<'a>(&mut self, id: NodeId<'a>, nodes: &mut Nodes<'a>) {
        self.visit_statement(id, nodes);
    }
}

impl<'a> Visit<'a> for TransformTypeof {
    fn visit_unary_expression(&mut self, id: NodeId<'a>, nodes: &mut Nodes<'a>) {
        let node = nodes.get_node(id).as_unary_unchecked();

        if node.operator != UnaryOperator::Typeof {
            return;
        }

        let Some(binary) = nodes.get_node(node.parent).as_binary() else {
            return;
        };

        if !matches!(
            binary.operator,
            BinaryOperator::Equality | BinaryOperator::StrictEquality
        ) {
            return;
        }

        if nodes.get_node(binary.right).as_expr().is_some() {
            let parent_id = node.parent.clone();
            let parent = nodes.get_node_mut(parent_id).as_binary_mut_unchecked();
            std::mem::swap(&mut parent.left, &mut parent.right);
        }

        self.walk_unary_expression(id, nodes);
    }
}
