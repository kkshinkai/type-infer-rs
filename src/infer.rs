// Copyright (c) Kk Shinkai. All Rights Reserved. See LICENSE.txt in the project
// root for license information.

use std::collections::BTreeMap;

use crate::{ty::{TyVar, ty_ctxt::TyCtxt, ty_scheme::TyScheme, Ty, types::Types, subst::Subst}, expr::{expr::{Expr, ExprKind}, lit::Lit}, error::{TyResult, TyError}};

pub struct InferCtxt {
    used_type_var_id: u32,
}

impl InferCtxt {
    pub fn new() -> InferCtxt {
        InferCtxt {
            used_type_var_id: 0,
        }
    }

    fn new_type_var(&mut self) -> TyVar {
        let id = self.used_type_var_id;
        self.used_type_var_id += 1;
        TyVar::Unknown(id)
    }

    /// Abstracts a type over all type variables which are free in the type but
    /// not free in the given type environment.
    ///
    /// ```text
    /// Γ ⊢ e : σ     σ′ ∉ ftv(Γ)
    /// ------------------------- (Gen)
    ///      Γ ⊢ e : ∀ α . σ
    /// ```
    pub fn generalize(&self, tcx: TyCtxt, ty: Ty) -> TyScheme {
        let vars = ty.ftv().difference(&tcx.ftv()).cloned().collect::<Vec<_>>();
        TyScheme::mk_forall(vars, ty)
    }

    /// Replaces all bound type variables in a type scheme with fresh type
    /// variables.
    ///
    /// ```text
    /// Γ ⊢ e : σ′    σ′ ⊑ σ
    /// --------------------- (Inst)
    ///       Γ ⊢ e : σ
    /// ```
    pub fn instantiate(&mut self, tys: TyScheme) -> Ty {
        let mut subst = Subst::identity();
        for var in tys.vars {
            subst.insert(var, Ty::mk_var(self.new_type_var()));
        }
        tys.ty.apply(&subst)
    }

    fn infer_impl(&mut self, tcx: TyCtxt, expr: &Expr) -> TyResult<(Subst, Ty)> {
        match expr.kind {
            // x : σ ∈ Γ
            // --------- (Var)
            // Γ ⊢ x : σ
            ExprKind::Var(ref name) => {
                // Lookup in the context to check if it contains an entry for
                // the variable. If it doesn't then the variable must be
                // unbound.
                if let Some(tys) = tcx.get(name) {
                    Ok((Subst::identity(), self.instantiate(tys.clone())))
                } else {
                    Err(TyError::Unknown(format!("unbound variable {name}")))
                }
            },

            ExprKind::Lit(ref lit) => {
                match lit {
                    //
                    // --−−−−−−−------- (Int)
                    // Γ ⊢ [0-9]+ : int
                    Lit::Int(_) => Ok((Subst::identity(), Ty::mk_int())),

                    //
                    // --−−−−−−−------ (Bool)
                    // Γ ⊢ true : bool
                    //
                    // --−−−−−−−------- (Bool)
                    // Γ ⊢ false : bool
                    Lit::Bool(_) => Ok((Subst::identity(), Ty::mk_bool())),
                }
            },

            //   Γ, x : τ ⊢ e : τ′
            // −−−−−−−−−−−−−−------- (Abs)
            // Γ ⊢ λ x . e : τ → τ′
            ExprKind::Abs { ref param, ref body } => {
                // Add the new type variable of the parameter to the context,
                // then infer the body with this new context.
                let new_ty = Ty::mk_var(self.new_type_var());
                let mut new_tcx = tcx;
                new_tcx.insert(
                    param.clone(),
                    TyScheme::mk_forall(vec![], new_ty.clone()),
                );
                let (subst, ty) = self.infer_impl(new_tcx, &body)?;

                // FIXME: Remove this `clone` in `subst.clone()`.
                Ok((subst.clone(), Ty::mk_arrow(new_ty.apply(&subst), ty)))
            },

            // Γ ⊢ e0 : τ → τ′   Γ ⊢ e1 : τ
            // −--------−−−−−−−−−−−−−−−−−−−− (App)
            //          Γ ⊢ e0(e1) : τ′
            ExprKind::App { ref callee, ref arg } => {
                let new_ty = Ty::mk_var(self.new_type_var());
                let (s1, ty1) = self.infer_impl(tcx.clone(), &callee)?;
                let (s2, ty2) = self.infer_impl(tcx.apply(&s1), &arg)?;
                let s3 = Subst::mgu(
                    ty1.apply(&s2),
                    Ty::mk_arrow(ty2, new_ty.clone()),
                )?;
                Ok((s3.compose(&s2).compose(&s1), new_ty.apply(&s3)))
            },

            // Γ ⊢ e0 : σ     Γ, x : σ ⊢ e1 : τ
            // −------------−−−−−−−−−−−−−−−−−−−− (Let)
            //     Γ ⊢ let x = e0 in e1 : τ
            ExprKind::Let { ref name, ref value, ref body } => {
                let (s1, t1) = self.infer_impl(tcx.clone(), &value)?;
                let s = self.generalize(tcx.apply(&s1), t1);
                let mut new_tcx = tcx;
                new_tcx.insert(name.clone(), s.clone());
                let (s2, t2) = self.infer_impl(new_tcx.apply(&s1), &body)?;
                Ok((s2.compose(&s1), t2))
            },
        }
    }

    pub fn infer(&mut self, expr: &Expr) -> TyResult<Ty> {
        let (s, t) = self.infer_impl(TyCtxt::new(), expr)?;
        Ok(t.apply(&s))
    }
}
