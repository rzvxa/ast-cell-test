use oxc_allocator::Allocator;

mod ast;
mod print;
mod visit;
use ast::{
    AsAstKind, AstKind, BinaryOperator, Expression, IdentifierReference, NodeId, Statement,
    StringLiteral, UnaryExpression, UnaryOperator,
};
use print::Printer;
use visit::Visit;

use crate::ast::BinaryExpression;

type Nodes<'a> = Vec<AstKind<'a>>;

fn main() {
    let alloc = Allocator::default();
    let (mut nodes, stmt) = parse(&alloc);
    println!("before: {}", Printer::print(stmt, &mut nodes));
    TransformTypeof.visit_statement(stmt, &mut nodes);
    println!("after: {}", Printer::print(stmt, &mut nodes));
}

/// Create AST for `typeof foo === 'object'`.
/// Hard-coded here, but these are the steps actual parser would take to create the AST
/// with "back-links" to parents on each node.
fn parse<'a, 't>(alloc: &'a Allocator) -> (Vec<AstKind<'a>>, NodeId<'a>) {
    let mut nodes = Vec::with_capacity(5);
    macro_rules! push {
        ($expr:expr) => {{
            nodes.push(alloc.alloc($expr).as_ast_kind());
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
    nodes[ident_id.as_index()].as_ident_mut_or_panic().parent = unary_id;

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

    nodes[unary_id.as_index()].as_unary_mut_or_panic().parent = binary_id;
    nodes[str_id.as_index()].as_str_mut_or_panic().parent = binary_id;

    // `typeof foo === 'object'` (as expression)
    let expr_id = push!(Expression::BinaryExpression(binary_id));
    nodes[binary_id.as_index()].as_binary_mut_or_panic().parent = expr_id;

    // `typeof foo === 'object'` (as statement)
    let stmt_id = push!(Statement::ExpressionStatement(expr_id));

    (nodes, stmt_id)
}

/// Transformer for `typeof x === 'y'` to `'y' === typeof x`
struct TransformTypeof;

impl<'a> Visit<'a> for TransformTypeof {
    fn visit_unary_expression(&mut self, id: NodeId<'a>, nodes: &mut Nodes<'a>) {
        let Some(node) = nodes[id.as_index()].as_unary() else {
            unreachable!()
        };

        if node.operator == UnaryOperator::Typeof {
            let parent = &nodes[node.parent.as_index()];
            if let Some(binary) = parent.as_binary() {
                if matches!(
                    binary.operator,
                    BinaryOperator::Equality | BinaryOperator::StrictEquality
                ) {
                    let Some(Expression::StringLiteral(_)) =
                        nodes[binary.right.as_index()].as_expr()
                    else {
                        unreachable!()
                    };
                    let parent_id = node.parent.clone();
                    let parent = nodes[parent_id.as_index()].as_binary_mut_or_panic();
                    std::mem::swap(&mut parent.left, &mut parent.right);
                }
            }
        }

        self.walk_unary_expression(id, nodes);
    }
}
