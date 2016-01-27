//! Borrowing module.
//!
//! This is a not particularly safe interface to types with internal mutability.
//! Mainly a makeshift abstraction over `Rc<RefCell<T>>`, `Arc<RwLock<T>>` and
//! `Arc<Mutex<T>>`.

use std::rc::Rc;
use std::cell::RefCell;
use std::sync::{Arc, RwLock, Mutex};


/// Abstraction of borrowing operating on a value with internal mutability.
pub trait Borrowing {
    /// The dereferenced target.
    type Target;

    /// Do something with an immutable reference to the target.
    fn with<A, F: FnOnce(&Self::Target) -> A>(&self, f: F) -> A;

    /// Do something with a mutable reference to the target.
    fn with_mut<A, F: FnOnce(&mut Self::Target) -> A>(&mut self, f: F) -> A;
}

impl<T> Borrowing for Rc<RefCell<T>> {
    type Target = T;

    fn with<A, F: FnOnce(&Self::Target) -> A>(&self, f: F) -> A {
        f(&self.borrow())
    }

    fn with_mut<A, F: FnOnce(&mut Self::Target) -> A>(&mut self, f: F) -> A {
        f(&mut self.borrow_mut())
    }
}

impl<T> Borrowing for Arc<RwLock<T>> {
    type Target = T;

    fn with<A, F: FnOnce(&Self::Target) -> A>(&self, f: F) -> A {
        f(&self.read().unwrap())
    }

    fn with_mut<A, F: FnOnce(&mut Self::Target) -> A>(&mut self, f: F) -> A {
        f(&mut self.write().unwrap())
    }
}

impl<T> Borrowing for Arc<Mutex<T>> {
    type Target = T;

    fn with<A, F: FnOnce(&Self::Target) -> A>(&self, f: F) -> A {
        f(&self.lock().unwrap())
    }

    fn with_mut<A, F: FnOnce(&mut Self::Target) -> A>(&mut self, f: F) -> A {
        f(&mut self.lock().unwrap())
    }
}
