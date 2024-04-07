use crate::{
    ast::{
        Statement, TraversableBinaryExpression, TraversableExpression,
        TraversableExpressionStatement, TraversableIdentifierReference, TraversableStatement,
        TraversableStringLiteral, TraversableUnaryExpression,
    },
    cell::GCell,
    context::TraverseCtx,
    node_ref, Token,
};

/// Run transform visitor on AST.
///
/// The provided transformer must implement `Traverse` and will be run on a version of the AST
/// with interior mutability, allowing traversal in any direction (up or down).
/// Once the transform is finished, caller can continue to use the standard version of the AST
/// in the usual way, without interior mutability.
pub fn transform<'a, 't, T>(transformer: &mut T, stmt: &mut Statement<'a>)
where
    't: 'a,
    T: Traverse<'a, 't>,
{
    // Generate `GhostToken` which transformer uses to access the AST.
    // SAFETY: We only create one token, and it never leaves this function.
    let mut token = unsafe { Token::new_unchecked() };

    // I'm not sure if this line is needed or not
    let token_ref: &mut Token<'t> = &mut token;
    // SAFETY: Similar to the token itself it won't leave the scope of this function.
    let token_ref: &'a mut Token<'t> = unsafe { std::mem::transmute(token_ref) };

    let mut ctx = TraverseCtx::new(token_ref);

    // Convert AST to traversable version.
    // SAFETY: `Statement` and `TraversableStatement` are mirrors of each other, with identical layouts.
    // The same is true of all child types - this is ensured by `#[repr(C)]` on all types.
    // Therefore one can safely be transmuted to the other.
    // As we hold a `&mut` reference, it's guaranteed there are no other live references.
    let stmt = unsafe { &mut *(stmt as *mut Statement<'a> as *mut TraversableStatement<'a, 't>) };
    let stmt = GCell::from_mut(stmt);

    // Run transformer on the traversable AST
    Traverse::visit_statement(transformer, stmt, &mut ctx);

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
        ctx: &TraverseCtx<'a, 't>,
    ) {
        self.walk_statement(stmt, ctx)
    }

    fn walk_statement(
        &mut self,
        stmt: node_ref!(&TraversableStatement<'a, 't>),
        ctx: &TraverseCtx<'a, 't>,
    ) {
        match ctx.get_node(stmt) {
            TraversableStatement::ExpressionStatement(expr_stmt) => {
                self.visit_expression_statement(expr_stmt, ctx)
            } // _ => {} // No other variants at present
        }
    }

    fn visit_expression_statement(
        &mut self,
        expr_stmt: node_ref!(&TraversableExpressionStatement<'a, 't>),
        ctx: &TraverseCtx<'a, 't>,
    ) {
        self.walk_expression_statement(expr_stmt, ctx);
    }

    fn walk_expression_statement(
        &mut self,
        expr_stmt: node_ref!(&TraversableExpressionStatement<'a, 't>),
        ctx: &TraverseCtx<'a, 't>,
    ) {
        self.visit_expression(&ctx.get_node(expr_stmt).expression.clone(), ctx);
    }

    fn visit_expression(
        &mut self,
        expr: &TraversableExpression<'a, 't>,
        ctx: &TraverseCtx<'a, 't>,
    ) {
        self.walk_expression(expr, ctx);
    }

    fn walk_expression(
        &mut self,
        expr: &TraversableExpression<'a, 't>,
        ctx: &TraverseCtx<'a, 't>,
    ) {
        match expr {
            TraversableExpression::Identifier(id) => {
                self.visit_identifier_reference(id, ctx);
            }
            TraversableExpression::StringLiteral(str_lit) => {
                self.visit_string_literal(str_lit, ctx);
            }
            TraversableExpression::BinaryExpression(bin_expr) => {
                self.visit_binary_expression(bin_expr, ctx);
            }
            TraversableExpression::UnaryExpression(unary_expr) => {
                self.visit_unary_expression(unary_expr, ctx);
            }
        }
    }

    #[allow(unused_variables)]
    fn visit_identifier_reference(
        &mut self,
        id: node_ref!(&TraversableIdentifierReference<'a, 't>),
        ctx: &TraverseCtx<'a, 't>,
    ) {
    }

    #[allow(unused_variables)]
    fn visit_string_literal(
        &mut self,
        str_lit: node_ref!(&TraversableStringLiteral<'a, 't>),
        ctx: &TraverseCtx<'a, 't>,
    ) {
    }

    fn visit_binary_expression(
        &mut self,
        bin_expr: node_ref!(&TraversableBinaryExpression<'a, 't>),
        ctx: &TraverseCtx<'a, 't>,
    ) {
        self.walk_binary_expression(bin_expr, ctx);
    }

    fn walk_binary_expression(
        &mut self,
        bin_expr: node_ref!(&TraversableBinaryExpression<'a, 't>),
        ctx: &TraverseCtx<'a, 't>,
    ) {
        let node = ctx.get_node(bin_expr);

        self.visit_expression(&node.left.clone(), ctx);
        self.visit_expression(&node.right.clone(), ctx);
    }

    fn visit_unary_expression(
        &mut self,
        unary_expr: node_ref!(&TraversableUnaryExpression<'a, 't>),
        ctx: &TraverseCtx<'a, 't>,
    ) {
        self.walk_unary_expression(unary_expr, ctx);
    }

    fn walk_unary_expression(
        &mut self,
        unary_expr: node_ref!(&TraversableUnaryExpression<'a, 't>),
        ctx: &TraverseCtx<'a, 't>,
    ) {
        self.visit_expression(&ctx.get_node(unary_expr).argument.clone(), ctx);
    }
}
