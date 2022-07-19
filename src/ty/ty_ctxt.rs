// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.

use std::collections::{BTreeMap, BTreeSet};

use super::{TyVar, ty_scheme::TyScheme, types::Types, subst::Subst};

pub struct TyCtxt {
    types: BTreeMap<TyVar, TyScheme>,
}

impl TyCtxt {
    pub fn new() -> TyCtxt {
        TyCtxt {
            types: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, ty_var: TyVar, ty_scheme: TyScheme) {
        self.types.insert(ty_var, ty_scheme);
    }

    pub fn remove(&mut self, ty_var: &TyVar) -> Option<TyScheme> {
        self.types.remove(ty_var)
    }

    pub fn get(&self, ty_var: &TyVar) -> Option<&TyScheme> {
        self.types.get(ty_var)
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
