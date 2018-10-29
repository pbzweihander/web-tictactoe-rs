extern crate web_tictactoe;
extern crate yew;

use web_tictactoe::*;
use yew::prelude::*;
use yew::services::console::ConsoleService;

struct Context {
    console: ConsoleService,
}

impl Printer for Context {
    fn print(&mut self, data: &str) {
        self.console.log(data);
    }
}

fn main() {
    yew::initialize();
    let context = Context {
        console: ConsoleService::new(),
    };
    App::<Context, Game>::new(context).mount_to_body();
    yew::run_loop();
}
