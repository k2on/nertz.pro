use gloo::storage::{LocalStorage, Storage};
use state::{Player, PlayerId, Score, State};
use strum::IntoEnumIterator;
use web_sys::HtmlInputElement as InputElement;
use yew::events::{FocusEvent, KeyboardEvent};
use yew::html::Scope;
use yew::{classes, html, Classes, Component, Context, Html, NodeRef, TargetCast};

mod state;

const KEY: &str = "yew.nertzpro.self";

pub enum Msg {
    PlayerAdd(String),
    PlayerRemove(usize),
    GameStart,
    ScoreEnter(PlayerId, Score),
}

pub struct App {
    state: State,
    focus_ref: NodeRef,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let state = LocalStorage::get(KEY).unwrap_or_else(|_| State::new());
        let focus_ref = NodeRef::default();
        Self { state, focus_ref }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::PlayerAdd(name) => {
                if !name.is_empty() {
                    let player = Player {
                        id: self.state.players.len() as u32,
                        name,
                    };
                    self.state.players.push(player);
                }
            }
            Msg::PlayerRemove(idx) => {
                self.state.player_remove(idx);
            }
            Msg::GameStart => {
                self.state.is_game_started = true;
            }
            Msg::ScoreEnter(_, _) => todo!(),
        }
        LocalStorage::set(KEY, &self.state).expect("failed to set");
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="nertzpro">
                <h2>{"NERTZ.PRO"}</h2>
                { if self.state.is_game_started {
                    html! {
                        <table>
                        <tr>
                        { for self.state.players.iter().map(|player| html! { <td>{player.name.clone().chars().nth(0).unwrap().to_uppercase()}</td> }) }
                        </tr>
                        </table>


                    }
                } else {
                    html! {
                        <>
                            <ul class="todo-list">
                                { for self.state.players.iter().enumerate().map(|e| self.view_entry(e, ctx.link())) }
                            </ul>
                            { self.view_input(ctx.link()) }
                            <div>
                                { self.view_start_game_button(ctx.link()) }
                            </div>
                        </>
                    }
                } }
            </div>
        }
    }
}

impl App {
    fn view_input(&self, link: &Scope<Self>) -> Html {
        let onkeypress = link.batch_callback(|e: KeyboardEvent| {
            if e.key() == "Enter" {
                let input: InputElement = e.target_unchecked_into();
                let value = input.value();
                input.set_value("");
                Some(Msg::PlayerAdd(value))
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

    fn view_entry(&self, (idx, player): (usize, &Player), link: &Scope<Self>) -> Html {
        let mut class = Classes::from("todo");
        html! {
            <li {class}>
                <div class="view">
                    <label>{ &player.name }</label>
                    <button class="destroy" onclick={link.callback(move |_| Msg::PlayerRemove(idx))}>{"x"}</button>
                </div>
            </li>
        }
    }

    fn view_start_game_button(&self, link: &Scope<Self>) -> Html {
        html! {
            <button class="start" onclick={link.callback(move |_| Msg::GameStart)}>{"START GAME"}</button>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
