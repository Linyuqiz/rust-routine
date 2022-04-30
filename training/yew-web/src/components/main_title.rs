use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: String,
}

#[styled_component(MainTitle)]
pub fn main_title(props: &Props) -> Html {
    html! {
        <div>
            <h1>{&props.title}</h1>
        </div>
    }
}
