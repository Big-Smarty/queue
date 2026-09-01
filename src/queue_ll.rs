use std::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};

pub mod list;
pub mod node;

pub struct QueueLl<T> {
    size: AtomicUsize,
    head: AtomicPtr<node::Node<T>>,
}

impl<T> QueueLl<T> {
    pub fn steal(&mut self, proportion: f64) -> list::List<T> {
        todo!()
    }

    pub fn pop(&mut self) -> Option<*mut node::Node<T>> {
        let rv = self.head.load(Ordering::Relaxed);

        if rv.is_null() {
            return None;
        }

        self.head.store(
            unsafe { std::ptr::read(rv) }.next.into_inner(),
            Ordering::Relaxed,
        );

        self.size.fetch_sub(1, Ordering::AcqRel);

        unsafe { std::ptr::read(rv) }
            .next
            .store(std::ptr::null_mut(), Ordering::Relaxed);

        Some(rv)
    }

    pub fn push(&mut self, nodes: &list::List<T>) {
        let start = nodes.start;
        let end = nodes.end;
        let n = nodes.size;

        (unsafe { std::ptr::read(end) })
            .next
            .store(self.head.load(Ordering::Relaxed), Ordering::Relaxed);

        self.head.store(start, Ordering::Release);
        self.size.fetch_add(n, Ordering::AcqRel);
    }
}

impl<T> Default for QueueLl<T> {
    fn default() -> Self {
        Self {
            size: Default::default(),
            head: Default::default(),
        }
    }
}
