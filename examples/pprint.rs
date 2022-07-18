// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.
//
// $ cargo run --example pprint

// TODO:
// - Improve the `Display` implementation for `Expr`;
// - Add macro for initializing `Expr`;

use type_infer_rs::expr::{expr::{Expr, Ident}, lit::Lit};

fn main() {
    let expr = Expr::mk_let(
        Ident::new("id".to_string()),
        Expr::mk_abs(Ident::new("x".to_string()), Expr::mk_var(Ident::new("x".to_string()))),
        Expr::mk_app(
            Expr::mk_var(Ident::new("id".to_string())),
            Expr::mk_lit(Lit::Int(42)),
        ),
    ).assign_ids();

    println!("{:#?}", expr);
}
