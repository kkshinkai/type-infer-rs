// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.

use std::{collections::BTreeMap, fmt};

use super::{TyVar, Ty};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Subst {
    mapping: BTreeMap<TyVar, Ty>,
}

impl Subst {
    /// Creates a new empty substitution `[]`.
    pub fn new() -> Subst {
        Subst {
            mapping: BTreeMap::new(),
        }
    }

    /// Trys to lookup type for a given type variable in the substitution.
    pub fn get(&self, var: &TyVar) -> Option<Ty> {
        self.mapping.get(var).cloned()
    }

    /// Adds a new mapping to the substitution.
    pub fn add(&mut self, var: TyVar, ty: Ty) {
        self.mapping.insert(var, ty);
    }

/*
    /// Composes anonther substitution with this one.
    ///
    /// The result of composing two substitutions `S1` and `S2` is a new
    /// substitution `S = S1 ∘ S2` that for all type expressions `e`,
    /// `S(e) = S1(S2(e))`.
    ///
    /// For example,
    ///
    /// ```text
    /// S1 = [t2 ↦ int]
    /// S2 = [t1 ↦ int -> t2]
    /// S1 ∘ S2 = [t2 ↦ int, t1 ↦ int -> int]
    /// ```
    ///
    /// Note that `S1 ∘ S2` is not just the union of the mappings in `S1` and
    /// `S2`, i.e., it is not equal to `[t2 ↦ int, t1 ↦ int -> t2]`.
    pub fn compose(&self, other: &Subst) -> Subst {
        other.mapping.iter()
            .map(|(var, ty)| ty.apply(self))
    }
*/
}

impl fmt::Display for Subst {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]",
            self.mapping.iter()
                .map(|(var, ty)| format!("{}: {}", var, ty))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

#[macro_export]
macro_rules! subst {
    ($($var:expr => $ty:expr),* $(,)?) => {{
        let mut subst = $crate::ty::subst::Subst::new();
        for (var, ty) in [$(($var, $ty),)*].into_iter() {
            subst.add(var, ty);
        }
        subst
    }};
}
