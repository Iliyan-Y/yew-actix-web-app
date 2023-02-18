use yew::prelude::*;
mod components;

use crate::components::{home::index::Home, nav::top_nav::TopNav};

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);

    html! {
        <>
        <TopNav counter={counter.clone()} />
        <Home counter={counter} />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
