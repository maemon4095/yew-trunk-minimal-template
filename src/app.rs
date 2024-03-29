use yew::prelude::*;
#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <img class="logo" src="https://yew.rs/img/logo.png" alt="Yew logo" />
            <h1>{ "Hello Yew with Trunk" }</h1>
        </main>
    }
}
