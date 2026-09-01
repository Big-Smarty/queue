use crate::queue_ll::node;

#[derive(Default)]
pub struct List<T> {
    pub start: *mut node::Node<T>,
    pub end: *mut node::Node<T>,
    pub size: usize,
}

impl<T> List<T> {
    pub fn new(nodes: &[*mut node::Node<T>]) -> Self {
        Self {
            start: match nodes.first() {
                Some(first) => *first,
                None => std::ptr::null_mut(),
            },
            end: match nodes.last() {
                Some(last) => *last,
                None => std::ptr::null_mut(),
            },
            size: nodes.len(),
        }
    }
}
