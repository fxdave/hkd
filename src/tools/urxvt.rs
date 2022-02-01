use std::{env, fmt::Display};

use super::run_in_shell;

#[allow(unused)]
struct Urxvt {
    shell: String,
    path: Option<String>,
    bg: Option<Color>,
}

#[allow(unused)]
impl Urxvt {
    pub fn new() -> Self {
        Urxvt {
            shell: env::var("SHELL").unwrap_or("/bin/sh".into()),
            path: None,
            bg: None,
        }
    }

    pub fn path(mut self, path: &str) -> Self {
        self.path = Some(path.to_string());
        self
    }

    pub fn bg(mut self, color: Color) -> Self {
        self.bg = Some(color);
        self
    }

    pub fn run(self) {
        run_in_shell(&format!(
            "urxvt {background} -e sh -c '{path} {shell}'",
            background = if let Some(color) = self.bg {
                format!(r##"--background-expr 'keep {{ solid "{}" }}' "##, color)
            } else {
                "".into()
            },
            path = if let Some(path) = self.path {
                format!("cd {};", path)
            } else {
                "".into()
            },
            shell = self.shell
        ));
    }
}

#[allow(unused)]
struct Color((u8, u8, u8));

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{}{}{}", self.0 .0, self.0 .1, self.0 .2)
    }
}
