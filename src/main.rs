use crate::{state_machine::{define_keys, next, restart, run, wait, NodeOut}, tools::sequence};
use display::DisplayServerClient;

mod display;
mod state_machine;
mod tools;

define_keys! {
    super_l => display::XK_Super_L,
    esc => display::XK_Escape,
    shift => display::XK_Shift_L,
    c => display::XK_c,
    n => display::XK_n,
    w => display::XK_w,
    a => display::XK_a,
    j => display::XK_j,
    s => display::XK_s,
    l => display::XK_l,
    m => display::XK_m,
    d => display::XK_d,
    k => display::XK_k,
    u => display::XK_u,
    h => display::XK_h,
    p => display::XK_p,
    r => display::XK_r,
    long_i => display::XK_iacute
}

struct UserState { count: i32 }
#[rustfmt::skip]
fn start(key: &KeyState, state: &mut UserState) -> NodeOut<KeyState, UserState> {

    sequence! {
        key,
        (super_l && c) => {
            (shift && n): "nm-connection-editor";
            (n): "urxvt -e nmtui";
            (w): "~/.config/bspwm/toggle_rfkill.sh";
            (a) => {
                (a): "pavucontrol";
                (j): "pw-jack catia";
                (s) => {
                    (l): "notify-send TODO 'switch sound to laptop'";
                    (m): "notify-send TODO 'switch sound to mixer'";
                    (d): "notify-send TODO 'switch sound to dock'";
                };
            };
            (k) => {
                (u): "setxkbmap us";
                (h): "setxkbmap hu";
            };
        };
    }

    if key.long_i() {
        state.count += 1;
        println!("counter is increased: {:?}", state.count)
    }

    // Access common config
    if key.super_l() && key.c() {
        next!(|key, _state| {
            if key.esc() { restart!() }
            if key.shift() && key.n() { run!("nm-connection-editor") }
            if key.n() { run!(urxvt "-e" nmtui) }
            if key.w() { run!("~/.config/bspwm/toggle_rfkill.sh") }
            if key.a() {
                next!(|key, _state| {
                    if key.esc() { restart!() }
                    if key.a() { run!(pavucontrol) }
                    if key.j() { run!("pw-jack" catia) }
                    if key.s() {
                        next!(|key, _state| {
                            if key.esc() { restart!() }
                            if key.l() { run!("notify-send" "TODO" "'switch sound to laptop'") }
                            if key.m() { run!("notify-send" "TODO" "'switch sound to mixer'") }
                            if key.d() { run!("notify-send" "TODO" "'switch sound to dock'") }
                            wait!()
                        })
                    }
                    wait!()
                })
            }
            if key.k() {
                next!(|key, _state| {
                    if key.esc() { restart!() }
                    if key.u() { run!(setxkbmap us) }
                    if key.h() { run!(setxkbmap hu) }
                    wait!()
                })
            }
            wait!()
        })
    }

    // session control
    if key.super_l() && key.l() {
        next!(|key, _state| {
            if key.esc() { return NodeOut::Next(start) }
            if key.p() { run!(systemctl poweroff) }
            if key.l() { run!(systemctl suspend) }
            if key.r() { run!(systemctl reboot) }
            wait!()
        })
    }

    wait!()
}

fn main() {
    let mut x = display::X11Client::new();
    let mut executor = state_machine::Executor::new(&mut x, start, UserState {
        count: 0
    });
    loop {
        if let Some(event) = x.wait_for_event() {
            executor.next(event, &mut x);
        }
    }
}
