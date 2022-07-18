// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.
//
// $ cargo run --example subst

use type_infer_rs::subst;

use type_infer_rs::ty::{Ty, TyVar};

fn main() {
    let subst1 = subst![
        TyVar::new("a".to_string()) => Ty::mk_int(),
        TyVar::new("b".to_string()) => Ty::mk_arrow(
            Ty::mk_int(),
            Ty::mk_int(),
        ),
    ];

    let subst2 = subst![
        TyVar::new("c".to_string()) => Ty::Var(TyVar::new("b".to_string())),
    ];

    println!("S1      = {}", subst1);
    println!("S2      = {}", subst2);
    println!("S1 + S2 = {}", subst1.compose(&subst2));
}
