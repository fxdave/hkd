use std::env;
use crate::bindings::{run_checked};
use crate::bindings::run;
use crate::hot_key_daemon::{HotKeyDaemonBuilder, HotKeyDaemon};
use crate::bindings::bind as __;
use crate::display as dpy;

#[allow(non_snake_case)]
#[allow(unused)]
pub fn setup_bindings(ctx: &mut HotKeyDaemonBuilder) {
    let esc = &ctx.key(dpy::XK_Escape);
    let _super = &(ctx.key(dpy::XK_Super_L) | ctx.key(dpy::XK_Super_R));
    let shift = &(ctx.key(dpy::XK_Shift_L) | ctx.key(dpy::XK_Shift_R));
    let _return = &ctx.key(dpy::XK_Return);
    let alt = &(ctx.key(dpy::XK_Alt_L) | ctx.key(dpy::XK_Alt_R));
    let ctrl = &(ctx.key(dpy::XK_Control_L) | ctx.key(dpy::XK_Control_R));
    let menu = &ctx.key(dpy::XK_Menu);
    let print = &ctx.key(dpy::XK_Print);
    let escape = &ctx.key(dpy::XK_Escape);
    let space = &ctx.key(dpy::XK_space);
    let XF86XK_MonBrightnessUp = &ctx.key(dpy::XF86XK_MonBrightnessUp);
    let XF86XK_MonBrightnessDown = &ctx.key(dpy::XF86XK_MonBrightnessDown);
    let a = &ctx.key(dpy::XK_a);
    let b = &ctx.key(dpy::XK_b);
    let c = &ctx.key(dpy::XK_c);
    let d = &ctx.key(dpy::XK_d);
    let e = &ctx.key(dpy::XK_e);
    let f = &ctx.key(dpy::XK_f);
    let g = &ctx.key(dpy::XK_g);
    let h = &ctx.key(dpy::XK_h);
    let i = &ctx.key(dpy::XK_i);
    let j = &ctx.key(dpy::XK_j);
    let k = &ctx.key(dpy::XK_k);
    let l = &ctx.key(dpy::XK_l);
    let m = &ctx.key(dpy::XK_m);
    let n = &ctx.key(dpy::XK_n);
    let o = &ctx.key(dpy::XK_o);
    let p = &ctx.key(dpy::XK_p);
    let q = &ctx.key(dpy::XK_q);
    let r = &ctx.key(dpy::XK_r);
    let s = &ctx.key(dpy::XK_s);
    let t = &ctx.key(dpy::XK_t);
    let u = &ctx.key(dpy::XK_u);
    let v = &ctx.key(dpy::XK_v);
    let w = &ctx.key(dpy::XK_w);
    let x = &ctx.key(dpy::XK_x);
    let y = &ctx.key(dpy::XK_y);
    let z = &ctx.key(dpy::XK_z);

    ctx.setup_bindings(vec![
        __(esc.clone()) >> |_, ctx: &mut HotKeyDaemon| {
            ctx.reset_all_bindings();
        },
        __(_super + { _return | (shift + _return) | (alt + _return) }) >>
            |i, ctx: &mut HotKeyDaemon| match i {
                1 => run!("~/.config/bspwm/context-run-urxvt.sh"),
                2 => run!(urxvt),
                3 => run!("~/.config/bspwm/colored-urxvt.sh"),
                _ => unimplemented!(),
            },
        __(ctrl + shift + v) >>
            "copyq menu",
        __(_super + p - {n | o | a | u | d | r}) >>
            |i, ctx: &mut HotKeyDaemon| {
                run!(i, "~/Projects/manage.fish" {"new", "open", "archive", "unarchive", "delete", "run"})
            },
        __(_super + x - { x | c | (shift+v) | v | (shift+s) | s | (shift+n) | n | (alt+s) | m | f | z | (a - g) | (a - a)}) >> 
            |i, ctx: &mut HotKeyDaemon| match i {
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
            |_, ctx: &mut HotKeyDaemon| {
                run!(EDITOR=micro urxvt "-e" nnn);
            },
        __(shift + menu) >>
            |_, ctx: &mut HotKeyDaemon| {
                let display = env::var("DISPLAY").expect("DISPLAY should be set");
                let hud_is_active = run_checked(&format!("systemctl --user is-active qmenu_hud@{}.service", display)).is_ok();
                if hud_is_active {
                    run_checked(&format!("systemctl --user stop qmenu_hud@{}.service", display)).unwrap();
                } else {
                    run_checked(&format!("systemctl --user stop qmenu_hud@{}.service", display)).unwrap();
                }
            },
        __(menu.clone()) >>
            "qmenu_hud",
        __(_super + l - {p | l | r}) >>
            |i, ctx: &mut HotKeyDaemon| {
                run!(i, systemctl {poweroff,suspend,reboot})
            },

        __(_super + c - { n | (shift + n) | (a-a) | (a-j) | (a-s-d) | (a-s-l) | (a-s-m) | w | (k-u) | (k-h) }) >>
            |i, ctx: &mut HotKeyDaemon| {
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
        __(print.clone()) >>
            "flameshot gui",

        //xf86 backlight
        __(_super + XF86XK_MonBrightnessUp) >>
            "/usr/bin/night inc",
        __(_super + XF86XK_MonBrightnessDown) >>
            "/usr/bin/night dec",
    ])
}
