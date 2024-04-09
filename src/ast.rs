#![allow(dead_code, clippy::enum_variant_names)]
#[cfg(feature = "unsafe")]
use std::fmt::Formatter;
#[cfg(all(feature = "unsafe", not(feature = "astref")))]
use std::mem::ManuallyDrop;

use std::{fmt::Debug, marker::PhantomData};

#[derive(Default, Debug, Clone, Copy)]
pub struct NodeId<'a>(usize, PhantomData<&'a ()>);

impl<'a> NodeId<'a> {
    #[inline(always)]
    pub fn new(index: usize) -> Self {
        Self(index, PhantomData {})
    }

    #[inline(always)]
    pub fn as_index(self) -> usize {
        self.0
    }
}

#[derive(Debug, Clone)]
pub enum Statement<'a> {
    ExpressionStatement(NodeId<'a>),
}

#[cfg(all(not(feature = "unsafe"), not(feature = "astref")))]
impl<'a> AsAstRef<'a> for Statement<'a> {
    fn as_ast_ref(self) -> AstRef<'a> {
        AstRef {
            inner: AstKind::Stmt(self),
        }
    }
}

#[cfg(all(not(feature = "unsafe"), feature = "astref"))]
impl<'a> AsAstRef<'a> for &'a mut Statement<'a> {
    fn as_ast_ref(self) -> AstRef<'a> {
        AstRef {
            inner: AstKind::Stmt(self),
        }
    }
}

#[cfg(all(feature = "unsafe", not(feature = "astref")))]
impl<'a> AsAstRef<'a> for ManuallyDrop<Statement<'a>> {
    fn as_ast_ref(self) -> AstRef<'a> {
        AstRef {
            ty: AstType::Stmt,
            val: { AstUntyped { stmt: self } },
        }
    }
}

#[cfg(all(feature = "unsafe", feature = "astref"))]
impl<'a> AsAstRef<'a> for &'a mut Statement<'a> {
    fn as_ast_ref(self) -> AstRef<'a> {
        AstRef {
            ty: AstType::Stmt,
            val: { AstUntyped { stmt: self } },
        }
    }
}

#[derive(Debug, Clone)]
pub enum Expression<'a> {
    StringLiteral(NodeId<'a>),
    Identifier(NodeId<'a>),
    BinaryExpression(NodeId<'a>),
    UnaryExpression(NodeId<'a>),
}

#[cfg(all(not(feature = "unsafe"), not(feature = "astref")))]
impl<'a> AsAstRef<'a> for Expression<'a> {
    fn as_ast_ref(self) -> AstRef<'a> {
        AstRef {
            inner: AstKind::Expr(self),
        }
    }
}

#[cfg(all(not(feature = "unsafe"), feature = "astref"))]
impl<'a> AsAstRef<'a> for &'a mut Expression<'a> {
    fn as_ast_ref(self) -> AstRef<'a> {
        AstRef {
            inner: AstKind::Expr(self),
        }
    }
}

#[cfg(all(feature = "unsafe", not(feature = "astref")))]
impl<'a> AsAstRef<'a> for ManuallyDrop<Expression<'a>> {
    fn as_ast_ref(self) -> AstRef<'a> {
        AstRef {
            ty: AstType::Expr,
            val: AstUntyped { expr: self },
        }
    }
}

#[cfg(all(feature = "unsafe", feature = "astref"))]
impl<'a> AsAstRef<'a> for &'a mut Expression<'a> {
    fn as_ast_ref(self) -> AstRef<'a> {
        AstRef {
            ty: AstType::Expr,
            val: AstUntyped { expr: self },
        }
    }
}

#[derive(Debug, Clone)]
pub struct IdentifierReference<'a> {
    pub name: &'a str,
    pub parent: NodeId<'a>,
}

#[cfg(all(not(feature = "unsafe"), not(feature = "astref")))]
impl<'a> AsAstRef<'a> for IdentifierReference<'a> {
    fn as_ast_ref(self) -> AstRef<'a> {
        AstRef {
            inner: AstKind::Ident(self),
        }
    }
}

