// Copyright (C) Parity Technologies (UK) Ltd.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use if_chain::if_chain;
use ink_linting_utils::clippy::{
    diagnostics::span_lint_and_then,
    peel_hir_expr_refs,
    source::snippet_opt,
};
use rustc_errors::Applicability;
use rustc_hir::{
    BinOpKind,
    Expr,
    ExprKind,
};
use rustc_lint::{
    LateContext,
    LateLintPass,
};
use rustc_session::{
    declare_lint,
    declare_lint_pass,
};

declare_lint! {
    /// ## What it does
    /// Highlights arithmetic expressions in which division is performed before multiplication.
    ///
    /// ## Why is this bad?
    /// Integer division might unexpectedly result with 0.
    ///
    /// ## Example
    /// Bad:
    ///
    /// ```rust
    /// let a: u32 = 2;
    /// let b: u32 = 4;
    /// let c: u32 = a / b * 8; // c == 0
    /// ```
    ///
    /// Use the following:
    ///
    /// ```rust
    /// let a: u32 = 2;
    /// let b: u32 = 4;
    /// let c: u32 = a * 8 / b; // c == 4
    /// ```
    ///
    pub DIVIDE_BEFORE_MULTIPLY,
    Warn,
    "division before multiplication"
}

declare_lint_pass!(DivideBeforeMultiply => [DIVIDE_BEFORE_MULTIPLY]);

impl DivideBeforeMultiply {
    /// Returns true if the given expression has integral type
    fn has_integral_ty(self, cx: &LateContext<'_>, expr: &Expr<'_>) -> bool {
        if !cx.tcx.has_typeck_results(expr.hir_id.owner.to_def_id()) {
            return false;
        }
        let (actual_expr, _) = peel_hir_expr_refs(expr);
        cx.typeck_results()
            .expr_ty(actual_expr)
            .peel_refs()
            .is_integral()
    }

    /// Detects expression patterns that represent binary operations or method calls,
    /// returning their LHS and RHS if found.
    fn find_method_or_op<'tcx>(
        self,
        expr: &'tcx Expr<'tcx>,
        method_name: &str,
        bin_op: BinOpKind,
    ) -> Option<(&'tcx Expr<'tcx>, &'tcx Expr<'tcx>)> {
        match expr.kind {
            ExprKind::Binary(op, lhs, rhs) if op.node == bin_op => Some((lhs, rhs)),
            ExprKind::MethodCall(path, receiver, [arg], _)
                if path.ident.name.as_str() == method_name =>
            {
                Some((receiver, arg))
            }
            _ => None,
        }
    }

    // If the given expression represents a multiplication operation, returns its LHS and
    // RHS
    fn is_mul<'tcx>(
        self,
        expr: &'tcx Expr<'tcx>,
    ) -> Option<(&'tcx Expr<'tcx>, &'tcx Expr<'tcx>)> {
        self.find_method_or_op(expr, "saturating_mul", BinOpKind::Mul)
    }

    // If the given expression represents a division operation, returns its LHS and RHS
    fn is_div<'tcx>(
        self,
        expr: &'tcx Expr<'tcx>,
    ) -> Option<(&'tcx Expr<'tcx>, &'tcx Expr<'tcx>)> {
        self.find_method_or_op(expr, "saturating_div", BinOpKind::Div)
    }
}

impl<'tcx> LateLintPass<'tcx> for DivideBeforeMultiply {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if_chain! {
            // Look for the following pattern: a * b / c
            if let Some((lhs, c)) = self.is_mul(expr);
            if let Some((a, b)) = self.is_div(lhs);
            if self.has_integral_ty(cx, a) && self.has_integral_ty(cx, b) && self.has_integral_ty(cx, c);
            then {
                span_lint_and_then(
                    cx,
                    DIVIDE_BEFORE_MULTIPLY,
                    expr.span,
                    "division performed before multiplication",
                    |diag| {
                        let snippet = snippet_opt(cx, expr.span).expect("snippet must exist");
                        diag.span_suggestion(
                            expr.span,
                            "consider rearranging the order of arithmetic operations".to_string(),
                            snippet,
                            Applicability::Unspecified,
                        );
                    },
                )
            }
        }
    }
}
