// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.

use std::collections::{BTreeMap, BTreeSet};

use crate::expr::expr::Ident;

use super::{TyVar, ty_scheme::TyScheme, types::Types, subst::Subst};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TyCtxt {
    types: BTreeMap<Ident, TyScheme>,
}

impl TyCtxt {
    pub fn new() -> TyCtxt {
        TyCtxt {
            types: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, var: Ident, ty_scheme: TyScheme) {
        self.types.insert(var, ty_scheme);
    }

    pub fn remove(&mut self, var: &Ident) -> Option<TyScheme> {
        self.types.remove(var)
    }

    pub fn get(&self, var: &Ident) -> Option<&TyScheme> {
        self.types.get(var)
    }
}

impl Types for TyCtxt {
    fn ftv(&self) -> BTreeSet<TyVar> {
        self.types.values().map(|x| x.clone()).collect::<Vec<_>>().ftv()
    }

    fn apply(&self, subst: &Subst) -> Self {
        TyCtxt {
            types: self.types.iter()
                .map(|(k, v)| (k.clone(), v.apply(subst)))
                .collect(),
        }
    }
}
