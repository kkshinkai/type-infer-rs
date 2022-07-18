// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.

use std::collections::BTreeMap;

use super::{TyVar, ty_scheme::TyScheme};

pub struct TyCtxt {
    types: BTreeMap<TyVar, TyScheme>,
}
