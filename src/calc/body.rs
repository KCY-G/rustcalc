use crate::keys::Button::{self, *};
use crate::save::*;
use evalexpr::*;
use yew::prelude::*;

pub struct Body {
    result: Result,
    buttons: Vec<Button>,
    link: ComponentLink<Self>,
}

pub enum Msg {
    Input(char),
    Calculate,
    Initialize,
}

impl Component for Body {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let ott: Vec<u8> = vec![1, 2, 3];
        let mut buttons: Vec<Button> = vec![];
        buttons.extend(ott.iter().map(|n| Number(n + 6)));
        buttons.push(Op('/'));
        buttons.extend(ott.iter().map(|n| Number(n + 3)));
        buttons.push(Op('*'));
        buttons.extend(ott.iter().map(|n| Number(n + 0)));
        buttons.push(Op('+'));
        buttons.push(Number(0));
        buttons.push(Op('.'));
        buttons.push(Op('='));
        buttons.push(Op('-'));

        Self {
            result: Result {
                expr: String::new(),
            },
            buttons,
            link,
        }
    }
    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            Msg::Input(c)  => {
                self.result.expr.push(c);
                true
            }
            Msg::Calculate => {
                if let Ok(v) = eval(&self.result.expr) {
                    self.result.expr = v.to_string();
                    true
                } else {
                    false
                }
            }
            Msg::Initialize => {
                self.result.expr = "".to_string();
                true
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let result: Html = html! {
            <button ondblclick=self.link.callback(move |_| Msg::Initialize) style="height:30px; width:160px">{&self.result.expr}</button>
        };

        let buttons_fst: Vec<Html> = (&self.buttons[..4])
            .iter()
            .map(|b| {
                let c = b.char();
                html! { <button onclick=self.link.callback(move |_| Msg::Input(c)) style="height:30px; width:40px">{b.string()}</button> }
            })
            .collect();

        let buttons_snd: Vec<Html> = (&self.buttons[4..8])
            .iter()
            .map(|b| {
                let c = b.char();
                html! { <button onclick=self.link.callback(move |_| Msg::Input(c)) style="height:30px; width:40px">{b.string()}</button> }
            })
            .collect();

        let buttons_trd: Vec<Html> = (&self.buttons[8..12])
            .iter()
            .map(|b| {
                let c = b.char();
                html! { <button onclick=self.link.callback(move |_| Msg::Input(c)) style="height:30px; width:40px">{b.string()}</button> }
            })
            .collect();

        let buttons_fth: Vec<Html> = (&self.buttons[12..])
            .iter()
            .map(|b| {
                let c = b.char();
                if c.eq(&'=') {
                    html! { <button onclick=self.link.callback(move |_| Msg::Calculate) style="height:30px; width:40px">{b.string()}</button> }
                } else {
                    html! { <button onclick=self.link.callback(move |_| Msg::Input(c)) style="height:30px; width:40px">{b.string()}</button> }
                }
            })
            .collect();

        let buttons: Html = html! {
            <span>
                <div>{buttons_fst}</div>
                <div>{buttons_snd}</div>
                <div>{buttons_trd}</div>
                <div>{buttons_fth}</div>
            </span>
        };

        html! {
            <div>
                <span>{result}</span>
                <span>{buttons}</span>
            </div>
        }
    }
}
