// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.
//
// $ cargo run --example subst

use type_infer_rs::subst;

use type_infer_rs::ty::{Ty, TyVar};

fn main() {
    let subst = subst![
        TyVar::new("a".to_string()) => Ty::mk_int(),
        TyVar::new("b".to_string()) => Ty::mk_arrow(
            Ty::mk_int(),
            Ty::mk_int(),
        ),
    ];

    println!("{}", subst);
}
