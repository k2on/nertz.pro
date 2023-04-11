use std::{borrow::BorrowMut, collections::HashMap, hash::Hash, vec};

use gloo::{
    console,
    storage::{LocalStorage, Storage},
};
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
struct State {
    players: Vec<String>,
    scores: Vec<Vec<Score>>,
}
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
        self.scores.push(round);
    }
}

const KEY: &str = "yew.nertzpro.self";

pub enum AppMsg {
    ScoreEnter(usize, usize, i8),
}

impl App {
    pub fn get_focused(&self) -> String {
        let (last_round_idx, last_round) = self.state.scores.iter().enumerate().last().unwrap();

        let (player_idx, _) = last_round
            .iter()
            .enumerate()
            .filter(|(_, score)| score.is_editing)
            .nth(0)
            .unwrap();

        format!("{}_{}", last_round_idx, player_idx)
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
}

impl Component for App {
    type Message = AppMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let mut state = LocalStorage::get(KEY).unwrap_or_else(|_| State::new());

        state.players.push("Max".to_owned());
        state.players.push("Bella".to_owned());

        state.scores.push(vec![
            Score {
                val: None,
                is_editing: true,
            },
            Score {
                val: None,
                is_editing: false,
            },
        ]);

        Self {
            state,
            refs: HashMap::from([
                (String::from("0_0"), NodeRef::default()),
                (String::from("0_1"), NodeRef::default()),
            ]),
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        let node_ref = self.refs.get(&self.get_focused()).unwrap();

        if let Some(input) = node_ref.cast::<HtmlInputElement>() {
            input.focus();
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMsg::ScoreEnter(round, player, score) => {
                self.state.scores[round][player] = Score {
                    val: Some(score),
                    is_editing: false,
                };
                if let Some(score) = self.state.scores[round].get_mut(player + 1) {
                    score.is_editing = true;
                } else {
                    self.next_round();
                }
            }
        }
        LocalStorage::set(KEY, &self.state).expect("failed to set");
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <table>

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

                    html! {
                        <td>
                        {if score.is_editing {
                            html! {
                                <input ref={node_ref} {onkeypress} value={if let Some(s) = score.val { s.to_string() } else { String::new() }} type="number"/>
                            }
                        } else {
                            html! {
                                {if let Some(s) = score.val {
                                    html! {
                                        {s.to_string()}
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
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
