use crate::{
    ast::{
        BinaryExpression, Expression, ExpressionStatement, IdentifierReference, Statement,
        StringLiteral, UnaryExpression,
    },
    node_ref, Token,
};

#[allow(clippy::single_match)]
pub trait Visit<'a, 't> {
    fn visit_statement(&mut self, stmt: &Statement<'a, 't>, tk: &mut Token<'t>) {
        self.walk_statement(stmt, tk)
    }

    fn walk_statement(&mut self, stmt: &Statement<'a, 't>, tk: &mut Token<'t>) {
        match stmt {
            Statement::ExpressionStatement(expr_stmt) => {
                self.visit_expression_statement(expr_stmt, tk)
            } // _ => {} // No other variants at present
        }
    }

    fn visit_expression_statement(
        &mut self,
        expr_stmt: node_ref!(ExpressionStatement<'a, 't>),
        tk: &mut Token<'t>,
    ) {
        self.walk_expression_statement(expr_stmt, tk);
    }

    fn walk_expression_statement(
        &mut self,
        expr_stmt: node_ref!(ExpressionStatement<'a, 't>),
        tk: &mut Token<'t>,
    ) {
        self.visit_expression(&expr_stmt.borrow(tk).expression.clone(), tk);
    }

    fn visit_expression(&mut self, expr: &Expression<'a, 't>, tk: &mut Token<'t>) {
        self.walk_expression(expr, tk);
    }

    fn walk_expression(&mut self, expr: &Expression<'a, 't>, tk: &mut Token<'t>) {
        match expr {
            Expression::Identifier(id) => {
                self.visit_identifier_reference(id, tk);
            }
            Expression::StringLiteral(str_lit) => {
                self.visit_string_literal(str_lit, tk);
            }
            Expression::BinaryExpression(bin_expr) => {
                self.visit_binary_expression(bin_expr, tk);
            }
            Expression::UnaryExpression(unary_expr) => {
                self.visit_unary_expression(unary_expr, tk);
            }
        }
    }

    #[allow(unused_variables)]
    fn visit_identifier_reference(
        &mut self,
        id: node_ref!(IdentifierReference<'a, 't>),
        tk: &mut Token<'t>,
    ) {
    }

    #[allow(unused_variables)]
    fn visit_string_literal(
        &mut self,
        str_lit: node_ref!(StringLiteral<'a, 't>),
        tk: &mut Token<'t>,
    ) {
    }

    fn visit_binary_expression(
        &mut self,
        bin_expr: node_ref!(BinaryExpression<'a, 't>),
        tk: &mut Token<'t>,
    ) {
        self.walk_binary_expression(bin_expr, tk);
    }

    fn walk_binary_expression(
        &mut self,
        bin_expr: node_ref!(BinaryExpression<'a, 't>),
        tk: &mut Token<'t>,
    ) {
        self.visit_expression(&bin_expr.borrow(tk).left.clone(), tk);
        self.visit_expression(&bin_expr.borrow(tk).right.clone(), tk);
    }

    fn visit_unary_expression(
        &mut self,
        unary_expr: node_ref!(UnaryExpression<'a, 't>),
        tk: &mut Token<'t>,
    ) {
        self.walk_unary_expression(unary_expr, tk);
    }

    fn walk_unary_expression(
        &mut self,
        unary_expr: node_ref!(UnaryExpression<'a, 't>),
        tk: &mut Token<'t>,
    ) {
        self.visit_expression(&unary_expr.borrow(tk).argument.clone(), tk);
    }
}
