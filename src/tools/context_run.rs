use std::process::{Command};

#[allow(unused)]
pub fn run_in_shell(command: &str) -> Option<String> {
    Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .ok()
        .and_then(|out| String::from_utf8(out.stdout).ok())
}

#[allow(unused)]
pub fn context_run<F: FnOnce(&str), G: FnOnce()>(
    focused_process_name: &str,
    callback_on_focus: F,
    callback_otherwise: G,
) {
    if let Some(process_id) = run_in_shell("xdotool getwindowfocus getwindowpid") {
        if let Some(process_name) = run_in_shell(&format!("ps -p {} -o comm=", process_id)) {
            if process_name == focused_process_name {
                if let Some(window_name) = run_in_shell("xdotool getwindowfocus getwindowname") {
                    callback_on_focus(&window_name)
                }
            } else {
                callback_otherwise()
            }
        }
    }
}
