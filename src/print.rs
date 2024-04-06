use crate::{
    ast::{
        BinaryExpression, BinaryOperator, IdentifierReference, Statement, StringLiteral,
        UnaryExpression, UnaryOperator,
    },
    node_ref, Token, Visit,
};

pub struct Printer {
    output: String,
}

impl Printer {
    pub fn print<'t>(stmt: &Statement<'_, 't>, tk: &mut Token<'t>) -> String {
        let mut printer = Printer {
            output: String::new(),
        };
        printer.visit_statement(stmt, tk);
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
        tk: &mut Token<'t>,
    ) {
        self.output(id.borrow(tk).name);
    }

    fn visit_string_literal(
        &mut self,
        str_lit: node_ref!(StringLiteral<'a, 't>),
        tk: &mut Token<'t>,
    ) {
        self.output(&format!("'{}'", str_lit.borrow(tk).value));
    }

    fn visit_unary_expression(
        &mut self,
        unary_expr: node_ref!(UnaryExpression<'a, 't>),
        tk: &mut Token<'t>,
    ) {
        match unary_expr.borrow(tk).operator {
            UnaryOperator::UnaryNegation => self.output("-"),
            UnaryOperator::UnaryPlus => self.output("+"),
            UnaryOperator::LogicalNot => self.output("!"),
            UnaryOperator::BitwiseNot => self.output("~"),
            UnaryOperator::Typeof => self.output("typeof "),
            UnaryOperator::Void => self.output("void "),
            UnaryOperator::Delete => self.output("delete "),
        }
        self.visit_expression(&unary_expr.borrow(tk).argument.clone(), tk);
    }

    fn visit_binary_expression(
        &mut self,
        bin_expr: node_ref!(BinaryExpression<'a, 't>),
        tk: &mut Token<'t>,
    ) {
        self.visit_expression(&bin_expr.borrow(tk).left.clone(), tk);
        self.output(&format!(
            " {} ",
            match bin_expr.borrow(tk).operator {
                BinaryOperator::Equality => "==",
                BinaryOperator::StrictEquality => "===",
            }
        ));
        self.visit_expression(&bin_expr.borrow(tk).right.clone(), tk);
    }
}
