extern crate stdweb;
#[macro_use]
extern crate yew;

use std::fmt;
use yew::prelude::*;

enum SquareValue {
    None,
    X,
    O,
}

enum SquareMsg {
    ChangeValue(SquareValue),
}

struct Square {
    value: SquareValue,
}

struct Board {}

pub struct Game {}

impl fmt::Display for SquareValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use SquareValue::*;
        write!(
            f,
            "{}",
            match self {
                None => "",
                X => "X",
                O => "O",
            }
        )
    }
}

impl Component<()> for Square {
    type Message = SquareMsg;
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<(), Self>) -> Self {
        Square {
            value: SquareValue::None,
        }
    }

    fn update(&mut self, msg: Self::Message, _: &mut Env<(), Self>) -> ShouldRender {
        match msg {
            SquareMsg::ChangeValue(v) => {
                self.value = v;
                true
            }
        }
    }
}

impl Renderable<(), Self> for Square {
    fn view(&self) -> Html<(), Self> {
        html!(
            <button
                class="square", onclick=|_| SquareMsg::ChangeValue(SquareValue::X),>
                { &self.value }
            </button>
        )
    }
}

impl Board {
    fn render_square(_: usize) -> Html<(), Self> {
        html!(
            <Square: />
        )
    }
}

impl Component<()> for Board {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<(), Self>) -> Self {
        Board {}
    }

    fn update(&mut self, _: Self::Message, _: &mut Env<(), Self>) -> ShouldRender {
        false
    }
}

impl Renderable<(), Self> for Board {
    fn view(&self) -> Html<(), Self> {
        let status = "Next player: X";

        html!(
            <div>
                <div class="status",>{ status }</div>
                <div class="board-row",>
                    { Self::render_square(0) }
                    { Self::render_square(1) }
                    { Self::render_square(2) }
                </div>
                <div class="board-row",>
                    { Self::render_square(3) }
                    { Self::render_square(4) }
                    { Self::render_square(5) }
                </div>
                <div class="board-row",>
                    { Self::render_square(6) }
                    { Self::render_square(7) }
                    { Self::render_square(8) }
                </div>
            </div>
        )
    }
}

impl Component<()> for Game {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<(), Self>) -> Self {
        Game {}
    }

    fn update(&mut self, _: Self::Message, _: &mut Env<(), Self>) -> ShouldRender {
        false
    }
}

impl Renderable<(), Self> for Game {
    fn view(&self) -> Html<(), Self> {
        html!(
            <div class="game",>
                <div class="game-board",>
                    <Board: />
                </div>
                /* <div class="game-info",>
                    <div>{/* status */}</div>
                    <ol>{/* TODO */}</ol>
                </div> */
            </div>
        )
    }
}
