// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Expr {
    pub kind: ExprKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExprKind {
    Var(Ident),
    Lit(Lit),
    App { callee: Box<Expr>, arg: Box<Expr> },
    Abs { param: Ident, body: Box<Expr> },
    Let { name: Ident, value: Box<Expr>, body: Box<Expr> },
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.kind {
            ExprKind::Var(ident) =>
                write!(f, "{}", ident),
            ExprKind::Lit(lit) =>
                write!(f, "{}", lit),
            ExprKind::App { callee, arg } =>
                write!(f, "(app {} {})", callee, arg),
            ExprKind::Abs { param, body } =>
                write!(f, "(abs {} -> {})", param, body),
            ExprKind::Let { name, value, body } =>
                write!(f, "(let {} = {} in {})", name, value, body),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Lit {
    Int(i32),
    Bool(bool),
}

impl fmt::Display for Lit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Lit::Int(i) => write!(f, "{}", i),
            Lit::Bool(b) => write!(f, "{}", b),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ident {
    pub name: String,
}

impl fmt::Display for Ident {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
