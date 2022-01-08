use super::AST;
use crate::{bindings::run, hot_key_daemon::HotKeyDaemon, display};
use std::ops::Shr;

pub struct Binding {
    key_tree: AST,
    callback: Option<Box<dyn FnMut(usize, &mut HotKeyDaemon) + 'static>>,
}

impl Binding {
    fn handle_event(&mut self, event: display::DisplayServerEvent) {
        // TODO: call "callback"
    }
    fn then(mut self, f: impl FnMut(usize, &mut HotKeyDaemon) + 'static) -> Self {
        self.callback = Some(Box::new(f));
        self
    }
    fn then_run(self, path: &str) -> Self {
        self.then(|_, _| {
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

impl<T: FnMut(usize, &mut HotKeyDaemon) + 'static> Shr<T> for Binding {
    type Output = Self;

    fn shr(self, rhs: T) -> Self::Output {
        self.then(rhs)
    }
}
