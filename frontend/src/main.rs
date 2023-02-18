use yew::prelude::*;
mod components;

use crate::components::home::index::Home;

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);

    html! {
        <>
        <Home counter={counter} />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
