extern crate stdweb;
#[macro_use]
extern crate yew;

use std::fmt;
use yew::prelude::*;

#[derive(Clone, Copy, PartialEq)]
enum Mark {
    None,
    X,
    O,
}

enum SquareMsg {
    OnClick,
}

#[derive(Default, Clone, PartialEq)]
struct SquareProps {
    value: Mark,
    onclick: Option<Callback<()>>,
}

struct Square {
    value: Mark,
    onclick: Option<Callback<()>>,
}

enum BoardMsg {
    OnSquareClick(usize),
}

struct Board {
    squares: [Mark; 9],
    x_is_next: bool,
}

pub struct Game {}

impl fmt::Display for Mark {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Mark::*;
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

impl Default for Mark {
    fn default() -> Self {
        Mark::None
    }
}

impl Component<()> for Square {
    type Message = SquareMsg;
    type Properties = SquareProps;

    fn create(props: Self::Properties, _: &mut Env<(), Self>) -> Self {
        Square {
            value: props.value,
            onclick: props.onclick,
        }
    }

    fn update(&mut self, msg: Self::Message, _: &mut Env<(), Self>) -> ShouldRender {
        match msg {
            SquareMsg::OnClick => {
                if let Some(ref onclick) = self.onclick {
                    onclick.emit(());
                }
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties, _: &mut Env<(), Self>) -> ShouldRender {
        self.value = props.value;
        self.onclick = props.onclick;
        true
    }
}

impl Renderable<(), Self> for Square {
    fn view(&self) -> Html<(), Self> {
        html!(
            <button
                class="square",
                onclick=|_| SquareMsg::OnClick,
            >
                { &self.value }
            </button>
        )
    }
}

impl Board {
    fn render_square(&self, i: usize) -> Html<(), Self> {
        html!(
            <Square:
                value={ self.squares[i] },
                onclick=move |_| BoardMsg::OnSquareClick(i),
            />
        )
    }
}

impl Component<()> for Board {
    type Message = BoardMsg;
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<(), Self>) -> Self {
        use Mark::None;
        Board {
            squares: [None, None, None, None, None, None, None, None, None],
            x_is_next: true,
        }
    }

    fn update(&mut self, msg: Self::Message, _: &mut Env<(), Self>) -> ShouldRender {
        match msg {
            BoardMsg::OnSquareClick(i) => {
                self.squares[i] = if self.x_is_next { Mark::X } else { Mark::O };
                self.x_is_next = !self.x_is_next;
                true
            }
        }
    }
}

impl Renderable<(), Self> for Board {
    fn view(&self) -> Html<(), Self> {
        let status = format!("Next player: {}", if self.x_is_next { "X" } else { "O" });

        html!(
            <div>
                <div class="status",>{ status }</div>
                <div class="board-row",>
                    { self.render_square(0) }
                    { self.render_square(1) }
                    { self.render_square(2) }
                </div>
                <div class="board-row",>
                    { self.render_square(3) }
                    { self.render_square(4) }
                    { self.render_square(5) }
                </div>
                <div class="board-row",>
                    { self.render_square(6) }
                    { self.render_square(7) }
                    { self.render_square(8) }
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
