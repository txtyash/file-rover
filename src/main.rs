#[allow(dead_code)]
use neovim_lib::{Neovim, NeovimApi, Session};

fn main() {
    let mut event_handler = EventHandler::new();
    event_handler.recv();
}

struct Butcher;

impl Butcher {
    fn murder(&self, args: Vec<String>) -> Vec<String> {
        args
    }
    fn new() -> Butcher {
        Butcher {}
    }
}

struct EventHandler {
    neovim: Neovim,
    butcher: Butcher,
}

impl EventHandler {
    fn new() -> EventHandler {
        let session = Session::new_parent().unwrap();
        let neovim = Neovim::new(session);
        let butcher = Butcher::new();
        EventHandler { neovim, butcher }
    }
    fn recv(&mut self) {
        let receiver = self.neovim.session.start_event_loop_channel();
        for (event, values) in receiver {
            match Messages::from(event) {
                Messages::Murder => {
                    let words = values
                        .iter()
                        .map(|v| v.to_string())
                        .collect::<Vec<String>>();
                    self.neovim // <-- Echo response to Nvim
                        .command(&format!("{:?}", words))
                        .unwrap();
                }
                Messages::Unknown(uevent) => {
                    self.neovim // <-- Echo unknown command
                    .command(&format!("echo \"Unknown command: {}\"", uevent))
                    .unwrap();
                }
            }
        }
    }
}

enum Messages {
    Murder,
    Unknown(String),
}

impl From<String> for Messages {
    fn from(event: String) -> Messages {
        match event.as_str() {
            "murder" => Messages::Murder,
            _ => Messages::Unknown(event),
        }
    }
}
