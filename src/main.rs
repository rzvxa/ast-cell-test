use oxc_allocator::Allocator;

mod ast;
mod cell;
mod print;
mod visit;
use ast::{
    BinaryExpression, BinaryOperator, Expression, ExpressionParent, ExpressionStatement,
    IdentifierReference, Statement, StringLiteral, UnaryExpression, UnaryOperator,
};
use cell::{node_ref, GhostAlloc, Token};
use print::Printer;
use visit::Visit;

fn main() {
    let alloc = Allocator::default();
    Token::new(|mut tk| {
        let stmt = parse(&alloc, &mut tk);
        println!("before: {}", Printer::print(&stmt, &mut tk));
        TransformTypeof.visit_statement(&stmt, &mut tk);
        println!("after: {}", Printer::print(&stmt, &mut tk));
    });
}

/// Create AST for `typeof foo === 'object'`.
/// Hard-coded here, but these are the steps actual parser would take to create the AST
/// with "back-links" to parents on each node.
fn parse<'a, 't>(alloc: &'a Allocator, tk: &mut Token<'t>) -> Statement<'a, 't> {
    // `foo`
    let id = alloc.galloc(IdentifierReference {
        name: "foo",
        parent: ExpressionParent::None,
    });

    // `typeof foo`
    let unary_expr = alloc.galloc(UnaryExpression {
        operator: UnaryOperator::Typeof,
        argument: Expression::Identifier(id),
        parent: ExpressionParent::None,
    });
    id.borrow_mut(tk).parent = ExpressionParent::UnaryExpression(unary_expr);

    // `'object'`
    let str_lit = alloc.galloc(StringLiteral {
        value: "object",
        parent: ExpressionParent::None,
    });

    // `typeof foo === 'object'` (as expression)
    let binary_expr = alloc.galloc(BinaryExpression {
        operator: BinaryOperator::StrictEquality,
        left: Expression::UnaryExpression(unary_expr),
        right: Expression::StringLiteral(str_lit),
        parent: ExpressionParent::None,
    });
    unary_expr.borrow_mut(tk).parent = ExpressionParent::BinaryExpressionLeft(binary_expr);
    str_lit.borrow_mut(tk).parent = ExpressionParent::BinaryExpressionRight(binary_expr);

    // `typeof foo === 'object'` (as expression statement)
    let expr_stmt = alloc.galloc(ExpressionStatement {
        expression: Expression::BinaryExpression(binary_expr),
    });
    binary_expr.borrow_mut(tk).parent = ExpressionParent::ExpressionStatement(expr_stmt);

    // `typeof foo === 'object'` (as statement)
    Statement::ExpressionStatement(expr_stmt)
}

/// Transformer for `typeof x === 'y'` to `'y' === typeof x`
struct TransformTypeof;

impl<'a, 't> Visit<'a, 't> for TransformTypeof {
    fn visit_unary_expression(
        &mut self,
        unary_expr: node_ref!(UnaryExpression<'a, 't>),
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
