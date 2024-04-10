#![allow(dead_code, clippy::enum_variant_names)]

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

impl<'a> AsAstRef<'a> for &'a mut Statement<'a> {
    fn as_ast_ref(self) -> AstRef<'a> {
        AstRef {
            inner: AstKind::Stmt(self),
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

impl<'a> AsAstRef<'a> for &'a mut Expression<'a> {
    fn as_ast_ref(self) -> AstRef<'a> {
        AstRef {
            inner: AstKind::Expr(self),
        }
    }
}

#[derive(Debug, Clone)]
pub struct IdentifierReference<'a> {
    pub name: &'a str,
    pub parent: NodeId<'a>,
}

impl<'a> AsAstRef<'a> for &'a mut IdentifierReference<'a> {
    fn as_ast_ref(self) -> AstRef<'a> {
        AstRef {
            inner: AstKind::Ident(self),
        }
    }
}

#[derive(Debug, Clone)]
pub struct StringLiteral<'a> {
    pub value: &'a str,
    pub parent: NodeId<'a>,
}


impl<'a> AsAstRef<'a> for &'a mut StringLiteral<'a> {
    fn as_ast_ref(self) -> AstRef<'a> {
        AstRef {
            inner: AstKind::Str(self),
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


impl<'a> AsAstRef<'a> for &'a mut BinaryExpression<'a> {
    fn as_ast_ref(self) -> AstRef<'a> {
        AstRef {
            inner: AstKind::Binary(self),
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


impl<'a> AsAstRef<'a> for &'a mut UnaryExpression<'a> {
    fn as_ast_ref(self) -> AstRef<'a> {
        AstRef {
            inner: AstKind::Unary(self),
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

pub trait AsAstRef<'a> {
    fn as_ast_ref(self) -> AstRef<'a>;
}
