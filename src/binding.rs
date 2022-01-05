use std::{
    env,
    ops::{Add, BitOr, Not, Shr, Sub},
};

#[derive(Clone)]
pub enum KeyTree {
    // e.g.: Single(x) ; Single(c)
    Sequence(Box<KeyTree>, Box<KeyTree>),
    // e.g.: Single(super) + Single(c)
    All(Box<KeyTree>, Box<KeyTree>),
    // e.g.: {Single(c), Single(n)}
    Any(Box<KeyTree>, Box<KeyTree>),
    // e.g.: c
    Single { key: xcb::Keysym, replay: bool },
}

/// requires two expressions active at the same time
impl Add for KeyTree {
    type Output = KeyTree;

    fn add(self, rhs: Self) -> Self::Output {
        KeyTree::All(Box::new(self), Box::new(rhs))
    }
}
impl Add<&KeyTree> for KeyTree {
    type Output = KeyTree;

    fn add(self, rhs: &KeyTree) -> Self::Output {
        self + rhs.clone()
    }
}
impl Add<&KeyTree> for &KeyTree {
    type Output = KeyTree;

    fn add(self, rhs: &KeyTree) -> Self::Output {
        self.clone() + rhs.clone()
    }
}
impl Add<KeyTree> for &KeyTree {
    type Output = KeyTree;

    fn add(self, rhs: KeyTree) -> Self::Output {
        self.clone() + rhs
    }
}

/// requires at least one expression active at the same time
impl BitOr for KeyTree {
    type Output = KeyTree;

    fn bitor(self, rhs: Self) -> Self::Output {
        KeyTree::Any(Box::new(self), Box::new(rhs))
    }
}
impl BitOr<&KeyTree> for KeyTree {
    type Output = KeyTree;

    fn bitor(self, rhs: &KeyTree) -> Self::Output {
        self | rhs.clone()
    }
}
impl BitOr<&KeyTree> for &KeyTree {
    type Output = KeyTree;

    fn bitor(self, rhs: &KeyTree) -> Self::Output {
        self.clone() | rhs.clone()
    }
}
impl BitOr<KeyTree> for &KeyTree {
    type Output = KeyTree;

    fn bitor(self, rhs: KeyTree) -> Self::Output {
        self.clone() | rhs
    }
}

/// requires the first expression to be active, then requires the second expression to be active.
impl Sub for KeyTree {
    type Output = KeyTree;

    fn sub(self, rhs: Self) -> Self::Output {
        KeyTree::Sequence(Box::new(self), Box::new(rhs))
    }
}
impl Sub<&KeyTree> for KeyTree {
    type Output = KeyTree;

    fn sub(self, rhs: &KeyTree) -> Self::Output {
        self - rhs.clone()
    }
}
impl Sub<&KeyTree> for &KeyTree {
    type Output = KeyTree;

    fn sub(self, rhs: &KeyTree) -> Self::Output {
        self.clone() - rhs.clone()
    }
}
impl Sub<KeyTree> for &KeyTree {
    type Output = KeyTree;

    fn sub(self, rhs: KeyTree) -> Self::Output {
        self.clone() - rhs
    }
}

/// Toggle the replay key
impl Not for KeyTree {
    type Output = KeyTree;

    fn not(self) -> Self::Output {
        match self {
            KeyTree::Single { key, replay } => KeyTree::Single {
                key,
                replay: !replay,
            },
            _ => panic!("You can only replay a single key!"),
        }
    }
}
impl Not for &KeyTree {
    type Output = KeyTree;

    fn not(self) -> Self::Output {
        !self.clone()
    }
}

/// consume key from other clients
pub const fn key(key: xcb::Keysym) -> KeyTree {
    KeyTree::Single { replay: false, key }
}

pub struct Binding {
    key_tree: KeyTree,
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

pub fn bind(key_tree: KeyTree) -> Binding {
    Binding {
        key_tree,
        callback: None,
    }
}
#[macro_export]
macro_rules! run {
    ($($t:tt)*) => {{}};
}
pub(crate) use run;

pub fn run_checked(path: &str) -> Result<(), ()> {
    Ok(())
}