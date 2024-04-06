use oxc_allocator::Allocator;

mod ast;
mod cell;
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

    let stmt = transform(&mut TransformTypeof, stmt);
    println!("after: {}", Printer::print(stmt));
}

/// Transformer for `typeof x === 'y'` to `'y' === typeof x`
struct TransformTypeof;

impl<'a, 't> Traverse<'a, 't> for TransformTypeof {
    fn visit_unary_expression(
        &mut self,
        unary_expr: node_ref!(&UnaryExpression<'a, 't>),
        tk: &mut Token<'t>,
    ) {
        self.walk_unary_expression(unary_expr, tk);

        if unary_expr.borrow(tk).operator == UnaryOperator::Typeof {
            if let ExpressionParent::BinaryExpressionLeft(bin_expr) = unary_expr.borrow(tk).parent {
                if matches!(
                    bin_expr.borrow(tk).operator,
                    BinaryOperator::Equality | BinaryOperator::StrictEquality
                ) {
                    if let Expression::StringLiteral(str_lit) = bin_expr.borrow(tk).right {
                        // Swap left and right of binary expression
                        let bin_expr_mut = bin_expr.borrow_mut(tk);
                        std::mem::swap(&mut bin_expr_mut.left, &mut bin_expr_mut.right);

                        // Update parent links of left and right
                        let temp = str_lit.borrow(tk).parent;
                        str_lit.borrow_mut(tk).parent = unary_expr.borrow(tk).parent;
                        unary_expr.borrow_mut(tk).parent = temp;
                    }
                }
            }
        }
    }
}
