// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Expr {
    pub kind: ExprKind,
}

impl Expr {
    pub fn new(kind: ExprKind) -> Expr {
        Expr { kind }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExprKind {
    Var(Ident),
    Lit(Lit),
    App { callee: Box<Expr>, arg: Box<Expr> },
    Abs { param: Ident, body: Box<Expr> },
    Let { name: Ident, value: Box<Expr>, body: Box<Expr> },
}

impl ExprKind {
    pub fn var(name: Ident) -> ExprKind {
        ExprKind::Var(name)
    }

    pub fn lit(lit: Lit) -> ExprKind {
        ExprKind::Lit(lit)
    }

    pub fn app(callee: Expr, arg: Expr) -> ExprKind {
        ExprKind::App {
            callee: Box::new(callee),
            arg: Box::new(arg),
        }
    }

    pub fn abs(param: Ident, body: Expr) -> ExprKind {
        ExprKind::Abs {
            param,
            body: Box::new(body),
        }
    }

    pub fn let_in(name: Ident, value: Expr, body: Expr) -> ExprKind {
        ExprKind::Let {
            name,
            value: Box::new(value),
            body: Box::new(body),
        }
    }
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

impl Ident {
    pub fn new(name: String) -> Ident {
        Ident { name }
    }
}

impl fmt::Display for Ident {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