#[cfg(all(not(feature = "unsafe"), feature = "astref"))]
impl<'a> AsAstRef<'a> for &'a mut IdentifierReference<'a> {
    fn as_ast_ref(self) -> AstRef<'a> {
        AstRef {
            inner: AstKind::Ident(self),
        }
    }
}

#[cfg(all(feature = "unsafe", not(feature = "astref")))]
impl<'a> AsAstRef<'a> for ManuallyDrop<IdentifierReference<'a>> {
    fn as_ast_ref(self) -> AstRef<'a> {
        AstRef {
            ty: AstType::Ident,
            val: AstUntyped { ident: self },
        }
    }
}

#[cfg(all(feature = "unsafe", feature = "astref"))]
impl<'a> AsAstRef<'a> for &'a mut IdentifierReference<'a> {
    fn as_ast_ref(self) -> AstRef<'a> {
        AstRef {
            ty: AstType::Ident,
            val: AstUntyped { ident: self },
        }
    }
}

#[derive(Debug, Clone)]
pub struct StringLiteral<'a> {
    pub value: &'a str,
    pub parent: NodeId<'a>,
}

#[cfg(all(not(feature = "unsafe"), not(feature = "astref")))]
impl<'a> AsAstRef<'a> for StringLiteral<'a> {
    fn as_ast_ref(self) -> AstRef<'a> {
        AstRef {
            inner: AstKind::Str(self),
        }
    }
}

#[cfg(all(not(feature = "unsafe"), feature = "astref"))]
impl<'a> AsAstRef<'a> for &'a mut StringLiteral<'a> {
    fn as_ast_ref(self) -> AstRef<'a> {
        AstRef {
            inner: AstKind::Str(self),
        }
    }
}

#[cfg(all(feature = "unsafe", not(feature = "astref")))]
impl<'a> AsAstRef<'a> for ManuallyDrop<StringLiteral<'a>> {
    fn as_ast_ref(self) -> AstRef<'a> {
        AstRef {
            ty: AstType::Str,
            val: AstUntyped { str: self },
        }
    }
}

#[cfg(all(feature = "unsafe", feature = "astref"))]
impl<'a> AsAstRef<'a> for &'a mut StringLiteral<'a> {
    fn as_ast_ref(self) -> AstRef<'a> {
        AstRef {
            ty: AstType::Str,
            val: AstUntyped { str: self },
        }
    }
}

#[derive(Debug, Clone)]
pub struct BinaryExpression<'a> {
    pub left: NodeId<'a>,
    pub operator: BinaryOperator,
    pub right: NodeId<'a>,
    pub parent: NodeId<'a>,
}

#[cfg(all(not(feature = "unsafe"), not(feature = "astref")))]
impl<'a> AsAstRef<'a> for BinaryExpression<'a> {
    fn as_ast_ref(self) -> AstRef<'a> {
        AstRef {
            inner: AstKind::Binary(self),
        }
    }
}

#[cfg(all(not(feature = "unsafe"), feature = "astref"))]
impl<'a> AsAstRef<'a> for &'a mut BinaryExpression<'a> {
    fn as_ast_ref(self) -> AstRef<'a> {
        AstRef {
            inner: AstKind::Binary(self),
        }
    }
}

#[cfg(all(feature = "unsafe", not(feature = "astref")))]
impl<'a> AsAstRef<'a> for ManuallyDrop<BinaryExpression<'a>> {
    fn as_ast_ref(self) -> AstRef<'a> {
        AstRef {
            ty: AstType::Binary,
            val: AstUntyped { binary: self },
        }
    }
}

