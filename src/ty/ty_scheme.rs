// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.

use std::fmt;

use super::{TyVar, Ty};


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
                .map(|var| format!("{}", var))
                .collect::<Vec<String>>()
                .join(" "),
            self.ty
        )
    }
}
