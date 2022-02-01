
/// A way to simplify sequences like this:
/// ```
/// if key.super_l() && key.c() {
///     next!(|key, _state| {
///         if key.esc() { restart!() }
///         if key.shift() && key.n() { run!("nm-connection-editor") }
///         if key.n() { run!(urxvt "-e" nmtui) }
///         if key.w() { run!("~/.config/bspwm/toggle_rfkill.sh") }
///         if key.a() {
///             next!(|key, _state| {
///                 if key.esc() { restart!() }
///                 if key.a() { run!(pavucontrol) }
///                 if key.j() { run!("pw-jack" catia) }
///                 if key.s() {
///                     next!(|key, _state| {
///                         if key.esc() { restart!() }
///                         if key.l() { run!("notify-send" "TODO" "'switch sound to laptop'") }
///                         if key.m() { run!("notify-send" "TODO" "'switch sound to mixer'") }
///                         if key.d() { run!("notify-send" "TODO" "'switch sound to dock'") }
///                         wait!()
///                     })
///                 }
///                 wait!()
///             })
///         }
///         if key.k() {
///             next!(|key, _state| {
///                 if key.esc() { restart!() }
///                 if key.u() { run!(setxkbmap us) }
///                 if key.h() { run!(setxkbmap hu) }
///                 wait!()
///             })
///         }
///         wait!()
///     })
/// }
/// ```
/// to this:
/// ```
/// sequence! {
///     key,
///     (super_l && c) => {
///         (shift && n): "nm-connection-editor";
///         (n): "urxvt -e nmtui";
///         (w): "~/.config/bspwm/toggle_rfkill.sh";
///         (a) => {
///             (a): "pavucontrol";
///             (j): "pw-jack catia";
///             (s) => {
///                 (l): "notify-send TODO 'switch sound to laptop'";
///                 (m): "notify-send TODO 'switch sound to mixer'";
///                 (d): "notify-send TODO 'switch sound to dock'";
///             };
///         };
///         (k) => {
///             (u): "setxkbmap us";
///             (h): "setxkbmap hu";
///         };
///     };
/// }
/// ```
macro_rules! sequence {
    (@parse_body_first $key:ident $($condition:tt $symbol:tt $body:tt);* $(;)?) => {{
        $(crate::tools::sequence!{
            @parse_body_part $key $condition $symbol $body
        })*
    }};
    (@parse_body $key:ident $($condition:tt $symbol:tt $body:tt);* $(;)?) => {{
        if $key.esc() { restart!() }
        $(crate::tools::sequence!{
            @parse_body_part $key $condition $symbol $body
        })*
        wait!()
    }};
    (@parse_condition $key:ident (($token:ident))) => {
        sequence!(@parse_condition $key ($token))
    };
    (@parse_condition $key:ident ($token:ident)) => {
        $key.$token()
    };
    (@parse_condition $key:ident $token:ident) => {
        $key.$token()
    };
    (@parse_condition $key:ident ($a:tt && $($b:tt)*)) => {
        ($key.$a() && crate::tools::sequence!(@parse_condition $key ( $($b)*) ))
    };
    (@parse_body_part $key:ident $condition:tt => { $($rest:tt)* }) => {
        if crate::tools::sequence!(@parse_condition $key $condition) {
            crate::state_machine::next!(|key, _state| {
                crate::tools::sequence!(@parse_body key $($rest)*)
            });
        }
    };
    (@parse_body_part $key:ident $condition:tt : $body:literal) => {
        if crate::tools::sequence!(@parse_condition $key $condition) {
            crate::state_machine::run!($body);
        }
    };
    ($key:ident, $($rest:tt)*) => {
        crate::tools::sequence!(@parse_body_first $key $($rest)*)
    }
}

pub(crate) use sequence;