use yew::{function_component, html, Html, Properties, UseStateHandle};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub counter: UseStateHandle<i32>,
}

#[function_component]
pub fn TopNav(props: &Props) -> Html {
    let width = if *props.counter >= 0 {
        *props.counter
    } else {
        5
    };
    let div_style = format!(
        "background: red;
          width: {}em;
          display: flex;
          justify-content: flex-end;
          padding: 1em;
          min-width: 5em;
          margin-bottom: 1em;",
        width
    );

    html! {
        <div style={div_style}>
            <p style="color: blue;">{ *props.counter }</p>
        </div>
    }
}
