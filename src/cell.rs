pub use ghost_cell::{GhostCell, GhostToken as Token};

/// Create new `Token` for traversing AST.
///
/// # SAFETY
/// Caller must ensure only a single token exists at a time,
/// or that, if multiple tokens exist, only 1 is within scope of code doing an AST traversal.
pub unsafe fn new_token_unchecked<'t>() -> Token<'t> {
    std::mem::transmute(())
}

/// Macro to reduce boilerplate of `GhostCell` references.
/// `node_ref!(ExpressionStatement<'a, 't>)` -> `&'a GhostCell<'t, ExpressionStatement<'a, 't>>`
macro_rules! node_ref {
    ($ty:ident<$arena:lifetime, $token:lifetime>) => {
        &$arena $crate::cell::GhostCell<$token, $ty<$arena, $token>>
    };
}
pub(crate) use node_ref;
