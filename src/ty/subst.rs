// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.

use std::{collections::BTreeMap, fmt};

use super::{TyVar, Ty};
use crate::{subst, ty::types::Types, error::{TyResult, TyError}};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Subst {
    mapping: BTreeMap<TyVar, Ty>,
}

impl Subst {
    /// Creates a identity substitution (aka. empty substitution) `[]`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use type_infer_rs::ty::{Ty, TyVar, subst::Subst};
    /// let var_a = Ty::Var(TyVar::new("a".to_string()));
    ///
    /// // Identity substitution maps every variable to itself.
    /// assert_eq!(var_a.apply(&Subst::identity()), var_a);
    /// ```
    pub fn identity() -> Subst {
        Subst {
            mapping: BTreeMap::new(),
        }
    }

    /// Trys to lookup type for a given type variable in the substitution.
    pub fn get(&self, var: &TyVar) -> Option<Ty> {
        self.mapping.get(var).cloned()
    }

    /// Adds a new mapping to the substitution.
    pub fn insert(&mut self, var: TyVar, ty: Ty) {
        // We named this function `insert` instead of `add` to indicate that
        // upcoming key-value pairs might overwrite the previous.
        self.mapping.insert(var, ty);
    }

    /// Removes a mapping from the substitution.
    pub fn remove(&mut self, var: &TyVar) -> Option<Ty> {
        self.mapping.remove(var)
    }

    /// Composes anonther substitution with this one.
    ///
    /// The composition `τ + σ` of two substitutions `σ = [x1: t1, ..., xN: tN]`
    /// and `τ = [y1: u1, ..., yM: uM]` is obtained by removing from the
    /// substitution `[x1: τ(t1), ..., xN: τ(tN), y1: u1, ..., yM: uM]` those
    /// pairs `yI: uI` for which `yI ∈ {x1, ..., xN}`. The composition of `τ`
    /// and `σ` is denoted by `τ + σ`, `(σ + τ)(x) = σ(τ(x))`.
    ///
    /// The composition of `τ` and `σ` is not commutative, you need to take care
    /// of the order. `τ.compose(&σ)` means `τ + σ`, not `σ + τ`.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[macro_use] extern crate type_infer_rs;
    /// # use type_infer_rs::ty::{Ty, TyVar, subst::Subst};
    ///
    /// // S1 = [t2: int, t3: bool], S2 = [t1: int -> t2, t3: int]
    /// let s1 = subst![
    ///     TyVar::new("t2".to_string()) => Ty::mk_int(),
    ///     TyVar::new("t3".to_string()) => Ty::mk_bool(),
    /// ];
    /// let s2 = subst![
    ///     TyVar::new("t1".to_string()) =>
    ///         Ty::mk_arrow(
    ///             Ty::mk_int(),
    ///             Ty::mk_var(TyVar::new("t2".to_string())),
    ///         ),
    ///    TyVar::new("t3".to_string()) => Ty::mk_int(),
    /// ];
    ///
    /// // S1 + S2 = [t1: int -> int, t2: int, t3: int]
    /// assert_eq!(s1.compose(&s2), subst![
    ///     TyVar::new("t1".to_string()) =>
    ///        Ty::mk_arrow(Ty::mk_int(), Ty::mk_int()),
    ///     TyVar::new("t2".to_string()) => Ty::mk_int(),
    ///     TyVar::new("t3".to_string()) => Ty::mk_int(),
    /// ]);
    /// ```
    ///
    pub fn compose(&self, other: &Subst) -> Subst {
        let mut subst = Subst::identity();

        self.iter()
            .for_each(|(var, ty)| subst.insert(var.clone(), ty.clone()));

        other.iter()
            .for_each(|(var, ty)| subst.insert(var.clone(), ty.apply(self)));

        subst
    }

    /// Binds a type variable to a type and return that binding as a
    /// substitution, but avoids binding a variable to itself and performs the
    /// occurs check. Constraints like `α = α` or `α = α -> β` are not allowed.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[macro_use] extern crate type_infer_rs;
    /// # use type_infer_rs::ty::{Ty, TyVar, subst::Subst};
    /// // bind(a, int) = [a: int]
    /// assert_eq!(
    ///     Subst::bind(TyVar::new("a".to_string()), Ty::mk_int()),
    ///     Ok(subst![TyVar::new("a".to_string()) => Ty::mk_int()]),
    /// );
    ///
    /// // bind(a, a -> b) = error
    /// assert!(Subst::bind(
    ///     TyVar::new("a".to_string()),
    ///     Ty::mk_arrow(
    ///         Ty::mk_var(TyVar::new("a".to_string())),
    ///         Ty::mk_var(TyVar::new("b".to_string())),
    ///     ),
    /// ).is_err());
    /// ```
    pub fn bind(var: TyVar, ty: Ty) -> TyResult<Subst> {
        match ty {
            Ty::Var(ref v) if v == &var =>
                Ok(Subst::identity()),
            _ if ty.ftv().contains(&var) =>
                Err(TyError::Unknown(format!("occur check fails {var} in {ty}"))),
            _ => Ok(subst![var => ty]),
        }
    }

    /// Finds the most general set of substitutions that can be found for two
    /// types. "MGU" means the most general unifier.
    pub fn mgu(ty1: Ty, ty2: Ty) -> TyResult<Subst> {
        match (ty1, ty2) {
            (Ty::Arrow(p1, r1), Ty::Arrow(p2, r2)) => {
                let s1 = Subst::mgu(*p1, *p2)?;
                let s2 = Subst::mgu(r1.apply(&s1), r2.apply(&s1))?;
                Ok(s1.compose(&s2))
            },
            (Ty::Var(var), ty) | (ty, Ty::Var(var)) =>
                Subst::bind(var, ty),
            (Ty::Int, Ty::Int) | (Ty::Bool, Ty::Bool) =>
                Ok(Subst::identity()),
            (ty1, ty2) =>
                Err(TyError::Unknown(format!("cannot unify {ty1} with {ty2}"))),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&TyVar, &Ty)> {
        self.mapping.iter()
    }
}

impl fmt::Display for Subst {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]",
            self.iter()
                .map(|(var, ty)| format!("{}: {}", var, ty))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

#[macro_export]
macro_rules! subst {
    ($($var:expr => $ty:expr),* $(,)?) => {{
        let mut subst = $crate::ty::subst::Subst::identity();
        for (var, ty) in [$(($var, $ty),)*].into_iter() {
            subst.insert(var, ty);
        }
        subst
    }};
}
