use notify_rust::{Notification, Urgency};
use regex::bytes::Regex;

use super::run_in_shell;
#[allow(unused)]
pub fn get_sink_name(sink_id: usize) -> Option<String> {
    let re = Regex::new(r#"^.*device.description = "(.*)".*$"#).unwrap();
    let sink_name = run_in_shell("pactl list sinks").and_then(|out| {
        out.lines()
            .skip_while(|line| !line.starts_with(&format!("Sink #{}", sink_id)))
            .find(|line| line.contains("device.description = "))
            .and_then(|line| re.captures(line.as_bytes()))
            .and_then(|captures| captures.get(0))
            .and_then(|m| String::from_utf8(m.as_bytes().to_vec()).ok())
    });
    sink_name
}

#[allow(unused)]
pub fn get_sink_inputs() -> Vec<usize> {
    run_in_shell("pactl list sink-inputs short")
        .and_then(|out| get_ids_from_list(out))
        .unwrap_or(vec![])
}

#[allow(unused)]
pub fn get_sinks() -> Vec<usize> {
    run_in_shell("pactl list sinks short")
        .and_then(|out| get_ids_from_list(out))
        .unwrap_or(vec![])
}

fn get_ids_from_list(list: String) -> Option<Vec<usize>> {
    list.lines()
        .map(|line| {
            line.split_whitespace()
                .next()
                .and_then(|id| id.parse::<usize>().ok())
        })
        .collect::<Option<Vec<_>>>()
}

#[allow(unused)]
pub fn set_default_sink(sink_id: usize) {
    run_in_shell(&format!("pactl set-default-sink {}", sink_id));
    for sink_input in get_sink_inputs() {
        run_in_shell(&format!("pactl move-sink-input {} {}", sink_input, sink_id));
    }

    if let Some(sink_name) = get_sink_name(sink_id) {
        Notification::new()
            .body("Default has changed to")
            .appname(&sink_name)
            .urgency(Urgency::Critical)
            .show();
    }
}

#[allow(unused)]
pub fn get_default_sink_name() -> Option<String> {
    run_in_shell("pactl info").and_then(|info| {
        info.lines()
            .filter(|line| line.starts_with("Default Sink: "))
            .map(|line| line.replace("Default Sink: ", ""))
            .next()
    })
}

#[allow(unused)]
pub fn get_sink_id_by_name(name: &str) -> Option<usize> {
    run_in_shell("pactl list sinks short").and_then(|list| {
        list.lines()
            .filter(|line| line.contains(name))
            .map(|line| {
                line.split_whitespace()
                    .next()
                    .and_then(|line| line.parse::<usize>().ok())
            })
            .next()?
    })
}

/// switches to the next audio device
#[allow(unused)]
pub fn cycle_sinks(left: bool) {
    if let Some(default_sink_name) = get_default_sink_name() {
        let default_sink_id = get_sink_id_by_name(&default_sink_name).unwrap();
        let mut sinks = get_sinks();

        sinks.sort();

        if left {
            sinks.reverse()
        }

        let mut next_will_be_the_default = false;
        for sink in &sinks {
            if next_will_be_the_default {
                next_will_be_the_default = false;
                set_default_sink(*sink);
            }

            if *sink == default_sink_id {
                next_will_be_the_default = true
            }
        }

        if next_will_be_the_default {
            set_default_sink(sinks[0])
        }
    }
}
