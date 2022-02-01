macro_rules! wait {
    () => {
        return NodeOut::None
    };
}
macro_rules! next {
    ($a:expr) => {
        return NodeOut::Next($a)
    };
}
macro_rules! restart {
    () => {
        return NodeOut::Next(start)
    };
}
macro_rules! run {
    ($($token:tt)*) => {{
        restart!()
    }};
}
pub(crate) use next;
pub(crate) use restart;
pub(crate) use run;
pub(crate) use wait;