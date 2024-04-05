#![allow(dead_code, clippy::enum_variant_names)]

use crate::node_ref;

#[derive(Clone)]
pub enum Statement<'a, 't> {
    ExpressionStatement(node_ref!(ExpressionStatement<'a, 't>)),
}

#[derive(Clone)]
pub struct ExpressionStatement<'a, 't> {
    pub expression: Expression<'a, 't>,
}

#[derive(Clone)]
pub enum Expression<'a, 't> {
    StringLiteral(node_ref!(StringLiteral<'a, 't>)),
    Identifier(node_ref!(IdentifierReference<'a, 't>)),
    BinaryExpression(node_ref!(BinaryExpression<'a, 't>)),
    UnaryExpression(node_ref!(UnaryExpression<'a, 't>)),
}

#[derive(Clone)]
pub enum ExpressionParent<'a, 't> {
    None,
    ExpressionStatement(node_ref!(ExpressionStatement<'a, 't>)),
    BinaryExpressionLeft(node_ref!(BinaryExpression<'a, 't>)),
    BinaryExpressionRight(node_ref!(BinaryExpression<'a, 't>)),
    UnaryExpression(node_ref!(UnaryExpression<'a, 't>)),
}

#[derive(Clone)]
pub struct IdentifierReference<'a, 't> {
    pub name: &'a str,
    pub parent: ExpressionParent<'a, 't>,
}

#[derive(Clone)]
pub struct StringLiteral<'a, 't> {
    pub value: &'a str,
    pub parent: ExpressionParent<'a, 't>,
}

#[derive(Clone)]
pub struct BinaryExpression<'a, 't> {
    pub left: Expression<'a, 't>,
    pub operator: BinaryOperator,
    pub right: Expression<'a, 't>,
    pub parent: ExpressionParent<'a, 't>,
}

#[derive(Clone, Copy)]
pub enum BinaryOperator {
    Equality,
    StrictEquality,
}

#[derive(Clone)]
pub struct UnaryExpression<'a, 't> {
    pub operator: UnaryOperator,
    pub argument: Expression<'a, 't>,
    pub parent: ExpressionParent<'a, 't>,
}

#[derive(Clone, Copy, PartialEq)]
pub enum UnaryOperator {
    UnaryNegation,
    UnaryPlus,
    LogicalNot,
    BitwiseNot,
    Typeof,
    Void,
    Delete,
}
