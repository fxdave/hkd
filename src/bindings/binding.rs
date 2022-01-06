use super::AST;
use crate::bindings::run;
use std::ops::Shr;

pub struct Binding {
    key_tree: AST,
    callback: Option<Box<dyn FnMut(usize)>>,
}

impl Binding {
    fn then(self, f: impl FnMut(usize)) -> Self {
        self
    }
    fn then_run(self, path: &str) -> Self {
        self.then(|_| {
            run!(path);
        })
    }
}

pub fn bind(key_tree: AST) -> Binding {
    Binding {
        key_tree,
        callback: None,
    }
}

/// More readable way than "then_run"
impl Shr<&str> for Binding {
    type Output = Self;

    fn shr(self, rhs: &str) -> Self::Output {
        self.then_run(rhs)
    }
}

impl<T: FnMut(usize)> Shr<T> for Binding {
    type Output = Self;

    fn shr(self, rhs: T) -> Self::Output {
        self.then(rhs)
    }
}