#[cfg(all(feature = "unsafe", feature = "astref"))]
impl<'a> AsAstRef<'a> for &'a mut BinaryExpression<'a> {
    fn as_ast_ref(self) -> AstRef<'a> {
        AstRef {
            ty: AstType::Binary,
            val: AstUntyped { binary: self },
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum BinaryOperator {
    Equality,
    StrictEquality,
}

#[derive(Debug, Clone)]
pub struct UnaryExpression<'a> {
    pub operator: UnaryOperator,
    pub argument: NodeId<'a>,
    pub parent: NodeId<'a>,
}

#[cfg(all(not(feature = "unsafe"), not(feature = "astref")))]
impl<'a> AsAstRef<'a> for UnaryExpression<'a> {
    fn as_ast_ref(self) -> AstRef<'a> {
        AstRef {
            inner: AstKind::Unary(self),
        }
    }
}

#[cfg(all(not(feature = "unsafe"), feature = "astref"))]
impl<'a> AsAstRef<'a> for &'a mut UnaryExpression<'a> {
    fn as_ast_ref(self) -> AstRef<'a> {
        AstRef {
            inner: AstKind::Unary(self),
        }
    }
}

#[cfg(all(feature = "unsafe", not(feature = "astref")))]
impl<'a> AsAstRef<'a> for ManuallyDrop<UnaryExpression<'a>> {
    fn as_ast_ref(self) -> AstRef<'a> {
        AstRef {
            ty: AstType::Unary,
            val: AstUntyped { unary: self },
        }
    }
}

#[cfg(all(feature = "unsafe", feature = "astref"))]
impl<'a> AsAstRef<'a> for &'a mut UnaryExpression<'a> {
    fn as_ast_ref(self) -> AstRef<'a> {
        AstRef {
            ty: AstType::Unary,
            val: AstUntyped { unary: self },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnaryOperator {
    UnaryNegation,
    UnaryPlus,
    LogicalNot,
    BitwiseNot,
    Typeof,
    Void,
    Delete,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AstType {
    Stmt,
    Expr,
    Ident,
    Str,
    Binary,
    Unary,
}

#[repr(u8)]
#[derive(Debug)]
#[cfg(all(not(feature = "unsafe"), not(feature = "astref")))]
pub enum AstKind<'a> {
    Stmt(Statement<'a>),
    Expr(Expression<'a>),
    Ident(IdentifierReference<'a>),
    Str(StringLiteral<'a>),
    Binary(BinaryExpression<'a>),
    Unary(UnaryExpression<'a>),
}

#[repr(u8)]
#[derive(Debug)]
#[cfg(all(not(feature = "unsafe"), feature = "astref"))]
pub enum AstKind<'a> {
    Stmt(&'a mut Statement<'a>),
    Expr(&'a mut Expression<'a>),
    Ident(&'a mut IdentifierReference<'a>),
    Str(&'a mut StringLiteral<'a>),
    Binary(&'a mut BinaryExpression<'a>),
    Unary(&'a mut UnaryExpression<'a>),
}

#[cfg(not(feature = "unsafe"))]
impl<'a> Into<AstType> for &AstKind<'a> {
    fn into(self) -> AstType {
        match self {
            AstKind::Stmt(_) => AstType::Stmt,
            AstKind::Expr(_) => AstType::Expr,
            AstKind::Ident(_) => AstType::Ident,
            AstKind::Str(_) => AstType::Str,
            AstKind::Binary(_) => AstType::Binary,
            AstKind::Unary(_) => AstType::Unary,
        }
    }
}

#[derive(Debug)]
#[cfg(not(feature = "unsafe"))]
pub struct AstRef<'a> {
    inner: AstKind<'a>,
}

#[cfg(not(feature = "unsafe"))]
macro_rules! as_or_panic {
    ($in:expr, $out:path) => {
        match $in {
            $out(out) => out,
            _ => panic!(),
        }
    };
}

#[cfg(not(feature = "unsafe"))]
impl<'a> AstRef<'a> {
    pub fn ast_type(&self) -> AstType {
        (&self.inner).into()
    }

    #[inline]
    pub fn is(&self, ty: AstType) -> bool {
        self.ast_type() == ty
    }

    pub fn as_stmt(&self) -> Option<&Statement<'a>> {
        if self.is(AstType::Stmt) {
            Some(self.as_stmt_unchecked())
        } else {
            None
        }
    }

    pub fn as_expr(&self) -> Option<&Expression<'a>> {
        if self.is(AstType::Expr) {
            Some(self.as_expr_unchecked())
        } else {
            None
        }
    }

    pub fn as_ident(&self) -> Option<&IdentifierReference<'a>> {
        if self.is(AstType::Ident) {
            Some(self.as_ident_unchecked())
        } else {
            None
        }
    }

    pub fn as_str(&self) -> Option<&StringLiteral<'a>> {
        if self.is(AstType::Str) {
            Some(self.as_str_unchecked())
        } else {
            None
        }
    }

    pub fn as_binary(&self) -> Option<&BinaryExpression<'a>> {
        if self.is(AstType::Binary) {
            Some(self.as_binary_unchecked())
        } else {
            None
        }
    }

    pub fn as_unary(&self) -> Option<&UnaryExpression<'a>> {
        if self.is(AstType::Unary) {
            Some(self.as_unary_unchecked())
        } else {
            None
        }
    }

