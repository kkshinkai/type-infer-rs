// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.
//
// $ cargo run --example pprint

// TODO:
// - Improve the `Display` implementation for `Expr`;
// - Add macro for initializing `Expr`;

use type_infer_rs::expr::*;

fn main() {

    let expr = Expr::new(ExprKind::let_in(
        Ident::new("number".to_string()),
        Expr::new(ExprKind::Lit(Lit::Int(42))),
        Expr::new(ExprKind::Var(Ident::new("number".to_string()))),
    ));

    println!("{}", expr);
}
