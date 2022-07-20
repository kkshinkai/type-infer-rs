// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.

use std::collections::{HashMap, BTreeSet};

use crate::expr::expr::Ident;

use super::{ty_scheme::TyScheme, types::Types, TyVar, subst::Subst};

#[derive(Debug)]
pub struct TyCtxt {
    frames: Vec<HashMap<Ident, TyScheme>>,
}

impl TyCtxt {
    pub fn new() -> TyCtxt {
        TyCtxt {
            frames: Vec::new(),
        }
    }

    pub fn new_scope(&mut self) {
        self.frames.push(HashMap::new());
    }

    pub fn exit_scope(&mut self) {
        debug_assert!(!self.frames.is_empty(),
            "exit_scope called without entering any scopes");
        self.frames.pop();
    }

    pub fn add(&mut self, ident: Ident, ty: TyScheme) {
        debug_assert!(!self.frames.is_empty(),
            "insert called without entering any scopes");
        self.frames.last_mut().unwrap().insert(ident, ty);
    }

    pub fn get(&self, ident: &Ident) -> Option<&TyScheme> {
        for frame in self.frames.iter().rev() {
            if let Some(ty) = frame.get(ident) {
                return Some(ty);
            }
        }
        None
    }
}

impl TyCtxt {
    pub fn ftv(&self) -> BTreeSet<TyVar> {
        self.frames
            .iter()
            .flat_map(|frame| frame.values())
            .cloned()
            .collect::<Vec<_>>()
            .ftv()
    }

    pub fn apply(&mut self, subst: &Subst) {
        for frame in  &mut self.frames {
            for entry in frame.values_mut() {
                entry.ty = entry.ty.apply(subst);
            }
        }
    }
}
