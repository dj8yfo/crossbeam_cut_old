use std::sync::atomic::Ordering::{Acquire, Relaxed, Release};
use std::ptr;

use epoch::{self, Atomic, Owned};

/// Treiber's lock-free stack.
///
/// Usable with any number of producers and consumers.
#[derive(Debug)]
pub struct TreiberStack<T> {
    head: Atomic<Node<T>>,
}

#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Atomic<Node<T>>,
}

impl<T> TreiberStack<T> {
    /// Create a new, empty stack.
    pub fn new() -> TreiberStack<T> {
        TreiberStack {
            head: Atomic::null(),
        }
    }

    /// Push `t` on top of the stack.
    pub fn push(&self, t: T) {
        let mut n = Owned::new(Node {
            data: t,
            next: Atomic::null(),
        });
        let guard = epoch::pin();
        loop {
            let head = self.head.load(Relaxed, &guard);
            n.next.store(head, Relaxed);
            match self.head.compare_and_set(head, n, Release, &guard) {
                Ok(_) => break,
                Err(e) => n = e.new,
            }
        }
    }

    /// Attempt to pop the top element of the stack.
    /// **Deprecated method**, use try_pop
    ///
    /// Returns `None` if the stack is observed to be empty.
    #[cfg_attr(any(feature="beta", feature="nightly"), deprecated(note="The pop method has been renamed to try_pop for consistency with other collections."))]
    pub fn pop(&self) -> Option<T> {
        self.try_pop()
    }

    /// Attempt to pop the top element of the stack.
    ///
    /// Returns `None` if the stack is observed to be empty.
    pub fn try_pop(&self) -> Option<T> {
        let guard = epoch::pin();
        loop {
            let head_shared = self.head.load(Acquire, &guard);
            match unsafe { head_shared.as_ref() } {
                Some(head) => {
                    let next = head.next.load(Relaxed, &guard);
                    if self.head
                        .compare_and_set(head_shared, next, Release, &guard)
                        .is_ok()
                    {
                        unsafe {
                            guard.defer_destroy(head_shared);
                            return Some(ptr::read(&(*head).data));
                        }
                    }
                }
                None => return None,
            }
        }
    }

    /// Check if this queue is empty.
    pub fn is_empty(&self) -> bool {
        let guard = epoch::pin();
        self.head.load(Acquire, &guard).is_null()
    }
}

impl<T> Drop for TreiberStack<T> {
    fn drop(&mut self) {
        while self.try_pop().is_some() {}
    }
}

impl<T> Default for TreiberStack<T> {
    fn default() -> Self {
        TreiberStack::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn is_empty() {
        let q: TreiberStack<i64> = TreiberStack::new();
        assert!(q.is_empty());
        q.push(20);
        q.push(20);
        assert!(!q.is_empty());
        assert!(!q.is_empty());
        assert!(q.try_pop().is_some());
        assert!(q.try_pop().is_some());
        assert!(q.is_empty());
        q.push(25);
        assert!(!q.is_empty());
    }
}
