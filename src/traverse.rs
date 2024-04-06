use crate::{
    ast::{
        Statement, TraversableBinaryExpression, TraversableExpression,
        TraversableExpressionStatement, TraversableIdentifierReference, TraversableStatement,
        TraversableStringLiteral, TraversableUnaryExpression,
    },
    cell::{new_token_unchecked, GhostCell},
    node_ref, Token,
};

/// Run transform visitor on AST.
///
/// The provided transformer must implement `Traverse` and will be runs on a version of the AST
/// with interior mutability, allowing traversal in any direction (up or down).
/// Once the transform is finished, the AST is transmuted back to its original form.
pub fn transform<'a, 't, T>(transformer: &mut T, stmt: &mut Statement<'a>)
where
    't: 'a,
    T: Traverse<'a, 't>,
{
    // Generate `GhostToken` which transformer uses to access the AST.
    // SAFETY: We only create one token, and it never leaves this function
    let mut token: Token<'t> = unsafe { new_token_unchecked() };

    // Convert AST to traversable version.
    // SAFETY: `Statement` and `TraversableStatement` are mirrors of each other, with identical layouts.
    // The same is true of all child types - this is ensured by `#[repr(C)]` on all types.
    // Therefore the 2 can be safely transmuted to each other.
    // As we hold a `&mut` reference, it's guaranteed there are no other live references.
    let stmt = unsafe { &mut *(stmt as *mut Statement<'a> as *mut TraversableStatement<'a, 't>) };
    let stmt = GhostCell::from_mut(stmt);

    // Run transformer on the traversable AST
    Traverse::visit_statement(transformer, stmt, &mut token);

    // The access token goes out of scope at this point, which guarantees that no references
    // (either mutable or immutable) to the traversable AST or the token still exist.
    // If the transformer attempts to hold on to any references to the AST, or to the token,
    // this will produce a compile-time error.
    // Therefore, the caller can now safely continue using the `&mut Statement` that they passed in.
}

pub trait Traverse<'a, 't> {
    fn visit_statement(
        &mut self,
        stmt: node_ref!(&TraversableStatement<'a, 't>),
        tk: &mut Token<'t>,
    ) {
        self.walk_statement(stmt, tk)
    }

    fn walk_statement(
        &mut self,
        stmt: node_ref!(&TraversableStatement<'a, 't>),
        tk: &mut Token<'t>,
    ) {
        match stmt.borrow(tk) {
            TraversableStatement::ExpressionStatement(expr_stmt) => {
                self.visit_expression_statement(expr_stmt, tk)
            } // _ => {} // No other variants at present
        }
    }

    fn visit_expression_statement(
        &mut self,
        expr_stmt: node_ref!(&TraversableExpressionStatement<'a, 't>),
        tk: &mut Token<'t>,
    ) {
        self.walk_expression_statement(expr_stmt, tk);
    }

    fn walk_expression_statement(
        &mut self,
        expr_stmt: node_ref!(&TraversableExpressionStatement<'a, 't>),
        tk: &mut Token<'t>,
    ) {
        self.visit_expression(&expr_stmt.borrow(tk).expression.clone(), tk);
    }

    fn visit_expression(&mut self, expr: &TraversableExpression<'a, 't>, tk: &mut Token<'t>) {
        self.walk_expression(expr, tk);
    }

    fn walk_expression(&mut self, expr: &TraversableExpression<'a, 't>, tk: &mut Token<'t>) {
        match expr {
            TraversableExpression::Identifier(id) => {
                self.visit_identifier_reference(id, tk);
            }
            TraversableExpression::StringLiteral(str_lit) => {
                self.visit_string_literal(str_lit, tk);
            }
            TraversableExpression::BinaryExpression(bin_expr) => {
                self.visit_binary_expression(bin_expr, tk);
            }
            TraversableExpression::UnaryExpression(unary_expr) => {
                self.visit_unary_expression(unary_expr, tk);
            }
        }
    }

    #[allow(unused_variables)]
    fn visit_identifier_reference(
        &mut self,
        id: node_ref!(&TraversableIdentifierReference<'a, 't>),
        tk: &mut Token<'t>,
    ) {
    }

    #[allow(unused_variables)]
    fn visit_string_literal(
        &mut self,
        str_lit: node_ref!(&TraversableStringLiteral<'a, 't>),
        tk: &mut Token<'t>,
    ) {
    }

    fn visit_binary_expression(
        &mut self,
        bin_expr: node_ref!(&TraversableBinaryExpression<'a, 't>),
        tk: &mut Token<'t>,
    ) {
        self.walk_binary_expression(bin_expr, tk);
    }

    fn walk_binary_expression(
        &mut self,
        bin_expr: node_ref!(&TraversableBinaryExpression<'a, 't>),
        tk: &mut Token<'t>,
    ) {
        self.visit_expression(&bin_expr.borrow(tk).left.clone(), tk);
        self.visit_expression(&bin_expr.borrow(tk).right.clone(), tk);
    }

    fn visit_unary_expression(
        &mut self,
        unary_expr: node_ref!(&TraversableUnaryExpression<'a, 't>),
        tk: &mut Token<'t>,
    ) {
        self.walk_unary_expression(unary_expr, tk);
    }

    fn walk_unary_expression(
        &mut self,
        unary_expr: node_ref!(&TraversableUnaryExpression<'a, 't>),
        tk: &mut Token<'t>,
    ) {
        self.visit_expression(&unary_expr.borrow(tk).argument.clone(), tk);
    }
}
