use std::env;
use crate::binding::{Binding, key, run_checked};
use super::binding::run;

#[allow(non_snake_case)]
#[allow(unused)]
pub fn setup_shortcuts() -> Vec<Binding> {
    let _super = &(key(x11::keysym::XK_Super_L) | key(x11::keysym::XK_Super_R));
    let shift = &(key(x11::keysym::XK_Shift_L) | key(x11::keysym::XK_Shift_R));
    let _return = &key(x11::keysym::XK_Return);
    let alt = &(key(x11::keysym::XK_Alt_L) | key(x11::keysym::XK_Alt_R));
    let ctrl = &(key(x11::keysym::XK_Control_L) | key(x11::keysym::XK_Control_R));
    let menu = &key(x11::keysym::XK_Menu);
    let print = &key(x11::keysym::XK_Print);
    let escape = &key(x11::keysym::XK_Escape);
    let space = &key(x11::keysym::XK_space);
    let XF86XK_MonBrightnessUp = &key(x11::keysym::XF86XK_MonBrightnessUp);
    let XF86XK_MonBrightnessDown = &key(x11::keysym::XF86XK_MonBrightnessDown);
    let a = &key(x11::keysym::XK_a);
    let b = &key(x11::keysym::XK_b);
    let c = &key(x11::keysym::XK_c);
    let d = &key(x11::keysym::XK_d);
    let e = &key(x11::keysym::XK_e);
    let f = &key(x11::keysym::XK_f);
    let g = &key(x11::keysym::XK_g);
    let h = &key(x11::keysym::XK_h);
    let i = &key(x11::keysym::XK_i);
    let j = &key(x11::keysym::XK_j);
    let k = &key(x11::keysym::XK_k);
    let l = &key(x11::keysym::XK_l);
    let m = &key(x11::keysym::XK_m);
    let n = &key(x11::keysym::XK_n);
    let o = &key(x11::keysym::XK_o);
    let p = &key(x11::keysym::XK_p);
    let q = &key(x11::keysym::XK_q);
    let r = &key(x11::keysym::XK_r);
    let s = &key(x11::keysym::XK_s);
    let t = &key(x11::keysym::XK_t);
    let u = &key(x11::keysym::XK_u);
    let v = &key(x11::keysym::XK_v);
    let w = &key(x11::keysym::XK_w);
    let x = &key(x11::keysym::XK_x);
    let y = &key(x11::keysym::XK_y);
    let z = &key(x11::keysym::XK_z);
    use super::binding::bind as __;

    vec![
        __(_super + { _return | (shift + _return) | (alt + _return) }) >>
            |i| match i {
                1 => run!("~/.config/bspwm/context-run-urxvt.sh"),
                2 => run!(urxvt),
                3 => run!("~/.config/bspwm/colored-urxvt.sh"),
                _ => unimplemented!(),
            },
        __(ctrl + shift + v) >>
            "copyq menu",
        __(_super + p - {n | o | a | u | d | r}) >>
            |id| {
                run!(id, "~/Projects/manage.fish" {"new", "open", "archive", "unarchive", "delete", "run"})
            },
        __(_super + x - { x | c | (shift+v) | v | (shift+s) | s | (shift+n) | n | (alt+s) | m | f | z | (a - g) | (a - a)}) >> 
            |i| match i {
                1 => run!(sh "-c" "'xcolor | xclip -sel clip'"),
                2 => run!(chromium),
                3 => run!(code),
                4 => run!("context-run" urxvt "code %F"),
                5 => run!(subl),
                6 => run!("context-run" urxvt "subl %F"),
                7 => run!(nautilus),
                8 => run!("context-run" urxvt "nautilus %F"),
                9 => run!(spotify),
                10 => run!(flatpak run "com.mattermost.Desktop"),
                11 => run!(chromium "facebook.com/messages"),
                12 => run!(chromium "youtube.com"),
                13 => run!("pw-jack" guitarix),
                14 => run!("pw-jack" ardour6),
                _ => unimplemented!(),
            },
        __(_super + n) >>
            |_| {
                run!(EDITOR=micro urxvt "-e" nnn);
            },
        __(shift + menu) >>
            |_| {
                let display = env::var("DISPLAY").expect("DISPLAY should be set");
                let hud_is_active = run_checked(&format!("systemctl --user is-active qmenu_hud@{}.service", display)).is_ok();
                if hud_is_active {
                    run_checked(&format!("systemctl --user stop qmenu_hud@{}.service", display)).unwrap();
                } else {
                    run_checked(&format!("systemctl --user stop qmenu_hud@{}.service", display)).unwrap();
                }
            },
        __(!!menu) >>
            "qmenu_hud",
        __(_super + l - {p | l | r}) >>
            |i| {
                run!(i, systemctl {poweroff,suspend,reboot})
            },

        __(_super + c - { n | (shift + n) | (a-a) | (a-j) | (a-s-d) | (a-s-l) | (a-s-m) | w | (k-u) | (k-h) }) >>
            |i| {
                match i {
                    1 => run!(urxvt "-e" nmtui),
                    2 => run!("nm-connection-editor"),
                    3 => run!(pavucontrol),
                    4 => run!("pw-jack" catia),
                    5 => run!("notify-send" "TODO" "'switch sound to dock'"),
                    6 => run!("notify-send" "TODO" "'switch sound to laptop'"),
                    7 => run!("notify-send" "TODO" "'switch sound to mixer'"),
                    8 => run!("~/.config/bspwm/toggle_rfkill.sh"),
                    9 => run!(setxkbmap us),
                    10 => run!(setxkbmap hu),
                    _ => unimplemented!(),
                }
            },

        __(_super + space) >>
            "echo show > /home/dbiro/Projects/All/Programming/vonal/VONALPIPE",
        __(shift + print) >>
            "killall ffcast || ffcast -s byzanz-record --x=%x --y=%y --width=%w --height=%h ~/Pictures/recorded_region.gif",
        __(_super + print) >>
            "if [ $(pgrep xflux) ]; then killall xflux; else xflux -l 47.0963692 -g 17.9094899; fi",
        __(!!print) >>
            "flameshot gui",

        //xf86 backlight
        __(_super + XF86XK_MonBrightnessUp) >>
            "/usr/bin/night inc",
        __(_super + XF86XK_MonBrightnessDown) >>
            "/usr/bin/night dec",
    ]
}
