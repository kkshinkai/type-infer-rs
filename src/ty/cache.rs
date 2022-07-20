// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.

use std::collections::BTreeMap;

use crate::expr::expr_id::ExprId;

use super::Ty;

pub struct TyCache {
    cache: BTreeMap<ExprId, Ty>,
}

impl TyCache {
    pub fn new() -> TyCache {
        TyCache {
            cache: BTreeMap::new(),
        }
    }

    pub fn write(&mut self, expr_id: ExprId, ty: Ty) {
        self.cache.insert(expr_id, ty);
    }

    pub fn read(&self, expr_id: ExprId) -> Option<&Ty> {
        self.cache.get(&expr_id)
    }
}
