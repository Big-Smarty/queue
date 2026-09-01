use std::sync::atomic::{AtomicPtr, Ordering};

pub struct Node<T> {
    pub data: T,
    pub next: AtomicPtr<Self>,
}

impl<T> Node<T> {
    pub unsafe fn next(node: *mut Self, ordering: Ordering) -> *mut Self {
        unsafe { (*node).next.load(ordering) }
    }
}
