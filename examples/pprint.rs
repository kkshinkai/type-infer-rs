// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.
//
// $ cargo run --example pprint

use type_infer_rs::{
    expr::{expr::{Expr, Ident}, lit::Lit},
    ty::{Ty, TyVar, ty_scheme::TyScheme}
};

fn main() {
    let expr = Expr::mk_let(
        Ident::new("id".to_string()),
        Expr::mk_abs(
            Ident::new("x".to_string()),
            Expr::mk_var(Ident::new("x".to_string())),
        ),
        Expr::mk_app(
            Expr::mk_var(Ident::new("id".to_string())),
            Expr::mk_lit(Lit::Int(42)),
        ),
    ).assign_ids();

    println!("{}", expr);

    let ty = TyScheme::mk_forall(
        vec![
            TyVar::new("a".to_string()),
            TyVar::new("b".to_string()),
        ],
        Ty::mk_arrow(
            Ty::mk_var(TyVar::new("a".to_string())),
            Ty::mk_arrow(
                Ty::mk_var(TyVar::new("b".to_string())),
                Ty::mk_int(),
            ),
        ),
    );

    println!("{}", ty);
}
