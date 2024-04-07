use std::{
    cell::{Cell, UnsafeCell},
    marker::PhantomData,
};

use crate::cell::{GCell, Token};

pub struct TraverseCtx<'a, 't> {
    // token: &'a mut Token<'t>,
    token: UnsafeCell<&'a mut Token<'t>>,

    /// It is only defined to make sure the `TraverseCtx` is `!Sync`. Since negative traits are nighly.
    /// We basically want our type to behave similar to the `Cell` since we both wrap `UnsafeCell`,
    /// In a similar manner.
    _cell_marker: PhantomData<Cell<&'a mut Token<'t>>>,
}

impl<'a, 't> TraverseCtx<'a, 't> {
    pub fn new(token: &'a mut Token<'t>) -> Self {
        Self {
            // token,
            token: UnsafeCell::new(token),
            _cell_marker: PhantomData {},
        }
    }

    pub fn get_node<'b, T>(&'b self, node_ref: &'b GCell<'t, T>) -> &'b T
    // where
    //     'a: 'b,
    {
        // SAFETY: This can cause data races if called from a separate thread,
        // but `TraverseCtx` is `!Sync` so this won't happen.
        let tk = unsafe { &*self.token.get() };
        node_ref.borrow(tk)

        // node_ref.borrow(self.token)
        // the`tk` reference gets dropped here, So after this call the cell is safe to use again!
    }

    pub fn get_node_mut<'b, T>(&'b self, node_ref: &'b GCell<'t, T>) -> &'b mut T
    // where
    //     'a: 'b,
    {
        // SAFETY: This can cause data races if called from a separate thread,
        // but `TraverseCtx` is `!Sync` so this won't happen.
        let tk = unsafe { &mut *self.token.get() };
        node_ref.borrow_mut(tk)

        // node_ref.borrow_mut(self.token)

        // the`tk` reference gets dropped here, So after this call the cell is safe to use again!
    }
}
