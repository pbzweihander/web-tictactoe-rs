extern crate stdweb;
#[macro_use]
extern crate yew;

use yew::prelude::*;

struct Square {
    value: usize,
}

#[derive(Default, Clone, PartialEq)]
struct SquareProps {
    value: usize,
}

struct Board {}

pub struct Game {}

impl Component<()> for Square {
    type Message = ();
    type Properties = SquareProps;

    fn create(props: Self::Properties, _: &mut Env<(), Self>) -> Self {
        Square { value: props.value }
    }

    fn update(&mut self, _: Self::Message, _: &mut Env<(), Self>) -> ShouldRender {
        false
    }
}

impl Renderable<(), Self> for Square {
    fn view(&self) -> Html<(), Self> {
        html!(
            <button class="square",>
                { self.value }
            </button>
        )
    }
}

impl Board {
    fn render_square(i: usize) -> Html<(), Self> {
        html!(
            <Square: value={i}, />
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
