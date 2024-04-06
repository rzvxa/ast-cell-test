use oxc_allocator::Allocator;

pub use ghost_cell::{GhostCell, GhostToken as Token};

// Trait to add method to `Allocator` to allocate into arena and return a `GhostCell` ref.
// This would be implemented in `oxc_allocator`, and `galloc` would become `alloc`.
pub trait GhostAlloc {
    fn galloc<'t, T>(&self, t: T) -> &GhostCell<'t, T>;
}

impl GhostAlloc for Allocator {
    /// Move `T` into arena and return a `&GhostCell` containing it
    fn galloc<'t, T>(&self, t: T) -> &GhostCell<'t, T> {
        GhostCell::from_mut(self.alloc(t))
    }
}

/// Macro to reduce boilerplate of `GhostCell` references.
/// `node_ref!(ExpressionStatement<'a, 't>)` -> `&'a GhostCell<'t, ExpressionStatement<'a, 't>>`
macro_rules! node_ref {
    ($ty:ident<$arena:lifetime, $token:lifetime>) => {
        &$arena $crate::cell::GhostCell<$token, $ty<$arena, $token>>
    };
}
pub(crate) use node_ref;
