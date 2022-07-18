// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.
//
// $ cargo run --example pprint

// TODO:
// - Improve the `Display` implementation for `Expr`;
// - Add macro for initializing `Expr`;

use type_infer_rs::expr::*;

fn main() {
    let expr_kind = ExprKind::Let {
        name: Ident { name: "number".to_string() },
        value: Box::new(Expr { kind: ExprKind::Lit(Lit::Int(42)) }),
        body: Box::new(Expr {
            kind: ExprKind::Var(Ident { name: "number".to_string() }),
        }),
    };

    let expr = Expr { kind: expr_kind };

    println!("{}", expr);
}
