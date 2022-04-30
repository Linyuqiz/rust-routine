use stylist::yew::styled_component;
use yew::prelude::*;

use components::main_title::MainTitle;

mod components;

#[styled_component(App)]
pub fn app() -> Html {
    html! {
        <div>
            <MainTitle title="Hello World"/>
        </div>
    }
}
