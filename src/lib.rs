extern crate stdweb;
#[macro_use]
extern crate yew;

use std::fmt;
use yew::prelude::*;

pub trait Printer {
    fn print(&mut self, data: &str);
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Mark {
    None,
    X,
    O,
}

struct Square {
    value: Mark,
    onclick: Option<Callback<()>>,
}

#[derive(Default, Clone, PartialEq)]
struct SquareProps {
    value: Mark,
    onclick: Option<Callback<()>>,
}

enum SquareMsg {
    OnClick,
}

struct Board {
    squares: [Mark; 9],
    onclick: Option<Callback<usize>>,
}

#[derive(Default, Clone, PartialEq)]
struct BoardProps {
    squares: [Mark; 9],
    onclick: Option<Callback<usize>>,
}

enum BoardMsg {
    OnSquareClick(usize),
}

pub struct Game {
    history: Vec<[Mark; 9]>,
    x_is_next: bool,
    step_number: usize,
}

pub enum GameMsg {
    OnSquareClick(usize),
    JumpTo(usize),
}

fn calculate_winner(squares: &[Mark]) -> Mark {
    let lines = [
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],
        [0, 4, 8],
        [2, 4, 6],
    ];
    for line in lines.into_iter() {
        let [a, b, c] = line;
        if squares[*a] == squares[*b] && squares[*a] == squares[*c] {
            return squares[*a];
        }
    }
    Mark::None
}

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

impl<CTX: 'static> Component<CTX> for Square {
    type Message = SquareMsg;
    type Properties = SquareProps;

    fn create(props: Self::Properties, _: &mut Env<CTX, Self>) -> Self {
        Square {
            value: props.value,
            onclick: props.onclick,
        }
    }

    fn update(&mut self, msg: Self::Message, _: &mut Env<CTX, Self>) -> ShouldRender {
        match msg {
            SquareMsg::OnClick => {
                if let Some(ref onclick) = self.onclick {
                    onclick.emit(());
                }
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties, _: &mut Env<CTX, Self>) -> ShouldRender {
        self.value = props.value;
        self.onclick = props.onclick;
        true
    }
}

impl<CTX: 'static> Renderable<CTX, Self> for Square {
    fn view(&self) -> Html<CTX, Self> {
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
    fn render_square<CTX: 'static>(&self, i: usize) -> Html<CTX, Self> {
        html!(
            <Square:
                value={ self.squares[i] },
                onclick=move |_| BoardMsg::OnSquareClick(i),
            />
        )
    }
}

impl<CTX: 'static> Component<CTX> for Board {
    type Message = BoardMsg;
    type Properties = BoardProps;

    fn create(props: Self::Properties, _: &mut Env<CTX, Self>) -> Self {
        Board {
            squares: props.squares,
            onclick: props.onclick,
        }
    }

    fn update(&mut self, msg: Self::Message, _: &mut Env<CTX, Self>) -> ShouldRender {
        match msg {
            BoardMsg::OnSquareClick(i) => {
                if let Some(ref onclick) = self.onclick {
                    onclick.emit(i);
                }
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties, _: &mut Env<CTX, Self>) -> ShouldRender {
        self.squares = props.squares;
        self.onclick = props.onclick;
        true
    }
}

impl<CTX: 'static> Renderable<CTX, Self> for Board {
    fn view(&self) -> Html<CTX, Self> {
        html!(
            <div>
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

impl Game {
    fn render_move<CTX: Printer + 'static>(i: usize) -> Html<CTX, Self> {
        let desc = if i == 0 {
            format!("Go to game start")
        } else {
            format!("Go to move #{}", i)
        };
        html!(
            <li>
                <button onclick=|_| GameMsg::JumpTo(i),>{ desc }</button>
            </li>
        )
    }

    fn jump_to(&mut self, step: usize) {
        self.step_number = step;
        self.x_is_next = (step % 2) == 0;
    }
}

impl<CTX: Printer + 'static> Component<CTX> for Game {
    type Message = GameMsg;
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<CTX, Self>) -> Self {
        use Mark::None;
        Game {
            history: vec![[None, None, None, None, None, None, None, None, None]],
            x_is_next: true,
            step_number: 0,
        }
    }

    fn update(&mut self, msg: Self::Message, _: &mut Env<CTX, Self>) -> ShouldRender {
        match msg {
            GameMsg::OnSquareClick(i) => {
                let mut squares = {
                    self.history.truncate(self.step_number + 1);
                    let current = self.history.last().unwrap();
                    current.clone()
                };
                if calculate_winner(&squares) != Mark::None {
                    return false;
                }
                squares[i] = if self.x_is_next { Mark::X } else { Mark::O };
                self.step_number = self.history.len();
                self.history.push(squares);
                self.x_is_next = !self.x_is_next;
                true
            }
            GameMsg::JumpTo(i) => {
                self.jump_to(i);
                true
            }
        }
    }
}

impl<CTX: Printer + 'static> Renderable<CTX, Self> for Game {
    fn view(&self) -> Html<CTX, Self> {
        let current = self.history[self.step_number];
        let winner = calculate_winner(&current);

        let status = match winner {
            Mark::None => format!("Next player: {}", if self.x_is_next { "X" } else { "O" }),
            m => format!("Winner: {}", m),
        };

        html!(
            <div class="game",>
                <div class="game-board",>
                    <Board:
                        squares={ current },
                        onclick=|i| GameMsg::OnSquareClick(i),
                    />
                </div>
                <div class="game-info",>
                    <div>{ status }</div>
                    <ol>{ for (0..self.history.len()).map(Self::render_move) }</ol>
                </div>
            </div>
        )
    }
}
