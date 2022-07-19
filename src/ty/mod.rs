// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.

pub mod ty_ctxt;
pub mod ty_scheme;
pub mod subst;
pub mod types;

use std::{fmt, collections::BTreeSet};

use self::{subst::Subst, types::Types};

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
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

impl Types for Ty {
    fn ftv(&self) -> BTreeSet<TyVar> {
        match self {
            Ty::Var(var) => BTreeSet::from([var.clone()]),
            Ty::Int | Ty::Bool => BTreeSet::new(),
            Ty::Arrow(param_ty, ret_ty) =>
                param_ty.ftv().union(&ret_ty.ftv()).cloned().collect(),
        }
    }

    fn apply(&self, subst: &Subst) -> Ty {
        match self {
            Ty::Var(name) => {
                match subst.get(name) {
                    Some(found_ty) if &found_ty != self => {
                        found_ty.apply(subst)
                    }
                    _ => self.clone(),
                }
            }
            Ty::Arrow(param_ty, ret_ty) => {
                Ty::mk_arrow(param_ty.apply(subst), ret_ty.apply(subst))
            }
            _ => self.clone(),
        }
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

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum TyVar {
    Name(String),
    Unknown(u32),
}

impl TyVar {
    pub fn new(name: String) -> TyVar {
        TyVar::Name(name)
    }

    pub fn unknown(id: u32) -> TyVar {
        TyVar::Unknown(id)
    }
}

impl fmt::Display for TyVar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TyVar::Name(name) => write!(f, "{}", name),
            TyVar::Unknown(_id) => write!(f, "?"),
        }
    }
}
