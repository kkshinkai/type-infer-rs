// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.
//
// $ cargo run --example pprint

// TODO:
// - Improve the `Display` implementation for `Expr`;
// - Add macro for initializing `Expr`;

use type_infer_rs::program;

fn main() {
    let expr = program!("abs");
    println!("{:?}", expr);
}
