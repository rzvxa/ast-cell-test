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
        let node = nodes[id.as_index()].as_ident_unchecked();
        self.output(node.name);
    }

    fn visit_string_literal(&mut self, id: NodeId<'a>, nodes: &mut Nodes<'a>) {
        let node = nodes[id.as_index()].as_str_unchecked();
        self.output(&format!("'{}'", node.value));
    }

    fn visit_unary_expression(&mut self, id: NodeId<'a>, nodes: &mut Nodes<'a>) {
        let node = nodes[id.as_index()].as_unary_unchecked();
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
            let node = nodes[id.as_index()].as_binary_unchecked();
            self.visit_expression(node.left, nodes);
        }

        let node = nodes[id.as_index()].as_binary_unchecked();
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
