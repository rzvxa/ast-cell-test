#![allow(dead_code, clippy::enum_variant_names)]

//! This file defines 2 different versions of the AST:
//!
//! 1. Standard version - using `Box<'a, T>` for references between types.
//! 2. Traversable version - identical, except references between types are `SharedBox<'a, 't, T>`.
//!
//! The difference between the two is that the traversable version features interior mutability
//! (via `GCell`). So the traversable AST can be mutated with just an immutable `&` reference.
//! It can also be traversed in any direction (up or down).
//!
//! To avoid an expensive conversion process between the two AST versions, they are laid out in memory
//! exactly the same, and one can be transmuted to the other at zero cost.
//!
//! # SAFETY
//! The size, alignment, and layout of all AST node types and their "traversable" counterparts
//! must be identical, so that transmuting `Statement` to `TraversableStatement` is sound.
//! All types must be `#[repr(C)]` to ensure predictable type layouts.
//! All enums must be `#[repr(C, u8)]` with explicit discriminants to ensure discriminants
//! match between the "standard" and "traversable" types.

// TODO: Create the "Traversable" types with a macro to ensure they cannot be out of sync,
// and apply `#[repr(C)]` (for structs) / `#[repr(C, u8)]` (for enums) programmatically,
// so can't get forgotten.

use oxc_allocator::{Box, Vec};

use crate::cell::{shared_box, shared_vec};

/// Macro to assert equivalence in size and alignment between standard and traversable types
macro_rules! assert_size_align_match {
    ($standard:ident, $traversable:ident) => {
        const _: () = {
            use std::mem::{align_of, size_of};
            assert!(size_of::<$standard>() == size_of::<$traversable>());
            assert!(align_of::<$standard>() == align_of::<$traversable>());
            assert!(size_of::<Box<$standard>>() == size_of::<&crate::cell::GCell<$traversable>>());
            assert!(
                align_of::<Box<$standard>>() == align_of::<&crate::cell::GCell<$traversable>>()
            );
        };
    };
}

#[derive(Debug)]
#[repr(C)]
pub struct Program<'a> {
    pub body: Vec<'a, Statement<'a>>,
}

#[repr(C)]
pub struct TraversableProgram<'a, 't> {
    pub body: shared_vec!(TraversableStatement<'a, 't>),
}

assert_size_align_match!(Program, TraversableProgram);

#[derive(Debug)]
#[repr(C, u8)]
pub enum Statement<'a> {
    ExpressionStatement(Box<'a, ExpressionStatement<'a>>) = 0,
}

#[derive(Clone)]
#[repr(C, u8)]
pub enum TraversableStatement<'a, 't> {
    ExpressionStatement(shared_box!(TraversableExpressionStatement<'a, 't>)) = 0,
}

assert_size_align_match!(Statement, TraversableStatement);

#[derive(Clone, Copy, Debug)]
#[repr(C, u8)]
pub enum StatementParent<'a> {
    None = 0,
    Program(*const Program<'a>) = 1,
}

#[derive(Clone, Copy)]
#[repr(C, u8)]
pub enum TraversableStatementParent<'a, 't> {
    None = 0,
    Program(shared_box!(TraversableProgram<'a, 't>)) = 1,
}

assert_size_align_match!(StatementParent, TraversableStatementParent);

#[derive(Debug)]
#[repr(C)]
pub struct ExpressionStatement<'a> {
    pub expression: Expression<'a>,
    pub parent: StatementParent<'a>,
}

#[derive(Clone)]
#[repr(C)]
pub struct TraversableExpressionStatement<'a, 't> {
    pub expression: TraversableExpression<'a, 't>,
    pub parent: TraversableStatementParent<'a, 't>,
}

assert_size_align_match!(ExpressionStatement, TraversableExpressionStatement);

#[derive(Debug)]
#[repr(C, u8)]
pub enum Expression<'a> {
    StringLiteral(Box<'a, StringLiteral<'a>>) = 0,
    Identifier(Box<'a, IdentifierReference<'a>>) = 1,
    BinaryExpression(Box<'a, BinaryExpression<'a>>) = 2,
    UnaryExpression(Box<'a, UnaryExpression<'a>>) = 3,
}

#[derive(Clone)]
#[repr(C, u8)]
pub enum TraversableExpression<'a, 't> {
    StringLiteral(shared_box!(TraversableStringLiteral<'a, 't>)) = 0,
    Identifier(shared_box!(TraversableIdentifierReference<'a, 't>)) = 1,
    BinaryExpression(shared_box!(TraversableBinaryExpression<'a, 't>)) = 2,
    UnaryExpression(shared_box!(TraversableUnaryExpression<'a, 't>)) = 3,
}

