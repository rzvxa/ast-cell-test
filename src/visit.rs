use ghost_cell::GhostToken;

use crate::ast::{
    BinaryExpression, Expression, ExpressionStatement, IdentifierReference, Statement,
    StringLiteral, UnaryExpression,
};
use crate::node_ref;

#[allow(clippy::single_match)]
pub trait Visit<'a, 't> {
    fn visit_statement(&mut self, stmt: &Statement<'a, 't>, token: &mut GhostToken<'t>) {
        self.walk_statement(stmt, token)
    }

    fn walk_statement(&mut self, stmt: &Statement<'a, 't>, token: &mut GhostToken<'t>) {
        match stmt {
            Statement::ExpressionStatement(expr_stmt) => {
                self.visit_expression_statement(expr_stmt, token)
            } // _ => {} // No other variants at present
        }
    }

    fn visit_expression_statement(
        &mut self,
        expr_stmt: node_ref!(ExpressionStatement<'a, 't>),
        token: &mut GhostToken<'t>,
    ) {
        self.walk_expression_statement(expr_stmt, token);
    }

    fn walk_expression_statement(
        &mut self,
        expr_stmt: node_ref!(ExpressionStatement<'a, 't>),
        token: &mut GhostToken<'t>,
    ) {
        self.visit_expression(&expr_stmt.borrow(token).expression.clone(), token);
    }

    fn visit_expression(&mut self, expr: &Expression<'a, 't>, token: &mut GhostToken<'t>) {
        self.walk_expression(expr, token);
    }

    fn walk_expression(&mut self, expr: &Expression<'a, 't>, token: &mut GhostToken<'t>) {
        match expr {
            Expression::Identifier(id) => {
                self.visit_identifier_reference(id, token);
            }
            Expression::StringLiteral(str_lit) => {
                self.visit_string_literal(str_lit, token);
            }
            Expression::BinaryExpression(bin_expr) => {
                self.visit_binary_expression(bin_expr, token);
            }
            Expression::UnaryExpression(unary_expr) => {
                self.visit_unary_expression(unary_expr, token);
            }
        }
    }

    #[allow(unused_variables)]
    fn visit_identifier_reference(
        &mut self,
        id: node_ref!(IdentifierReference<'a, 't>),
        token: &mut GhostToken<'t>,
    ) {
    }

    #[allow(unused_variables)]
    fn visit_string_literal(
        &mut self,
        str_lit: node_ref!(StringLiteral<'a, 't>),
        token: &mut GhostToken<'t>,
    ) {
    }

    fn visit_binary_expression(
        &mut self,
        bin_expr: node_ref!(BinaryExpression<'a, 't>),
        token: &mut GhostToken<'t>,
    ) {
        self.walk_binary_expression(bin_expr, token);
    }

    fn walk_binary_expression(
        &mut self,
        bin_expr: node_ref!(BinaryExpression<'a, 't>),
        token: &mut GhostToken<'t>,
    ) {
        self.visit_expression(&bin_expr.borrow(token).left.clone(), token);
        self.visit_expression(&bin_expr.borrow(token).right.clone(), token);
    }

    fn visit_unary_expression(
        &mut self,
        unary_expr: node_ref!(UnaryExpression<'a, 't>),
        token: &mut GhostToken<'t>,
    ) {
        self.walk_unary_expression(unary_expr, token);
    }

    fn walk_unary_expression(
        &mut self,
        unary_expr: node_ref!(UnaryExpression<'a, 't>),
        token: &mut GhostToken<'t>,
    ) {
        self.visit_expression(&unary_expr.borrow(token).argument.clone(), token);
    }
}
