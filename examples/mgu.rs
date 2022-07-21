// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.
//
// $ cargo run --example mgu

use type_infer_rs::ty::{Ty, TyVar, subst::Subst};

fn main() {
    let ty1 = Ty::mk_int();
    let ty2 = Ty::mk_var(TyVar::new("a".to_string()));
    let subst = Subst::mgu(ty1.clone(), ty2.clone());
    println!("mgu of '{}' and '{}' is '{}'", ty1, ty2, subst.unwrap());

    let ty1 = Ty::mk_arrow(
        Ty::mk_var(TyVar::new("a".to_string())),
        Ty::mk_arrow(
            Ty::mk_var(TyVar::new("a".to_string())),
            Ty::mk_var(TyVar::new("b".to_string())),
        ),
    );
    let ty2 = Ty::mk_arrow(
        Ty::mk_var(TyVar::new("b".to_string())),
        Ty::mk_var(TyVar::new("c".to_string())),
    );
    let subst = Subst::mgu(ty1.clone(), ty2.clone());
    println!("mgu of '{}' and '{}' is '{}'", ty1, ty2, subst.unwrap());
}
