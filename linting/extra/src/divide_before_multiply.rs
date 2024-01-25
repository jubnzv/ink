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
use rustc_hir::Expr;
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

impl<'tcx> LateLintPass<'tcx> for DivideBeforeMultiply {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        todo!()
    }
}
