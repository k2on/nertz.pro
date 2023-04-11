use yew::prelude::*;
use yew::{html, Callback, Component, Html, Properties, ShouldRender};

pub struct NumberList<'a> {
    numbers: Vec<u32>,
    link: &'a Scope<Self>,
}

enum Msg {
    EditNext(usize),
}

impl Component for NumberList {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            numbers: vec![1, 2, 3, 4, 5],
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::EditNext(index) => {
                if index + 1 < self.numbers.len() {
                    self.link.send_message(Msg::EditNext(index + 1));
                }
            }
        }
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                { for self.numbers.iter().enumerate().map(|(i, &n)| html! { <NumberItem index={i} number={n} on_edit_next={self.link.callback(Msg::EditNext)} /> }) }
            </>
        }
    }
}

struct NumberItem {
    index: usize,
    number: u32,
    link: ComponentLink<Self>,
    on_edit_next: Callback<usize>,
    editing: bool,
}

enum ItemMsg {
    StartEditing,
    DoneEditing,
}

impl Component for NumberItem {
    type Message = ItemMsg;
    type Properties = (usize, u32, Callback<usize>);

    fn create((index, number, on_edit_next): Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            index,
            number,
            link,
            on_edit_next,
            editing: index == 0, // Edit the first number by default
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            ItemMsg::StartEditing => {
                self.editing = true;
            }
            ItemMsg::DoneEditing => {
                self.editing = false;
                self.on_edit_next.emit(self.index);
            }
        }
        true
    }

    fn view(&self) -> Html {
        if self.editing {
            html! {
                <input type="number" value={self.number} onblur=self.link.callback(|_| ItemMsg::DoneEditing) onkeypress=self.link.callback(|e: KeyboardEvent| {
                    if e.key() == "Enter" {
                        e.prevent_default();
                        ItemMsg::DoneEditing
                    } else {
                        ItemMsg::StartEditing
                    }
                }) autofocus=true />
            }
        } else {
            html! {
                <span onclick=self.link.callback(|_| ItemMsg::StartEditing)>{self.number}</span>
            }
        }
    }
}
