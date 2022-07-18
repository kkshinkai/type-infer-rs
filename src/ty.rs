// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.

use std::fmt;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Ty {
    Var(TyVar),
    Int,
    Bool,
    Arrow(Box<Ty>, Box<Ty>),
}

impl Ty {
    pub fn mk_var(name: TyVar) -> Ty {
        Ty::Var(name)
    }

    pub fn mk_int() -> Ty {
        Ty::Int
    }

    pub fn mk_bool() -> Ty {
        Ty::Bool
    }

    pub fn mk_arrow(param: Ty, body: Ty) -> Ty {
        Ty::Arrow(Box::new(param), Box::new(body))
    }
}

impl fmt::Display for Ty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Ty::Var(var) => write!(f, "{}", var),
            Ty::Int => write!(f, "int"),
            Ty::Bool => write!(f, "bool"),
            Ty::Arrow(param_ty, ret_ty) => match **param_ty {
                Ty::Arrow(_, _) => write!(f, "({}) -> {}", param_ty, ret_ty),
                _ => write!(f, "{} -> {}", param_ty, ret_ty),
            },
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TyScheme {
    pub vars: Vec<TyVar>,
    pub ty: Ty,
}

impl TyScheme {
    pub fn mk_forall(vars: Vec<TyVar>, ty: Ty) -> TyScheme {
        TyScheme { vars, ty }
    }
}

impl fmt::Display for TyScheme {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "forall {} . {}",
            self.vars.iter()
                .map(|var| &var.name[..])
                .collect::<Vec<&str>>()
                .join(" "),
            self.ty
        )
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TyVar {
    pub name: String,
}

impl TyVar {
    pub fn new(name: String) -> TyVar {
        TyVar { name }
    }
}

impl fmt::Display for TyVar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
