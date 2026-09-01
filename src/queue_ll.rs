use std::sync::{
    Arc,
    atomic::{AtomicPtr, AtomicUsize, Ordering},
};

pub mod list;
pub mod node;

#[cfg(test)]
mod tests;

pub fn queue_ll<T: Default, const L: usize>() -> (Owner<T, L>, Stealer<T, L>) {
    let inner = Arc::new(Inner::default());
    (
        Owner {
            inner: inner.clone(),
        },
        Stealer {
            inner: inner.clone(),
        },
    )
}

#[derive(Default)]
pub struct Owner<T, const L: usize> {
    inner: Arc<Inner<T, L>>,
}

#[derive(Default)]
pub struct Stealer<T, const L: usize> {
    inner: Arc<Inner<T, L>>,
}

impl<T: Default, const L: usize> Owner<T, L> {
    pub fn push(&self, nodes: list::List<T>) {
        self.inner.push(nodes);
    }

    pub fn pop(&self) -> *mut node::Node<T> {
        self.inner.pop()
    }
}

impl<T: Default, const L: usize> Stealer<T, L> {
    pub fn steal(&self, proportion: f64) -> list::List<T> {
        self.inner.steal(proportion)
    }
}

#[derive(Default)]
struct Inner<T, const L: usize> {
    size: AtomicUsize,
    head: AtomicPtr<node::Node<T>>,
}

impl<T: Default, const L: usize> Inner<T, L> {
    pub fn steal(&self, proportion: f64) -> list::List<T> {
        let proportion = 1.0 - proportion;
        let sz = self.size.load(Ordering::Acquire);

        if sz < Self::LIMIT {
            return list::List::default();
        }

        let mut n_skip = (sz as f64 * proportion) as usize;

        let k = n_skip;

        let mut start = self.head.load(Ordering::Acquire);
        while n_skip != 0 && !start.is_null() {
            start = unsafe { node::Node::next(start, Ordering::Acquire) };
            n_skip -= 1;
        }

        if n_skip != 0 || start.is_null() {
            return list::List::default();
        }

        let ssz = self.size.load(Ordering::Acquire);
        if ssz <= (sz - (k >> 1)) {
            return list::List::default();
        }

        let begin = unsafe { node::Node::next(start, Ordering::Acquire) };
        unsafe { std::ptr::read(start) }
            .next
            .store(std::ptr::null_mut(), Ordering::Relaxed);
        self.size.fetch_add(0, Ordering::Release);

        let mut end = begin;
        let mut count = 0;
        while !end.is_null() {
            count += 1;
            if unsafe { std::ptr::read(end) }.next.into_inner().is_null() {
                break;
            }
            end = unsafe { std::ptr::read(end) }.next.into_inner();
        }

        self.size.fetch_sub(count, Ordering::SeqCst);

        list::List {
            start: begin,
            end,
            size: count,
        }
    }

    pub fn pop(&self) -> *mut node::Node<T> {
        let rv = self.head.load(Ordering::Relaxed);

        if rv.is_null() {
            return std::ptr::null_mut();
        }

        self.head.store(
            unsafe { std::ptr::read(rv) }.next.into_inner(),
            Ordering::Relaxed,
        );

        self.size.fetch_sub(1, Ordering::AcqRel);

        unsafe { std::ptr::read(rv) }
            .next
            .store(std::ptr::null_mut(), Ordering::Relaxed);

        rv
    }

    pub fn push(&self, nodes: list::List<T>) {
        let start = nodes.start;
        let end = nodes.end;
        let n = nodes.size;

        (unsafe { std::ptr::read(end) })
            .next
            .store(self.head.load(Ordering::Relaxed), Ordering::Relaxed);

        self.head.store(start, Ordering::Release);
        self.size.fetch_add(n, Ordering::AcqRel);
    }

    const LIMIT: usize = L;
}
