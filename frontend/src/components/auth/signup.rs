use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::{function_component, html, use_state, Callback, Event, Html};

enum Msg {
    InputValue(String),
}

#[function_component]
pub fn SignUp() -> Html {
    let input_value_handle = use_state(String::default);
    let input_value = (*input_value_handle).clone();

    let handle_email = {
        let input_value_handle = input_value_handle.clone();

        Callback::from(move |e: Event| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

            if let Some(input) = input {
                input_value_handle.set(input.value());
            }
        })
    };

    let handle_password = {
        let input_value_handle = input_value_handle.clone();

        Callback::from(move |e: Event| {
            let target: Option<EventTarget> = e.target();
            let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

            if let Some(input) = input {
                input_value_handle.set(input.value());
            }
        })
    };

    html! {
      <div>
        <label for="Email">
        { "Email: " }
        <input onchange={handle_email}
            id="Email"
            type="email"
            value={input_value.clone()}
         />
        </label>
        <label for="password">
        { "Password:" }
          <input
            id="dangerous-input"
            type="password"
            value={input_value}
            onchange={handle_password}
          />
        </label>
        <button>{"Sign Up"}</button>
      </div>
    }
}
