use ghost_cell::GhostToken;

use crate::ast::{
    BinaryExpression, BinaryOperator, IdentifierReference, Statement, StringLiteral,
    UnaryExpression, UnaryOperator,
};
use crate::node_ref;
use crate::Visit;

pub struct Printer {
    output: String,
}

impl Printer {
    pub fn print<'t>(stmt: &Statement<'_, 't>, token: &mut GhostToken<'t>) -> String {
        let mut printer = Printer {
            output: String::new(),
        };
        printer.visit_statement(stmt, token);
        printer.output
    }

    fn output(&mut self, s: &str) {
        self.output.push_str(s);
    }
}

impl<'a, 't> Visit<'a, 't> for Printer {
    fn visit_identifier_reference(
        &mut self,
        id: node_ref!(IdentifierReference<'a, 't>),
        token: &mut GhostToken<'t>,
    ) {
        self.output(id.borrow(token).name);
    }

    fn visit_string_literal(
        &mut self,
        str_lit: node_ref!(StringLiteral<'a, 't>),
        token: &mut GhostToken<'t>,
    ) {
        self.output(&format!("'{}'", str_lit.borrow(token).value));
    }

    fn visit_unary_expression(
        &mut self,
        unary_expr: node_ref!(UnaryExpression<'a, 't>),
        token: &mut GhostToken<'t>,
    ) {
        match unary_expr.borrow(token).operator {
            UnaryOperator::UnaryNegation => self.output("-"),
            UnaryOperator::UnaryPlus => self.output("+"),
            UnaryOperator::LogicalNot => self.output("!"),
            UnaryOperator::BitwiseNot => self.output("~"),
            UnaryOperator::Typeof => self.output("typeof "),
            UnaryOperator::Void => self.output("void "),
            UnaryOperator::Delete => self.output("delete "),
        }
        self.visit_expression(&unary_expr.borrow(token).argument.clone(), token);
    }

    fn visit_binary_expression(
        &mut self,
        bin_expr: node_ref!(BinaryExpression<'a, 't>),
        token: &mut GhostToken<'t>,
    ) {
        self.visit_expression(&bin_expr.borrow(token).left.clone(), token);
        self.output(&format!(
            " {} ",
            match bin_expr.borrow(token).operator {
                BinaryOperator::Equality => "==",
                BinaryOperator::StrictEquality => "===",
            }
        ));
        self.visit_expression(&bin_expr.borrow(token).right.clone(), token);
    }
}
