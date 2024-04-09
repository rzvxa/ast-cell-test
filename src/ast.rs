#![allow(dead_code, clippy::enum_variant_names)]

use std::{
    fmt::{Debug, Formatter},
    marker::PhantomData,
};

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

#[derive(Clone)]
pub enum Statement<'a> {
    ExpressionStatement(NodeId<'a>),
}

impl<'a> AsAstRef<'a> for Statement<'a> {
    fn as_ast_ref(&'a mut self) -> AstRef<'a> {
        AstRef {
            ty: AstType::Stmt,
            val: AstUntyped { stmt: self },
        }
    }
}

#[derive(Clone)]
pub enum Expression<'a> {
    StringLiteral(NodeId<'a>),
    Identifier(NodeId<'a>),
    BinaryExpression(NodeId<'a>),
    UnaryExpression(NodeId<'a>),
}

impl<'a> AsAstRef<'a> for Expression<'a> {
    fn as_ast_ref(&'a mut self) -> AstRef<'a> {
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

impl<'a> AsAstRef<'a> for IdentifierReference<'a> {
    fn as_ast_ref(&'a mut self) -> AstRef<'a> {
        AstRef {
            ty: AstType::Ident,
            val: AstUntyped { ident: self },
        }
    }
}

#[derive(Clone)]
pub struct StringLiteral<'a> {
    pub value: &'a str,
    pub parent: NodeId<'a>,
}

impl<'a> AsAstRef<'a> for StringLiteral<'a> {
    fn as_ast_ref(&'a mut self) -> AstRef<'a> {
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

impl<'a> AsAstRef<'a> for BinaryExpression<'a> {
    fn as_ast_ref(&'a mut self) -> AstRef<'a> {
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

#[derive(Clone)]
pub struct UnaryExpression<'a> {
    pub operator: UnaryOperator,
    pub argument: NodeId<'a>,
    pub parent: NodeId<'a>,
}

impl<'a> AsAstRef<'a> for UnaryExpression<'a> {
    fn as_ast_ref(&'a mut self) -> AstRef<'a> {
        AstRef {
            ty: AstType::Unary,
            val: AstUntyped { unary: self },
        }
    }
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

#[derive(Debug)]
pub struct AstRef<'a> {
    ty: AstType,
    val: AstUntyped<'a>,
}

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

    // --------- unchecked ---------

    pub fn as_stmt_unchecked(&self) -> &Statement<'a> {
        unsafe { self.val.stmt }
    }

    pub fn as_expr_unchecked(&self) -> &Expression<'a> {
        unsafe { self.val.expr }
    }

    pub fn as_ident_unchecked(&self) -> &IdentifierReference<'a> {
        unsafe { self.val.ident }
    }

    pub fn as_str_unchecked(&self) -> &StringLiteral<'a> {
        unsafe { self.val.str }
    }

    pub fn as_binary_unchecked(&self) -> &BinaryExpression<'a> {
        unsafe { self.val.binary }
    }

    pub fn as_unary_unchecked(&self) -> &UnaryExpression<'a> {
        unsafe { self.val.unary }
    }

    // --------- unchecked mut ---------

    pub fn as_stmt_mut_unchecked(&mut self) -> &mut Statement<'a> {
        unsafe { self.val.stmt }
    }

    pub fn as_expr_mut_unchecked(&mut self) -> &mut Expression<'a> {
        unsafe { self.val.expr }
    }

    pub fn as_ident_mut_unchecked(&mut self) -> &mut IdentifierReference<'a> {
        unsafe { self.val.ident }
    }
    pub fn as_str_mut_unchecked(&mut self) -> &mut StringLiteral<'a> {
        unsafe { self.val.str }
    }

    pub fn as_binary_mut_unchecked(&mut self) -> &mut BinaryExpression<'a> {
        unsafe { self.val.binary }
    }

    pub fn as_unary_mut_unchecked(&mut self) -> &mut UnaryExpression<'a> {
        unsafe { self.val.unary }
    }
}

union AstUntyped<'a> {
    stmt: &'a mut Statement<'a>,
    expr: &'a mut Expression<'a>,
    ident: &'a mut IdentifierReference<'a>,
    str: &'a mut StringLiteral<'a>,
    binary: &'a mut BinaryExpression<'a>,
    unary: &'a mut UnaryExpression<'a>,
}

impl<'a> Debug for AstUntyped<'a> {
    fn fmt(&self, _: &mut Formatter<'_>) -> core::fmt::Result {
        Ok(())
    }
}

impl<'a> AstUntyped<'a> {
    pub fn as_stmt_unchecked(&self) -> &Statement<'a> {
        unsafe { self.stmt }
    }

    pub fn as_expr_unchecked(&self) -> &Expression<'a> {
        unsafe { self.expr }
    }

    pub fn as_ident_unchecked(&self) -> &IdentifierReference<'a> {
        unsafe { self.ident }
    }

    pub fn as_str_unchecked(&self) -> &StringLiteral<'a> {
        unsafe { self.str }
    }

    pub fn as_binary_unchecked(&self) -> &BinaryExpression<'a> {
        unsafe { self.binary }
    }

    pub fn as_unary_unchecked(&self) -> &UnaryExpression<'a> {
        unsafe { self.unary }
    }

    // --------- mutables ---------
    //
    pub fn as_stmt_mut_unchecked(&mut self) -> &mut Statement<'a> {
        unsafe { self.stmt }
    }

    pub fn as_expr_mut_unchecked(&mut self) -> &mut Expression<'a> {
        unsafe { self.expr }
    }

    pub fn as_ident_mut_unchecked(&mut self) -> &mut IdentifierReference<'a> {
        unsafe { self.ident }
    }
    pub fn as_str_mut_unchecked(&mut self) -> &mut StringLiteral<'a> {
        unsafe { self.str }
    }

    pub fn as_binary_mut_unchecked(&mut self) -> &mut BinaryExpression<'a> {
        unsafe { self.binary }
    }

    pub fn as_unary_mut_unchecked(&mut self) -> &mut UnaryExpression<'a> {
        unsafe { self.unary }
    }
}

pub trait AsAstRef<'a> {
    fn as_ast_ref(&'a mut self) -> AstRef<'a>;
}
