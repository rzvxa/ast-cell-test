use context::TraverseCtx;
use oxc_allocator::Allocator;

mod ast;
mod cell;
mod context;
mod parser;
mod print;
mod traverse;
mod visit;
use ast::{
    BinaryOperator, TraversableExpression as Expression,
    TraversableExpressionParent as ExpressionParent, TraversableUnaryExpression as UnaryExpression,
    UnaryOperator,
};
use cell::{node_ref, Token};
use print::Printer;
use traverse::{transform, Traverse};
use visit::Visit;

// TODO: Implement semantic as a `Traverse` to set parents on nodes, rather than doing it in parser.
// TODO: Make `parent` fields inaccessible in standard AST, so user cannot alter them.

fn main() {
    let alloc = Allocator::default();
    let stmt = alloc.alloc(parser::parse(&alloc));
    println!("before: {}", Printer::print(stmt));

    transform(&mut TransformTypeof, stmt);
    println!("after: {}", Printer::print(stmt));
}

/// Transformer for `typeof x === 'y'` to `'y' === typeof x`
struct TransformTypeof;

impl<'a, 't> Traverse<'a, 't> for TransformTypeof {
    fn visit_unary_expression(
        &mut self,
        unary_expr: node_ref!(&UnaryExpression<'a, 't>),
        ctx: &TraverseCtx<'a, 't>,
    ) {
        let node = ctx.get_node(unary_expr);
        if node.operator == UnaryOperator::Typeof {
            if let ExpressionParent::BinaryExpressionLeft(bin_expr_ref) = node.parent {
                let bin_expr = ctx.get_node(bin_expr_ref);
                if matches!(
                    bin_expr.operator,
                    BinaryOperator::Equality | BinaryOperator::StrictEquality
                ) {
                    if let Expression::StringLiteral(str_lit) = bin_expr.right {
                        // Swap left and right of binary expression
                        let bin_expr_mut = ctx.get_node_mut(bin_expr_ref);
                        std::mem::swap(&mut bin_expr_mut.left, &mut bin_expr_mut.right);

                        // Update parent links of left and right
                        let temp = ctx.get_node(str_lit).parent;
                        ctx.get_node_mut(str_lit).parent = node.parent;
                        ctx.get_node_mut(unary_expr).parent = temp;
                    }
                }
            }
        }

        self.walk_unary_expression(unary_expr, ctx);
    }
}
