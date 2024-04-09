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

impl<'a> AsAstKind<'a> for Statement<'a> {
    fn as_ast_kind(&'a mut self) -> AstKind<'a> {
        AstKind::Statement(self)
    }
}

#[derive(Debug, Clone)]
pub enum Expression<'a> {
    StringLiteral(NodeId<'a>),
    Identifier(NodeId<'a>),
    BinaryExpression(NodeId<'a>),
    UnaryExpression(NodeId<'a>),
}

impl<'a> AsAstKind<'a> for Expression<'a> {
    fn as_ast_kind(&'a mut self) -> AstKind<'a> {
        AstKind::Expression(self)
    }
}

#[derive(Debug, Clone)]
pub struct IdentifierReference<'a> {
    pub name: &'a str,
    pub parent: NodeId<'a>,
}

impl<'a> AsAstKind<'a> for IdentifierReference<'a> {
    fn as_ast_kind(&'a mut self) -> AstKind<'a> {
        AstKind::IdentifierReference(self)
    }
}

#[derive(Debug, Clone)]
pub struct StringLiteral<'a> {
    pub value: &'a str,
    pub parent: NodeId<'a>,
}

impl<'a> AsAstKind<'a> for StringLiteral<'a> {
    fn as_ast_kind(&'a mut self) -> AstKind<'a> {
        AstKind::StringLiteral(self)
    }
}

#[derive(Debug, Clone)]
pub struct BinaryExpression<'a> {
    pub left: NodeId<'a>,
    pub operator: BinaryOperator,
    pub right: NodeId<'a>,
    pub parent: NodeId<'a>,
}

impl<'a> AsAstKind<'a> for BinaryExpression<'a> {
    fn as_ast_kind(&'a mut self) -> AstKind<'a> {
        AstKind::BinaryExpression(self)
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

impl<'a> AsAstKind<'a> for UnaryExpression<'a> {
    fn as_ast_kind(&'a mut self) -> AstKind<'a> {
        AstKind::UnaryExpression(self)
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

#[derive(Debug)]
pub enum AstKind<'a> {
    Statement(&'a mut Statement<'a>),
    Expression(&'a mut Expression<'a>),
    IdentifierReference(&'a mut IdentifierReference<'a>),
    StringLiteral(&'a mut StringLiteral<'a>),
    BinaryExpression(&'a mut BinaryExpression<'a>),
    UnaryExpression(&'a mut UnaryExpression<'a>),
}

macro_rules! as_or_panic {
    ($in:ident, $out:path) => {
        match $in {
            $out(out) => out,
            _ => unreachable!(),
        }
    };
}

impl<'a> AstKind<'a> {
    pub fn ast_type(&self) -> AstType {
        use AstKind::*;
        use AstType::*;
        match self {
            Statement(_) => Stmt,
            Expression(_) => Expr,
            IdentifierReference(_) => Ident,
            StringLiteral(_) => Str,
            BinaryExpression(_) => Binary,
            UnaryExpression(_) => Unary,
        }
    }

    pub fn is(&self, ty: AstType) -> bool {
        self.ast_type() == ty
    }

    pub fn as_stmt(&self) -> Option<&Statement<'a>> {
        if self.is(AstType::Stmt) {
            Some(self.as_stmt_or_panic())
        } else {
            None
        }
    }

    pub fn as_expr(&self) -> Option<&Expression<'a>> {
        if self.is(AstType::Expr) {
            Some(self.as_expr_or_panic())
        } else {
            None
        }
    }

    pub fn as_ident(&self) -> Option<&IdentifierReference<'a>> {
        if self.is(AstType::Ident) {
            Some(self.as_ident_or_panic())
        } else {
            None
        }
    }

    pub fn as_str(&self) -> Option<&StringLiteral<'a>> {
        if self.is(AstType::Str) {
            Some(self.as_str_or_panic())
        } else {
            None
        }
    }

    pub fn as_binary(&self) -> Option<&BinaryExpression<'a>> {
        if self.is(AstType::Binary) {
            Some(self.as_binary_or_panic())
        } else {
            None
        }
    }

    pub fn as_unary(&self) -> Option<&UnaryExpression<'a>> {
        if self.is(AstType::Unary) {
            Some(self.as_unary_or_panic())
        } else {
            None
        }
    }

    pub fn as_stmt_mut(&mut self) -> Option<&mut Statement<'a>> {
        if self.is(AstType::Stmt) {
            Some(self.as_stmt_mut_or_panic())
        } else {
            None
        }
    }

    pub fn as_expr_mut(&mut self) -> Option<&mut Expression<'a>> {
        if self.is(AstType::Expr) {
            Some(self.as_expr_mut_or_panic())
        } else {
            None
        }
    }

    pub fn as_ident_mut(&mut self) -> Option<&mut IdentifierReference<'a>> {
        if self.is(AstType::Ident) {
            Some(self.as_ident_mut_or_panic())
        } else {
            None
        }
    }

    pub fn as_str_mut(&mut self) -> Option<&mut StringLiteral<'a>> {
        if self.is(AstType::Str) {
            Some(self.as_str_mut_or_panic())
        } else {
            None
        }
    }

    pub fn as_binary_mut(&mut self) -> Option<&mut BinaryExpression<'a>> {
        if self.is(AstType::Binary) {
            Some(self.as_binary_mut_or_panic())
        } else {
            None
        }
    }

    pub fn as_unary_mut(&mut self) -> Option<&mut UnaryExpression<'a>> {
        if self.is(AstType::Unary) {
            Some(self.as_unary_mut_or_panic())
        } else {
            None
        }
    }

    // --------- unchecked ---------

    pub fn as_stmt_or_panic(&self) -> &Statement<'a> {
        as_or_panic!(self, AstKind::Statement)
    }

    pub fn as_expr_or_panic(&self) -> &Expression<'a> {
        as_or_panic!(self, AstKind::Expression)
    }

    pub fn as_ident_or_panic(&self) -> &IdentifierReference<'a> {
        as_or_panic!(self, AstKind::IdentifierReference)
    }

    pub fn as_str_or_panic(&self) -> &StringLiteral<'a> {
        as_or_panic!(self, AstKind::StringLiteral)
    }

    pub fn as_binary_or_panic(&self) -> &BinaryExpression<'a> {
        as_or_panic!(self, AstKind::BinaryExpression)
    }

    pub fn as_unary_or_panic(&self) -> &UnaryExpression<'a> {
        as_or_panic!(self, AstKind::UnaryExpression)
    }

    // --------- unchecked mut ---------

    pub fn as_stmt_mut_or_panic(&mut self) -> &mut Statement<'a> {
        as_or_panic!(self, AstKind::Statement)
    }

    pub fn as_expr_mut_or_panic(&mut self) -> &mut Expression<'a> {
        as_or_panic!(self, AstKind::Expression)
    }

    pub fn as_ident_mut_or_panic(&mut self) -> &mut IdentifierReference<'a> {
        as_or_panic!(self, AstKind::IdentifierReference)
    }

    pub fn as_str_mut_or_panic(&mut self) -> &mut StringLiteral<'a> {
        as_or_panic!(self, AstKind::StringLiteral)
    }

    pub fn as_binary_mut_or_panic(&mut self) -> &mut BinaryExpression<'a> {
        as_or_panic!(self, AstKind::BinaryExpression)
    }

    pub fn as_unary_mut_or_panic(&mut self) -> &mut UnaryExpression<'a> {
        as_or_panic!(self, AstKind::UnaryExpression)
    }
}

pub trait AsAstKind<'a> {
    fn as_ast_kind(&'a mut self) -> AstKind<'a>;
}
