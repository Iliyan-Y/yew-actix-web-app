use yew::{function_component, html, Html, Properties, UseStateHandle};

use crate::components::auth::signup::SignUp;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub counter: UseStateHandle<i32>,
}

#[function_component]
pub fn Home(props: &Props) -> Html {
    let counter = &props.counter;

    let onclick = {
        let c = counter.clone();
        move |_| {
            let value = *c + 1;
            c.set(value);
        }
    };

    let minus_click = {
        let c = counter.clone();
        move |_| {
            let value = *c - 1;
            c.set(value);
        }
    };

    html! {
        <div>
            <button {onclick}>{ "+1" }</button>
            <button onclick={minus_click}>{"-1"}</button>
            <p>{  *counter.clone()  }</p>
            <SignUp />
        </div>
    }
}
