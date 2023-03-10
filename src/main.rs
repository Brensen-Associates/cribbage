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
            <button
                class="border-2 border-black rounded-md py-1 px-2"
                {onclick}
            >
                {"Add +1"}
            </button>
            <p>
                <b class="text-red-700">{ "Counter: " }</b>
                { *counter }
            </p>
        </div>
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <div class="p-6">
            <h1>{ "Click the button!" }</h1>
            <GabeState />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
