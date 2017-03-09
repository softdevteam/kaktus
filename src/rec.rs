
use std::ops::Deref;
use std::rc::Rc;

use super::Stack;

struct StackNode<T> {
    value: T,
    parent: Option<RStack<T>>,
}


/// Recursive Stack
///
/// A **recursive** implementation for ``Stack``.
///
/// ```rust,ignore
/// pub struct StackNode<T> {
///     value: T,
///     parent: Option<RStack<T>>,
/// }
/// ```
///
pub struct RStack<T>(Rc<StackNode<T>>);

impl<T> Clone for RStack<T> {
    fn clone(&self) -> Self {
        RStack(self.0.clone())
    }
}

impl<T> Stack<T> for RStack<T> {
    fn root(val: T) -> Self {
        RStack(Rc::from(StackNode { value: val, parent: None }))
    }

    fn push(&self, val: T) -> Self {
        RStack(Rc::from(StackNode { value: val, parent: Some(RStack(self.0.clone())) }))
    }

    fn pop(&self) -> Option<Self> {
        match self.0.parent {
            None => None,
            Some(ref p) => Some(p.clone())
        }
    }
}

impl<T> Deref for RStack<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0.deref().value
    }
}
