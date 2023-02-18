use yew::{function_component, html, Html, Properties, UseStateHandle};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub counter: UseStateHandle<i32>,
}

#[function_component]
pub fn Home(props: &Props) -> Html {
    let counter = &props.counter;

    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    let minus_click = {
        let counter = counter.clone();
        move |_| {
            let value = *counter - 1;
            counter.set(value);
        }
    };

    html! {
        <div>
            <button {onclick}>{ "+1" }</button>
            <button onclick={minus_click}>{"-1"}</button>
            <p>{  *counter.clone() }</p>

        </div>
    }
}