    pub fn as_stmt_mut(&mut self) -> Option<&mut Statement<'a>> {
        if self.is(AstType::Stmt) {
            Some(self.as_stmt_mut_unchecked())
        } else {
            None
        }
    }

    pub fn as_expr_mut(&mut self) -> Option<&mut Expression<'a>> {
        if self.is(AstType::Expr) {
            Some(self.as_expr_mut_unchecked())
        } else {
            None
        }
    }

    pub fn as_ident_mut(&mut self) -> Option<&mut IdentifierReference<'a>> {
        if self.is(AstType::Ident) {
            Some(self.as_ident_mut_unchecked())
        } else {
            None
        }
    }

    pub fn as_str_mut(&mut self) -> Option<&mut StringLiteral<'a>> {
        if self.is(AstType::Str) {
            Some(self.as_str_mut_unchecked())
        } else {
            None
        }
    }

    pub fn as_binary_mut(&mut self) -> Option<&mut BinaryExpression<'a>> {
        if self.is(AstType::Binary) {
            Some(self.as_binary_mut_unchecked())
        } else {
            None
        }
    }

    pub fn as_unary_mut(&mut self) -> Option<&mut UnaryExpression<'a>> {
        if self.is(AstType::Unary) {
            Some(self.as_unary_mut_unchecked())
        } else {
            None
        }
    }

    // These should be unsafe in the final implementation
    // --------- unchecked ---------

    pub fn as_stmt_unchecked(&self) -> &Statement<'a> {
        as_or_panic!(&self.inner, AstKind::Stmt)
    }

    pub fn as_expr_unchecked(&self) -> &Expression<'a> {
        as_or_panic!(&self.inner, AstKind::Expr)
    }

    pub fn as_ident_unchecked(&self) -> &IdentifierReference<'a> {
        as_or_panic!(&self.inner, AstKind::Ident)
    }

    pub fn as_str_unchecked(&self) -> &StringLiteral<'a> {
        as_or_panic!(&self.inner, AstKind::Str)
    }

    pub fn as_binary_unchecked(&self) -> &BinaryExpression<'a> {
        as_or_panic!(&self.inner, AstKind::Binary)
    }

    pub fn as_unary_unchecked(&self) -> &UnaryExpression<'a> {
        as_or_panic!(&self.inner, AstKind::Unary)
    }

    // --------- unchecked mut ---------

    pub fn as_stmt_mut_unchecked(&mut self) -> &mut Statement<'a> {
        as_or_panic!(&mut self.inner, AstKind::Stmt)
    }

    pub fn as_expr_mut_unchecked(&mut self) -> &mut Expression<'a> {
        as_or_panic!(&mut self.inner, AstKind::Expr)
    }

    pub fn as_ident_mut_unchecked(&mut self) -> &mut IdentifierReference<'a> {
        as_or_panic!(&mut self.inner, AstKind::Ident)
    }
    pub fn as_str_mut_unchecked(&mut self) -> &mut StringLiteral<'a> {
        as_or_panic!(&mut self.inner, AstKind::Str)
    }

    pub fn as_binary_mut_unchecked(&mut self) -> &mut BinaryExpression<'a> {
        as_or_panic!(&mut self.inner, AstKind::Binary)
    }

    pub fn as_unary_mut_unchecked(&mut self) -> &mut UnaryExpression<'a> {
        as_or_panic!(&mut self.inner, AstKind::Unary)
    }
}

