use std::sync::atomic::AtomicPtr;

pub struct Node<T> {
    pub data: T,
    pub next: AtomicPtr<Self>,
}
