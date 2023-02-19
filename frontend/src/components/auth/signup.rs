use reqwasm::http::Request;
use serde::Serialize;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{
    console::{self, log},
    EventTarget, HtmlInputElement,
};
use yew::{function_component, html, use_state, Callback, Event, Html};

#[derive(Serialize, Debug)]
struct user_body {
    email: String,
    pass: String,
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

    let handle_singup = {
        move |_| {
            let post_user_url = "http://localhost:3000/api/v1/";
            let body_params = user_body {
                email: "123@test.com".to_string(),
                pass: "3214".to_string(),
            };
            let o = serde_json::to_string_pretty(&body_params).unwrap();

            wasm_bindgen_futures::spawn_local(async move {
                let data = Request::post(post_user_url)
                    .header("Content-Type", "application/json")
                    .body(JsValue::from(o))
                    .send()
                    .await;

                let object = JsValue::from(data.unwrap().text().await.unwrap());
                console::log_1(&object);
            });
        }
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
        <button onclick={handle_singup}>{"Sign Up"}</button>
      </div>
    }
}