#[derive(Debug)]
#[cfg(feature = "unsafe")]
pub struct AstRef<'a> {
    ty: AstType,
    val: AstUntyped<'a>,
}

// SAFETY: Statement kind should be currect,
// And as for the union itself, it shouldn't be used after this drop call
#[cfg(all(feature = "unsafe", not(feature = "astref")))]
macro_rules! unsafe_ast_ref_drop {
    ($self:ident, $kind:ident) => {{
        let drop = &mut $self.val.$kind;
        ManuallyDrop::drop(drop);
    }};
}
#[cfg(all(feature = "unsafe", not(feature = "astref")))]
impl<'a> Drop for AstRef<'a> {
    fn drop(&mut self) {
        // SAFETY: we alreay checked the type for the inner value.
        // And as for the drop we drop the value and never touch it again.
        match self.ast_type() {
            AstType::Stmt => unsafe { unsafe_ast_ref_drop!(self, stmt) },
            AstType::Expr => unsafe { unsafe_ast_ref_drop!(self, expr) },
            AstType::Ident => unsafe { unsafe_ast_ref_drop!(self, ident) },
            AstType::Str => unsafe { unsafe_ast_ref_drop!(self, str) },
            AstType::Binary => unsafe { unsafe_ast_ref_drop!(self, binary) },
            AstType::Unary => unsafe { unsafe_ast_ref_drop!(self, unary) },
        }
    }
}

#[cfg(feature = "unsafe")]
impl<'a> AstRef<'a> {
    pub fn ast_type(&self) -> AstType {
        self.ty
    }

    pub fn is(&self, ty: AstType) -> bool {
        self.ty == ty
    }

