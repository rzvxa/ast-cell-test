use crate::{
    ast::{Expression, NodeId, Statement},
    Nodes,
};

#[allow(clippy::single_match)]
pub trait Visit<'a> {
    fn visit_statement(&mut self, id: NodeId<'a>, nodes: &mut Nodes<'a>) {
        self.walk_statement(id, nodes)
    }

    fn walk_statement(&mut self, id: NodeId<'a>, nodes: &mut Nodes<'a>) {
        let node = nodes.get_node(id).as_stmt_unchecked();
        match *node {
            Statement::ExpressionStatement(expr_stmt) => self.visit_expression(expr_stmt, nodes), // _ => {} // No other variants at present
        }
    }

    fn visit_expression(&mut self, id: NodeId<'a>, nodes: &mut Nodes<'a>) {
        self.walk_expression(id, nodes);
    }

    fn walk_expression(&mut self, id: NodeId<'a>, nodes: &mut Nodes<'a>) {
        let node = nodes.get_node(id).as_expr_unchecked();
        match *node {
            Expression::Identifier(it) => {
                self.visit_identifier_reference(it, nodes);
            }
            Expression::StringLiteral(it) => {
                self.visit_string_literal(it, nodes);
            }
            Expression::BinaryExpression(it) => {
                self.visit_binary_expression(it, nodes);
            }
            Expression::UnaryExpression(it) => {
                self.visit_unary_expression(it, nodes);
            }
        }
    }

    #[allow(unused_variables)]
    fn visit_identifier_reference(&mut self, id: NodeId<'a>, nodes: &mut Nodes<'a>) {}

    #[allow(unused_variables)]
    fn visit_string_literal(&mut self, id: NodeId<'a>, nodes: &mut Nodes<'a>) {}

    fn visit_binary_expression(&mut self, id: NodeId<'a>, nodes: &mut Nodes<'a>) {
        self.walk_binary_expression(id, nodes);
    }

    fn walk_binary_expression(&mut self, id: NodeId<'a>, nodes: &mut Nodes<'a>) {
        let node = nodes.get_node(id).as_binary_unchecked();
        let left = node.left.clone();
        let right = node.right.clone();
        self.visit_expression(left, nodes);
        self.visit_expression(right, nodes);
    }

    fn visit_unary_expression(&mut self, id: NodeId<'a>, nodes: &mut Nodes<'a>) {
        self.walk_unary_expression(id, nodes);
    }

    fn walk_unary_expression(&mut self, id: NodeId<'a>, nodes: &mut Nodes<'a>) {
        let node = nodes.get_node(id).as_unary_unchecked();
        self.visit_expression(node.argument, nodes);
    }
}
