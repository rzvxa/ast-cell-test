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

use oxc_allocator::Box;

use crate::cell::shared_box;

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

const _: () = assert_eq_size_align::<Statement, TraversableStatement>();
const _: () = assert_box_and_cell_swappable::<Statement>();

#[derive(Debug)]
#[repr(C)]
pub struct ExpressionStatement<'a> {
    pub expression: Expression<'a>,
}

#[derive(Clone)]
#[repr(C)]
pub struct TraversableExpressionStatement<'a, 't> {
    pub expression: TraversableExpression<'a, 't>,
}

const _: () = assert_eq_size_align::<ExpressionStatement, TraversableExpressionStatement>();
const _: () = assert_box_and_cell_swappable::<ExpressionStatement>();

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

const _: () = assert_eq_size_align::<Expression, TraversableExpression>();
const _: () = assert_box_and_cell_swappable::<Expression>();

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

const _: () = assert_eq_size_align::<ExpressionParent, TraversableExpressionParent>();
const _: () = assert_box_and_cell_swappable::<ExpressionParent>();

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

const _: () = assert_eq_size_align::<IdentifierReference, TraversableIdentifierReference>();
const _: () = assert_box_and_cell_swappable::<IdentifierReference>();

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

const _: () = assert_eq_size_align::<StringLiteral, TraversableStringLiteral>();
const _: () = assert_box_and_cell_swappable::<StringLiteral>();

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

const _: () = assert_eq_size_align::<BinaryExpression, TraversableBinaryExpression>();
const _: () = assert_box_and_cell_swappable::<BinaryExpression>();

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

const _: () = assert_eq_size_align::<UnaryExpression, TraversableUnaryExpression>();
const _: () = assert_box_and_cell_swappable::<UnaryExpression>();

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
    pub type Statement<'a, 't> = super::TraversableStatement<'a, 't>;
    pub type ExpressionStatement<'a, 't> = super::TraversableExpressionStatement<'a, 't>;
    pub type Expression<'a, 't> = super::TraversableExpression<'a, 't>;
    pub type ExpressionParent<'a, 't> = super::TraversableExpressionParent<'a, 't>;
    pub type IdentifierReference<'a, 't> = super::TraversableIdentifierReference<'a, 't>;
    pub type StringLiteral<'a, 't> = super::TraversableStringLiteral<'a, 't>;
    pub type BinaryExpression<'a, 't> = super::TraversableBinaryExpression<'a, 't>;
    pub type UnaryExpression<'a, 't> = super::TraversableUnaryExpression<'a, 't>;
}

const fn assert_eq_size_align<T1, T2>() {
    use std::mem::{align_of, size_of};
    assert!(size_of::<T1>() == size_of::<T2>());
    assert!(align_of::<T1>() == align_of::<T2>());
}

const fn assert_box_and_cell_swappable<T>() {
    assert_eq_size_align::<Box<T>, &crate::cell::GCell<T>>()
}
