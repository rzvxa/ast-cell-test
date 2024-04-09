use crate::ast::{BinaryOperator, NodeId, UnaryOperator};
use crate::{Nodes, Visit};

pub struct Printer {
    output: String,
}

impl Printer {
    pub fn print<'a>(stmt: NodeId<'a>, nodes: &mut Nodes<'a>) -> String {
        let mut printer = Printer {
            output: String::new(),
        };
        printer.visit_statement(stmt, nodes);
        printer.output
    }

    fn output(&mut self, s: &str) {
        self.output.push_str(s);
    }
}

impl<'a> Visit<'a> for Printer {
    fn visit_identifier_reference(&mut self, id: NodeId<'a>, nodes: &mut Nodes<'a>) {
        let Some(node) = nodes[id.as_index()].as_ident() else {
            unreachable!()
        };
        self.output(node.name);
    }

    fn visit_string_literal(&mut self, id: NodeId<'a>, nodes: &mut Nodes<'a>) {
        let Some(node) = nodes[id.as_index()].as_str() else {
            unreachable!()
        };
        self.output(&format!("'{}'", node.value));
    }

    fn visit_unary_expression(&mut self, id: NodeId<'a>, nodes: &mut Nodes<'a>) {
        let Some(node) = nodes[id.as_index()].as_unary() else {
            unreachable!()
        };
        match node.operator {
            UnaryOperator::UnaryNegation => self.output("-"),
            UnaryOperator::UnaryPlus => self.output("+"),
            UnaryOperator::LogicalNot => self.output("!"),
            UnaryOperator::BitwiseNot => self.output("~"),
            UnaryOperator::Typeof => self.output("typeof "),
            UnaryOperator::Void => self.output("void "),
            UnaryOperator::Delete => self.output("delete "),
        }
        self.visit_expression(node.argument, nodes);
    }

    fn visit_binary_expression(&mut self, id: NodeId<'a>, nodes: &mut Nodes<'a>) {
        {
            // scope
            let Some(node) = nodes[id.as_index()].as_binary() else {
                unreachable!()
            };
            self.visit_expression(node.left, nodes);
        }

        let Some(node) = nodes[id.as_index()].as_binary() else {
            unreachable!()
        };
        self.output(&format!(
            " {} ",
            match node.operator {
                BinaryOperator::Equality => "==",
                BinaryOperator::StrictEquality => "===",
            }
        ));

        self.visit_expression(node.right, nodes);
    }
}
