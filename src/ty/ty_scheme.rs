// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.

use std::{fmt, collections::BTreeSet};

use super::{TyVar, Ty, types::Types, subst::Subst};


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

impl Types for TyScheme {
    fn ftv(&self) -> BTreeSet<TyVar> {
        self.ty
            .ftv()
            .difference(&self.vars.iter().cloned().collect())
            .cloned()
            .collect()
    }

    /// Apply a substitution to a type scheme.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[macro_use] extern crate type_infer_rs;
    /// # use type_infer_rs::ty::ty_scheme::TyScheme;
    /// # use type_infer_rs::ty::types::Types;
    /// # use type_infer_rs::ty::{Ty, TyVar, subst::Subst};
    /// // t : forall a. a -> b
    /// let ty = TyScheme::mk_forall(
    ///     vec![TyVar::new("a".to_string())],
    ///     Ty::mk_arrow(
    ///         Ty::mk_var(TyVar::new("a".to_string())),
    ///         Ty::mk_var(TyVar::new("b".to_string())),
    ///     ),
    /// );
    ///
    /// // S = [a: int, b: bool]
    /// let subst = subst![
    ///     TyVar::new("a".to_string()) => Ty::mk_int(),
    ///     TyVar::new("b".to_string()) => Ty::mk_bool(),
    /// ];
    ///
    /// // S(t) = forall a. a -> bool
    /// assert_eq!(ty.apply(&subst),
    ///     TyScheme::mk_forall(
    ///         vec![TyVar::new("a".to_string())],
    ///         Ty::mk_arrow(
    ///             Ty::mk_var(TyVar::new("a".to_string())),
    ///             Ty::mk_bool(),
    ///         ),
    ///     ),
    /// );
    /// ```
    fn apply(&self, subst: &Subst) -> TyScheme {
        // Remove bound variables from the substitution.
        let subst = self.vars
            .iter()
            .fold(subst.clone(), |mut sub, var| { sub.remove(var); sub });

        TyScheme::mk_forall(
            self.vars.clone(),
            self.ty.apply(&subst),
        )
    }
}

impl fmt::Display for TyScheme {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "forall {} . {}",
            self.vars.iter()
                .map(|var| format!("{}", var))
                .collect::<Vec<String>>()
                .join(" "),
            self.ty
        )
    }
}
