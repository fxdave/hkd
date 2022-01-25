use std::collections::HashSet;

use display::{DisplayServerClient, Keycode, Modifier};

pub mod display;
//pub mod config;
//pub mod hot_key_daemon;

fn main() {
    let mut x = display::X11Client::new();
    let mut executor = Executor::new(&mut x);
    loop {
        if let Some(event) = x.wait_for_event() {
            println!("e");
            executor.next(event, &mut x);
        }
    }
}

fn grab_keysym(client: &mut dyn DisplayServerClient, keysym: display::Keysym) -> Vec<Keycode> {
    let results = client.grab_keysym_checked(keysym, Modifier::Any as u16);
    results.1.iter().for_each(|err| {
        println!("{:?}", err);
    });
    results.0
}

macro_rules! define_keys {
    ($($key:ident as $name:ident),* $(,)?) => {
        struct KeyState {
            // keycode: a number marking the physical place of keys
            keycodes: HashSet<Keycode>,
            // keysym: a number assigned to the name of the key (understandable by both programs and humans). Like enter.
            // 1 keysym could need multiple keycodes
            // keys:
            $( $name: Vec<Keycode>, )*
        }

        impl KeyState {
            fn new(client: &mut dyn DisplayServerClient) -> Self {
                Self {
                    keycodes: HashSet::new(),
                    $( $name: grab_keysym(client, display::$key),)*
                }
            }
            fn update(&mut self, e: display::DisplayServerEvent) {
                match e {
                    display::DisplayServerEvent::KeyRelease(k) => {self.keycodes.remove(&k);},
                    display::DisplayServerEvent::KeyPress(k) => {self.keycodes.insert(k);},
                    _ => {
                        // do nothing
                    }
                }
            }

            // keys:
            $(
                fn $name(&self)-> bool {
                    self.$name.iter().map(|code| self.keycodes.contains(code)).any(|r| r)
                }
            )*
        }
    };
}

define_keys! {
    XK_Super_L as super_l,
    XK_Escape as esc,
    XK_Shift_L as shift,
    XK_c as c, // nope
    XK_n as n, // nope
    XK_w as w,
    XK_a as a,
    XK_j as j,
    XK_s as s, // nope
    XK_l as l, // nope
    XK_m as m, // nope
    XK_d as d,
    XK_k as k,
    XK_u as u,
    XK_h as h,
    XK_p as p, // nope
    XK_r as r, // nope
}

struct Executor {
    key_state: KeyState,
    actual_node: NodeOut,
}
impl Executor {
    fn new(client: &mut dyn DisplayServerClient) -> Self {
        Self {
            key_state: KeyState::new(client),
            actual_node: NodeOut::Next(start),
        }
    }
    fn next(&mut self, e: display::DisplayServerEvent, client: &mut dyn DisplayServerClient) {
        self.key_state.update(e);
        match self.actual_node {
            NodeOut::Next(fnptr) => {
                if let NodeOut::Next(new_fnptr) = (fnptr)(&self.key_state) {
                    client.release_event(e, display::EventHandling::Hide);
                    self.actual_node = NodeOut::Next(new_fnptr)
                } else {
                    client.release_event(e, display::EventHandling::Replay);
                }
            }
            NodeOut::None => unreachable!(),
        }
    }
}

/**
 * Flow controls
 */

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
        println!("restarting");
        restart!()
    }};
}

/**
 * call tree
 */

enum NodeOut {
    Next(fn(key_state: &KeyState) -> Self),
    // idea: NextMouse(fn(pos: Pos) -> Self),
    None,
}

#[rustfmt::skip]
fn start(key: &KeyState) -> NodeOut {
    // Access common config
    if key.super_l() && key.c() {
        next!(|key| {
            if key.esc() { restart!() }
            if key.shift() && key.n() { run!("nm-connection-editor") }
            if key.n() { run!(urxvt "-e" nmtui) }
            if key.w() { run!("~/.config/bspwm/toggle_rfkill.sh") }
            if key.a() {
                next!(|key| {
                    if key.esc() { restart!() }
                    if key.a() { run!(pavucontrol) }
                    if key.j() { run!("pw-jack" catia) }
                    if key.s() {
                        next!(|key| {
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
                next!(|key| {
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
        next!(|key| {
            if key.esc() { return NodeOut::Next(start) }
            if key.p() { run!(systemctl poweroff) }
            if key.l() { run!(systemctl suspend) }
            if key.r() { run!(systemctl reboot) }
            wait!()
        })
    }

    wait!()
}