assert_size_align_match!(Expression, TraversableExpression);

#[derive(Clone, Copy, Debug)]
#[repr(C, u8)]
pub enum ExpressionParent<'a> {
    None = 0,
    ExpressionStatement(*const ExpressionStatement<'a>) = 1,
    BinaryExpressionLeft(*const BinaryExpression<'a>) = 2,
    BinaryExpressionRight(*const BinaryExpression<'a>) = 3,
    UnaryExpression(*const UnaryExpression<'a>) = 4,
}

#[derive(Clone, Copy)]
#[repr(C, u8)]
pub enum TraversableExpressionParent<'a, 't> {
    None = 0,
    ExpressionStatement(shared_box!(TraversableExpressionStatement<'a, 't>)) = 1,
    BinaryExpressionLeft(shared_box!(TraversableBinaryExpression<'a, 't>)) = 2,
    BinaryExpressionRight(shared_box!(TraversableBinaryExpression<'a, 't>)) = 3,
    UnaryExpression(shared_box!(TraversableUnaryExpression<'a, 't>)) = 4,
}

assert_size_align_match!(ExpressionParent, TraversableExpressionParent);

#[derive(Debug)]
#[repr(C)]
pub struct IdentifierReference<'a> {
    pub name: &'a str,
    pub parent: ExpressionParent<'a>,
}

#[derive(Clone)]
#[repr(C)]
pub struct TraversableIdentifierReference<'a, 't> {
    pub name: &'a str,
    pub parent: TraversableExpressionParent<'a, 't>,
}

assert_size_align_match!(IdentifierReference, TraversableIdentifierReference);

#[derive(Debug)]
#[repr(C)]
pub struct StringLiteral<'a> {
    pub value: &'a str,
    pub parent: ExpressionParent<'a>,
}

#[derive(Clone)]
#[repr(C)]
pub struct TraversableStringLiteral<'a, 't> {
    pub value: &'a str,
    pub parent: TraversableExpressionParent<'a, 't>,
}

assert_size_align_match!(StringLiteral, TraversableStringLiteral);

#[derive(Debug)]
#[repr(C)]
pub struct BinaryExpression<'a> {
    pub left: Expression<'a>,
    pub operator: BinaryOperator,
    pub right: Expression<'a>,
    pub parent: ExpressionParent<'a>,
}

#[derive(Clone)]
#[repr(C)]
pub struct TraversableBinaryExpression<'a, 't> {
    pub left: TraversableExpression<'a, 't>,
    pub operator: BinaryOperator,
    pub right: TraversableExpression<'a, 't>,
    pub parent: TraversableExpressionParent<'a, 't>,
}

assert_size_align_match!(BinaryExpression, TraversableBinaryExpression);

#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(u8)]
pub enum BinaryOperator {
    Equality = 0,
    StrictEquality = 1,
}

#[derive(Debug)]
#[repr(C)]
pub struct UnaryExpression<'a> {
    pub operator: UnaryOperator,
    pub argument: Expression<'a>,
    pub parent: ExpressionParent<'a>,
}

#[derive(Clone)]
#[repr(C)]
pub struct TraversableUnaryExpression<'a, 't> {
    pub operator: UnaryOperator,
    pub argument: TraversableExpression<'a, 't>,
    pub parent: TraversableExpressionParent<'a, 't>,
}

assert_size_align_match!(UnaryExpression, TraversableUnaryExpression);

#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(u8)]
pub enum UnaryOperator {
    UnaryNegation = 0,
    UnaryPlus = 1,
    LogicalNot = 2,
    BitwiseNot = 3,
    Typeof = 4,
    Void = 5,
    Delete = 6,
}

pub mod traversable {
    pub type Program<'a, 't> = super::TraversableProgram<'a, 't>;
    pub type Statement<'a, 't> = super::TraversableStatement<'a, 't>;
    pub type ExpressionStatement<'a, 't> = super::TraversableExpressionStatement<'a, 't>;
    pub type Expression<'a, 't> = super::TraversableExpression<'a, 't>;
    pub type ExpressionParent<'a, 't> = super::TraversableExpressionParent<'a, 't>;
    pub type IdentifierReference<'a, 't> = super::TraversableIdentifierReference<'a, 't>;
    pub type StringLiteral<'a, 't> = super::TraversableStringLiteral<'a, 't>;
    pub type BinaryExpression<'a, 't> = super::TraversableBinaryExpression<'a, 't>;
    pub type UnaryExpression<'a, 't> = super::TraversableUnaryExpression<'a, 't>;
}
