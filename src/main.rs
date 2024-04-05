use ghost_cell::{GhostCell, GhostToken};
use oxc_allocator::Allocator;

mod ast;
mod print;
mod visit;
use ast::{
    BinaryExpression, BinaryOperator, Expression, ExpressionParent, ExpressionStatement,
    IdentifierReference, Statement, StringLiteral, UnaryExpression, UnaryOperator,
};
use print::Printer;
use visit::Visit;

pub trait GhostAlloc {
    fn galloc<'t, T>(&self, t: T) -> &GhostCell<'t, T>;
}

impl GhostAlloc for Allocator {
    /// Allocate `T` into arena and return a `&GhostCell` containing it
    fn galloc<'t, T>(&self, t: T) -> &GhostCell<'t, T> {
        GhostCell::from_mut(self.alloc(t))
    }
}

/// Macro to reduce boilerplate of `GhostCell` references.
/// `node_ref!(ExpressionStatement<'a, 't>)` -> `&'a GhostCell<'t, ExpressionStatement<'a, 't>>`
macro_rules! node_ref {
    ($ty:ident<$arena:lifetime, $token:lifetime>) => {
        &$arena ghost_cell::GhostCell<$token, $ty<$arena, $token>>
    };
}
pub(crate) use node_ref;

fn main() {
    let alloc = Allocator::default();
    GhostToken::new(|mut token| {
        let stmt = parse(&alloc, &mut token);
        println!("before: {}", Printer::print(&stmt, &mut token));
        TransformTypeof.visit_statement(&stmt, &mut token);
        println!("after: {}", Printer::print(&stmt, &mut token));
    });
}

/// Create AST for `typeof foo === 'object'`.
/// Hard-coded here, but these are the steps actual parser would take to create the AST
/// with "back-links" to parents on each node.
fn parse<'a, 't>(alloc: &'a Allocator, token: &mut GhostToken<'t>) -> Statement<'a, 't> {
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
    id.borrow_mut(token).parent = ExpressionParent::UnaryExpression(unary_expr);

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
    unary_expr.borrow_mut(token).parent = ExpressionParent::BinaryExpressionLeft(binary_expr);
    str_lit.borrow_mut(token).parent = ExpressionParent::BinaryExpressionRight(binary_expr);

    // `typeof foo === 'object'` (as expression statement)
    let expr_stmt = alloc.galloc(ExpressionStatement {
        expression: Expression::BinaryExpression(binary_expr),
    });
    binary_expr.borrow_mut(token).parent = ExpressionParent::ExpressionStatement(expr_stmt);

    // `typeof foo === 'object'` (as statement)
    Statement::ExpressionStatement(expr_stmt)
}

/// Transformer for `typeof x === 'y'` to `'y' === typeof x`
struct TransformTypeof;

impl<'a, 't> Visit<'a, 't> for TransformTypeof {
    fn visit_unary_expression(
        &mut self,
        unary_expr: node_ref!(UnaryExpression<'a, 't>),
        token: &mut GhostToken<'t>,
    ) {
        self.walk_unary_expression(unary_expr, token);

        if unary_expr.borrow(token).operator == UnaryOperator::Typeof {
            if let ExpressionParent::BinaryExpressionLeft(bin_expr) =
                unary_expr.borrow(token).parent
            {
                if matches!(
                    bin_expr.borrow(token).operator,
                    BinaryOperator::Equality | BinaryOperator::StrictEquality
                ) && matches!(bin_expr.borrow(token).right, Expression::StringLiteral(_))
                {
                    let parent = bin_expr.borrow_mut(token);
                    std::mem::swap(&mut parent.left, &mut parent.right);
                }
            }
        }
    }
}
