// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.

use std::collections::BTreeSet;

use super::{TyVar, subst::Subst};

pub trait Types {
    fn ftv(&self) -> BTreeSet<TyVar>;
    fn apply(&self, subst: &Subst) -> Self;
}

// FIXME: We should implement the `Types` trait for `I: Iterator` instead of
// `Vec<_>`, but I have no idea how to implement it yet.

impl<T: Types> Types for Vec<T> {
    fn ftv(&self) -> BTreeSet<TyVar> {
        self.iter()
            .map(|x| x.ftv())
            .fold(BTreeSet::new(), |set, x| set.union(&x).cloned().collect())
    }

    fn apply(&self, s: &Subst) -> Vec<T> {
        self.iter().map(|x| x.apply(s)).collect()
    }
}
