use leptos::prelude::*;

#[component]
pub fn Layout() -> impl IntoView {
    let (toggle, set_toggle) = signal(false);

    provide_context(set_toggle);

    view! {
        <div class:flex>
            <p>"Toggled? " {toggle}</p>
            <Body />
        </div>
    }
}

#[component]
fn Body() -> impl IntoView {
    view! {
        <main>
            <header>
                <h4 class="red">"Here is <Body header>"</h4>
            </header>
            <div class:flex>
                <SumbitButton />
            </div>
            <div class:flex>
                <h4>"Here is another button"</h4>
                <SumbitButton />
            </div>
        </main>
    }
}

#[component]
fn SumbitButton() -> impl IntoView {
    let setter = use_context::<WriteSignal<bool>>().expect("to have found the setter provided");

    view! { <button on:click=move |_| setter.update(|value| *value = !*value)>"set Toggle"</button> }
}
