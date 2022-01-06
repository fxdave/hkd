use config::setup_bindings;
use hot_key_daemon::HotKeyDaemonBuilder;
use display::X11Client;

mod bindings;
mod config;
mod display;
mod hot_key_daemon;

fn main() {
    // TODO:
    // handle SIGINT // Ctrl + C
    // handle SIGHUP // controlling terminal closed
    // handle SIGTERM // request to close
    // handle SIGUSR1 // custom
    // handle SIGUSR2 // custom
    // handle SIGALRM // low power

    let mut daemon_builder = HotKeyDaemonBuilder::new(Box::new(X11Client::new()));

    setup_bindings(&mut daemon_builder);

    daemon_builder.build().start();
}
