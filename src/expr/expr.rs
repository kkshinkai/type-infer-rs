// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.

use std::fmt;

use crate::expr::expr_id::ExprId;

use super::lit::Lit;

#[derive(Debug, Clone, PartialEq)]
pub struct Expr {
    pub kind: ExprKind,
    pub id: ExprId,
}

impl Expr {
    pub fn new(kind: ExprKind) -> Expr {
        Expr {
            kind,
            id: ExprId::dummy(),
        }
    }

    pub fn mk_var(name: Ident) -> Expr {
        Expr::new(ExprKind::Var(name))
    }

    pub fn mk_lit(lit: Lit) -> Expr {
        Expr::new(ExprKind::Lit(lit))
    }

    pub fn mk_app(callee: Expr, arg: Expr) -> Expr {
        Expr::new(ExprKind::App {
            callee: Box::new(callee),
            arg: Box::new(arg),
        })
    }

    pub fn mk_abs(param: Ident, body: Expr) -> Expr {
        Expr::new(ExprKind::Abs {
            param,
            body: Box::new(body),
        })
    }

    pub fn mk_let(name: Ident, value: Expr, body: Expr) -> Expr {
        Expr::new(ExprKind::Let {
            name,
            value: Box::new(value),
            body: Box::new(body),
        })
    }

    pub fn assign_ids(&mut self) {
        let mut id_gen = 0;
        self.assign_ids_rec(&mut id_gen);
    }

    fn assign_ids_rec(&mut self, used_id_space: &mut u32) {
        *used_id_space += 1;
        self.id = ExprId::from_u32(*used_id_space);
        match &mut self.kind {
            ExprKind::Var(_) => {}
            ExprKind::Lit(_) => {}
            ExprKind::App { callee, arg } => {
                callee.assign_ids_rec(used_id_space);
                arg.assign_ids_rec(used_id_space);
            }
            ExprKind::Abs { param: _, body } => {
                body.assign_ids_rec(used_id_space);
            }
            ExprKind::Let { name: _, value, body } => {
                value.assign_ids_rec(used_id_space);
                body.assign_ids_rec(used_id_space);
            }
        }
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
