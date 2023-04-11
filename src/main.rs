use std::{borrow::BorrowMut, collections::HashMap, hash::Hash, vec};

use gloo::{
    console,
    storage::{LocalStorage, Storage},
};
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use web_sys::HtmlInputElement;
use yew::{html::Scope, prelude::*};

#[derive(Debug, Serialize, Deserialize)]
struct State {
    players: Vec<Player>,
    scores: Vec<Vec<Score>>,
    is_in_progress: bool,
    first_to: u8,
}

type Player = String;
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Score {
    val: Option<i8>,
    is_editing: bool,
}

pub struct App {
    state: State,
    refs: HashMap<String, NodeRef>,
}
impl State {
    pub fn new() -> Self {
        Self {
            players: Vec::new(),
            scores: Vec::new(),
            is_in_progress: false,
            first_to: 100, // the game ends when a player hits this number
        }
    }

    fn next_round(&mut self) {
        let mut round: Vec<Score> = std::iter::repeat(Score {
            val: None,
            is_editing: false,
        })
        .take(self.players.len())
        .collect();
        round[0].is_editing = true;
        self.scores.insert(0, round);
    }
}

const KEY: &str = "yew.nertzpro.self";

pub enum AppMsg {
    ScoreEnter(usize, usize, i8),
    ScoreEdit(usize, usize),
    GameNew,
    GameStart,
    PlayerAdd(String),
    PlayerRemove(usize),
}

impl App {
    pub fn get_focused(&self) -> String {
        let (round, player) = self
            .state
            .scores
            .iter()
            .enumerate()
            .find_map(|(round_idx, round)| {
                round
                    .iter()
                    .enumerate()
                    .position(|(_, score)| score.is_editing)
                    .map(|player_idx| (round_idx, player_idx))
            })
            .unwrap_or((0, 0)); // You can use any default value here, (-1, -1) is just an example

        format!("{}_{}", round, player)
    }

    fn next_round(&mut self) {
        self.state.next_round();
        for (round_idx, round) in self.state.scores.iter().enumerate() {
            for (player_idx, _) in round.iter().enumerate() {
                let key = format!("{}_{}", round_idx, player_idx);
                if let None = self.refs.get(&key) {
                    self.refs.insert(key, NodeRef::default());
                }
            }
        }
    }

    fn view_input(&self, link: &Scope<Self>) -> Html {
        let onkeypress = link.batch_callback(|e: KeyboardEvent| {
            if e.key() == "Enter" {
                let input: HtmlInputElement = e.target_unchecked_into();
                let value = input.value();
                input.set_value("");
                Some(AppMsg::PlayerAdd(value))
            } else {
                None
            }
        });
        html! {
            <input
                class="new-player"
                placeholder="Player Name"
                {onkeypress}
            />
        }
    }

    fn view_player(&self, idx: usize, player: &Player, link: &Scope<App>) -> Html {
        let onclick = link.callback(move |_| AppMsg::PlayerRemove(idx));
        html! {
            <li>
                <label>{player}</label>
                <button {onclick}>{"x"}</button>
            </li>
        }
    }

    fn get_next_empty(&mut self) -> Option<&mut Score> {
        self.state
            .scores
            .iter_mut()
            .rev()
            .find_map(|round| round.iter_mut().find(|score| score.val.is_none()))
    }
}

fn make_refs(state: &State) -> HashMap<String, NodeRef> {
    let mut refs = HashMap::new();
    for (round_idx, round) in state.scores.iter().enumerate() {
        for (player_idx, _) in round.iter().enumerate() {
            let key = format!("{}_{}", round_idx, player_idx);
            refs.insert(key, NodeRef::default());
        }
    }
    refs
}

impl Component for App {
    type Message = AppMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let mut state = LocalStorage::get(KEY).unwrap_or_else(|_| State::new());

        let refs = make_refs(&state);

