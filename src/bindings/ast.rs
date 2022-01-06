use std::{
    ops::{Add, BitOr, Not, Sub},
};

use crate::display::{Keysym, Button, Modifier};

#[derive(Clone)]
pub enum AST {
    // e.g.: Key(x) ; Key(c)
    Sequence(Box<AST>, Box<AST>),
    // e.g.: Key(super) + Key(c)
    All(Box<AST>, Box<AST>),
    // e.g.: {Key(c), Key(n)}
    Any(Box<AST>, Box<AST>),
    // e.g.: c
    Key { key: Keysym, replay: bool },
    // e.g.: right mouse button
    Button { button: Button, replay: bool },
    // e.g.: Scroll lock
    Modifier { modifier: Modifier, replay: bool },
}

/// requires two expressions active at the same time
impl Add for AST {
    type Output = AST;

    fn add(self, rhs: Self) -> Self::Output {
        AST::All(Box::new(self), Box::new(rhs))
    }
}
impl Add<&AST> for AST {
    type Output = AST;

    fn add(self, rhs: &AST) -> Self::Output {
        self + rhs.clone()
    }
}
impl Add<&AST> for &AST {
    type Output = AST;

    fn add(self, rhs: &AST) -> Self::Output {
        self.clone() + rhs.clone()
    }
}
impl Add<AST> for &AST {
    type Output = AST;

    fn add(self, rhs: AST) -> Self::Output {
        self.clone() + rhs
    }
}

/// requires at least one expression active at the same time
impl BitOr for AST {
    type Output = AST;

    fn bitor(self, rhs: Self) -> Self::Output {
        AST::Any(Box::new(self), Box::new(rhs))
    }
}
impl BitOr<&AST> for AST {
    type Output = AST;

    fn bitor(self, rhs: &AST) -> Self::Output {
        self | rhs.clone()
    }
}
impl BitOr<&AST> for &AST {
    type Output = AST;

    fn bitor(self, rhs: &AST) -> Self::Output {
        self.clone() | rhs.clone()
    }
}
impl BitOr<AST> for &AST {
    type Output = AST;

    fn bitor(self, rhs: AST) -> Self::Output {
        self.clone() | rhs
    }
}

/// requires the first expression to be active, then requires the second expression to be active.
impl Sub for AST {
    type Output = AST;

    fn sub(self, rhs: Self) -> Self::Output {
        AST::Sequence(Box::new(self), Box::new(rhs))
    }
}
impl Sub<&AST> for AST {
    type Output = AST;

    fn sub(self, rhs: &AST) -> Self::Output {
        self - rhs.clone()
    }
}
impl Sub<&AST> for &AST {
    type Output = AST;

    fn sub(self, rhs: &AST) -> Self::Output {
        self.clone() - rhs.clone()
    }
}
impl Sub<AST> for &AST {
    type Output = AST;

    fn sub(self, rhs: AST) -> Self::Output {
        self.clone() - rhs
    }
}

/// Toggle the replay key
impl Not for AST {
    type Output = AST;

    fn not(self) -> Self::Output {
        match self {
            AST::Key { key, replay } => AST::Key {
                key,
                replay: !replay,
            },
            _ => panic!("You can only replay a single key!"),
        }
    }
}
impl Not for &AST {
    type Output = AST;

    fn not(self) -> Self::Output {
        !self.clone()
    }
}