// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.
//
// $ cargo run --example infer

use type_infer_rs::{
    expr::{expr::{Expr, Ident}, lit::Lit},
    infer::InferCtxt
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
            Expr::mk_app(
                Expr::mk_var(Ident::new("id".to_string())),
                Expr::mk_lit(Lit::Int(42)),
            ),
        ),
    ).assign_ids();

    let mut icx = InferCtxt::new();
    let ty = icx.infer(&expr).expect("");

    println!("{}", ty);
}
