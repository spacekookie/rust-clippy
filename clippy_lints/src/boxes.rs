use rustc::lint::{LateContext, LateLintPass, LintArray, LintPass};
use rustc::hir::*;
use rustc::hir::intravisit::*;
use syntax::ast::{LitKind, NodeId, DUMMY_NODE_ID};
use syntax::codemap::{dummy_spanned, Span, DUMMY_SP};
use syntax::util::ThinVec;
use utils::{in_macro, snippet_opt, span_lint_and_then, SpanlessEq};


/// **What it does:** Checks for Boxes around types that make no sense
/// 
/// 
declare_lint! {
    pub USELESS_BOX,
    Allow,
    "box around &T is probably useless"
}


pub struct UselessBox;

impl LintPass for UselessBox {
    fn get_lints(&self) -> LintArray {
        lint_array!(USELESS_BOX)
    }
}