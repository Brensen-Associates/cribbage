use yew::prelude::*;

#[function_component(GabeState)]
fn gabe_state() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        Callback::from(move |_| counter.set(*counter + 1))
    };

    html! {
        <div>
            <button {onclick}>{"Add +1"}</button>
            <p>
                <b>{ "Counter: " }</b>
                { *counter }
            </p>
        </div>
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <h1>{ "Click the button!" }</h1>
            <GabeState />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