    pub fn as_stmt(&self) -> Option<&Statement<'a>> {
        if self.is(AstType::Stmt) {
            Some(self.as_stmt_unchecked())
        } else {
            None
        }
    }

    pub fn as_expr(&self) -> Option<&Expression<'a>> {
        if self.is(AstType::Expr) {
            Some(self.as_expr_unchecked())
        } else {
            None
        }
    }

    pub fn as_ident(&self) -> Option<&IdentifierReference<'a>> {
        if self.is(AstType::Ident) {
            Some(self.as_ident_unchecked())
        } else {
            None
        }
    }

    pub fn as_str(&self) -> Option<&StringLiteral<'a>> {
        if self.is(AstType::Str) {
            Some(self.as_str_unchecked())
        } else {
            None
        }
    }

    pub fn as_binary(&self) -> Option<&BinaryExpression<'a>> {
        if self.is(AstType::Binary) {
            Some(self.as_binary_unchecked())
        } else {
            None
        }
    }

    pub fn as_unary(&self) -> Option<&UnaryExpression<'a>> {
        if self.is(AstType::Unary) {
            Some(self.as_unary_unchecked())
        } else {
            None
        }
    }

    pub fn as_stmt_mut(&mut self) -> Option<&mut Statement<'a>> {
        if self.is(AstType::Stmt) {
            Some(self.as_stmt_mut_unchecked())
        } else {
            None
        }
    }

    pub fn as_expr_mut(&mut self) -> Option<&mut Expression<'a>> {
        if self.is(AstType::Expr) {
            Some(self.as_expr_mut_unchecked())
        } else {
            None
        }
    }

    pub fn as_ident_mut(&mut self) -> Option<&mut IdentifierReference<'a>> {
        if self.is(AstType::Ident) {
            Some(self.as_ident_mut_unchecked())
        } else {
            None
        }
    }

    pub fn as_str_mut(&mut self) -> Option<&mut StringLiteral<'a>> {
        if self.is(AstType::Str) {
            Some(self.as_str_mut_unchecked())
        } else {
            None
        }
    }

    pub fn as_binary_mut(&mut self) -> Option<&mut BinaryExpression<'a>> {
        if self.is(AstType::Binary) {
            Some(self.as_binary_mut_unchecked())
        } else {
            None
        }
    }

    pub fn as_unary_mut(&mut self) -> Option<&mut UnaryExpression<'a>> {
        if self.is(AstType::Unary) {
            Some(self.as_unary_mut_unchecked())
        } else {
            None
        }
    }

    // These should be unsafe in the final implementation
    // --------- unchecked ---------

    pub fn as_stmt_unchecked(&self) -> &Statement<'a> {
        unsafe { &self.val.stmt }
    }

    pub fn as_expr_unchecked(&self) -> &Expression<'a> {
        unsafe { &self.val.expr }
    }

    pub fn as_ident_unchecked(&self) -> &IdentifierReference<'a> {
        unsafe { &self.val.ident }
    }

    pub fn as_str_unchecked(&self) -> &StringLiteral<'a> {
        unsafe { &self.val.str }
    }

    pub fn as_binary_unchecked(&self) -> &BinaryExpression<'a> {
        unsafe { &self.val.binary }
    }

    pub fn as_unary_unchecked(&self) -> &UnaryExpression<'a> {
        unsafe { &self.val.unary }
    }

    // --------- unchecked mut ---------

    pub fn as_stmt_mut_unchecked(&mut self) -> &mut Statement<'a> {
        unsafe { &mut self.val.stmt }
    }

    pub fn as_expr_mut_unchecked(&mut self) -> &mut Expression<'a> {
        unsafe { &mut self.val.expr }
    }

    pub fn as_ident_mut_unchecked(&mut self) -> &mut IdentifierReference<'a> {
        unsafe { &mut self.val.ident }
    }
    pub fn as_str_mut_unchecked(&mut self) -> &mut StringLiteral<'a> {
        unsafe { &mut self.val.str }
    }

    pub fn as_binary_mut_unchecked(&mut self) -> &mut BinaryExpression<'a> {
        unsafe { &mut self.val.binary }
    }

    pub fn as_unary_mut_unchecked(&mut self) -> &mut UnaryExpression<'a> {
        unsafe { &mut self.val.unary }
    }
}

#[cfg(all(feature = "unsafe", not(feature = "astref")))]
union AstUntyped<'a> {
    stmt: ManuallyDrop<Statement<'a>>,
    expr: ManuallyDrop<Expression<'a>>,
    ident: ManuallyDrop<IdentifierReference<'a>>,
    str: ManuallyDrop<StringLiteral<'a>>,
    binary: ManuallyDrop<BinaryExpression<'a>>,
    unary: ManuallyDrop<UnaryExpression<'a>>,
}

#[cfg(all(feature = "unsafe", feature = "astref"))]
union AstUntyped<'a> {
    stmt: &'a mut Statement<'a>,
    expr: &'a mut Expression<'a>,
    ident: &'a mut IdentifierReference<'a>,
    str: &'a mut StringLiteral<'a>,
    binary: &'a mut BinaryExpression<'a>,
    unary: &'a mut UnaryExpression<'a>,
}

#[cfg(feature = "unsafe")]
impl<'a> Debug for AstUntyped<'a> {
    fn fmt(&self, _: &mut Formatter<'_>) -> core::fmt::Result {
        Ok(())
    }
}

pub trait AsAstRef<'a> {
    fn as_ast_ref(self) -> AstRef<'a>;
}
