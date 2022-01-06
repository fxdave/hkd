use crate::bindings::Binding;
use crate::bindings::AST;
use crate::display::DisplayServerClient;
use crate::display::EventHandling;
use crate::display::Keysym;
use crate::display::Modifier;

pub struct HotKeyDaemon {
    client: Box<dyn DisplayServerClient>,
}

impl HotKeyDaemon {
    pub fn start(&mut self) {
        self.client.subscribe(Box::new(|event| {
            // TODO

            println!("{:?}", event);

            EventHandling::Replay
        }))
    }
}

/// Grab the required resources and builds a daemon that can be started.
/// TODO: maybe grabbing keys on the fly would be better?
pub struct HotKeyDaemonBuilder {
    client: Box<dyn DisplayServerClient>,
    bindings: Vec<Binding>,
}

impl HotKeyDaemonBuilder {
    pub fn new(client: Box<dyn DisplayServerClient>) -> Self {
        Self {
            client,
            bindings: vec![],
        }
    }

    /// Register key
    pub fn key(&mut self, keysym: Keysym) -> AST {
        let modifiers = Modifier::Any;
        // possibly already grabbed
        self.client.grab_keysym(keysym, modifiers as u16);
        AST::Key {
            replay: false,
            key: keysym,
        }
    }

    pub fn setup_bindings(&mut self, bindings: Vec<Binding>) {
        self.bindings = bindings
    }

    // TODO: button, modifier

    pub fn build(mut self) -> HotKeyDaemon {
        self.client.flush();
        HotKeyDaemon {
            client: self.client,
        }
    }
}
