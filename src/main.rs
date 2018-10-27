extern crate web_tictactoe;
extern crate yew;

use web_tictactoe::*;
use yew::prelude::*;

fn main() {
    yew::initialize();
    App::<(), Game>::new(()).mount_to_body();
    yew::run_loop();
}