        Self { state, refs }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if self.state.is_in_progress {
            let node_ref = self.refs.get(&self.get_focused()).unwrap();

            if let Some(input) = node_ref.cast::<HtmlInputElement>() {
                input.focus();
            }
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMsg::ScoreEnter(round, player, score) => {
                self.state.scores[round][player] = Score {
                    val: Some(score),
                    is_editing: false,
                };
                if let Some(score) = self.get_next_empty() {
                    score.is_editing = true;
                } else {
                    self.next_round();
                }
            }
            AppMsg::ScoreEdit(round_idx_edit, player_idx_edit) => self
                .state
                .scores
                .iter_mut()
                .enumerate()
                .for_each(|(round_idx, round)| {
                    round
                        .iter_mut()
                        .enumerate()
                        .for_each(|(player_idx, score)| {
                            let should_edit =
                                round_idx == round_idx_edit && player_idx == player_idx_edit;
                            score.is_editing = should_edit;
                        })
                }),
            AppMsg::GameNew => {
                let players = self.state.players.clone();
                self.state = State::new();
                self.state.players = players;
            }
            AppMsg::GameStart => {
                self.state.is_in_progress = true;
                self.next_round();
            }
            AppMsg::PlayerAdd(name) => self.state.players.push(name),
            AppMsg::PlayerRemove(idx) => {
                self.state.players.remove(idx);
            }
        }
        LocalStorage::set(KEY, &self.state).expect("failed to set");
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        console::log!(to_string(&format!("{:?}", self.state)).unwrap());

        html! {
            <div>
                <img id="logo" src="static/logo.png" alt="NERTS.PRO"/>
                {if self.state.is_in_progress {
                    html! {
                        <div>
                <table class="scores">

                <tr>
                    { for self.state.players.iter().map(|player| html! { <td>{player.clone().chars().nth(0).unwrap().to_uppercase()}</td> }) }
                </tr>

                { for self.state.scores.iter().enumerate().map(|(round_idx, round)| html! {
                    <tr>
                    { for round.iter().enumerate().map(|(player_idx, score)| {

                        let key = format!("{}_{}", round_idx, player_idx);
                        let node_ref = self.refs.get(&key).unwrap();

                        let onkeypress = ctx.link().batch_callback(move |e: KeyboardEvent| {
                            if e.key() == "Enter" {
                                let input: HtmlInputElement = e.target_unchecked_into();
                                let val = input.value().parse::<i8>().unwrap();
                                Some(AppMsg::ScoreEnter(round_idx, player_idx, val))
                            } else {
                                None
                            }
                        });

                        let onclick = ctx.link().callback(move |_| {
                            AppMsg::ScoreEdit(round_idx, player_idx)
                        });

                        html! {
                            <td {onclick}>
                            {if score.is_editing {
                                html! {
                                    <input ref={node_ref} {onkeypress} value={if let Some(s) = score.val { s.to_string() } else { String::new() }} type="number"/>
                                }
                            } else {
                                html! {
                                    {if let Some(s) = score.val {
                                        let mut class = Classes::from("score");
                                        if s < 0 {
                                            class.push("red");
                                        }

                                        html! {
                                            <span {class}>{s.to_string()}</span>
                                        }
                                    } else {
                                        html! {
                                            {"--"}
                                        }
                                    }}
                                }
                            }}
                            </td>
                        }
                    })}
                    </tr>
                }) }

                </table>
                <div class="button">
                    <button onclick={ctx.link().callback(move |_| AppMsg::GameNew)}>{"NEW GAME"}</button>
                </div>
                </div>


                    }

                } else {
                    html! {
                        <div>

                            <ul>
                                { for self.state.players.iter().enumerate().map(|(idx, player)| self.view_player(idx, player, ctx.link()))}
                            </ul>
                            {self.view_input(ctx.link())}


                <div class="button">
                    <button onclick={ctx.link().callback(move |_| AppMsg::GameStart)}>{"START GAME"}</button>
                </div>
                        </div>
                    }

                }}
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
