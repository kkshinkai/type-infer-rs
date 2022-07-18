// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.

#[macro_export]
macro_rules! program {
    ($lit:literal) => {
        $crate::expr::expr::Expr::new(
            $crate::expr::expr::ExprKind::Lit(
                $crate::expr::lit::ToLit::to_lit($lit)
            )
        )
    };
}
