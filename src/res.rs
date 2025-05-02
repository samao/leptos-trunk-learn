use std::time::Duration;

use gloo_timers::future::sleep;
use leptos::{html::Input, prelude::*};

async fn load_data(value: i32, delay: u64) -> i32 {
    sleep(Duration::from_secs(delay)).await;
    value * 20
}

#[component]
pub fn Counter() -> impl IntoView {
    let (count, set_count) = signal(1);
    let (count2, set_count2) = signal(0);
    let a = LocalResource::new(move || load_data(count.get(), 1));
    let b = LocalResource::new(move || load_data(count2.get(), 4));

    view! {
        <div class:flex style:margin-bottom="20px">
            <h1>"Async Data from different api"</h1>
            <button on:click=move |_| {
                *set_count.write() += 1;
                *set_count2.write() += 2;
            }>"Increment"</button>
            <Suspense fallback=move || view! { <p>"Loading..."</p> }>
                <h4>
                    "A delay 1 second: "
                    {move || { a.get().map(|a| view! { <span>{a.take()}</span> }) }}
                </h4>
                <h4>
                    "B delay 4 seconds: "
                    {move || { b.get().map(|b| view! { <span>{b.take()}</span> }) }}
                </h4>
            </Suspense>
        </div>
        <TodoList />
    }
}

// todo action
async fn get_todo(job: String) -> String {
    leptos::logging::log!("get todo action");
    // 线程安全要求必须是send ? 后续复盘解决
    // sleep(Duration::from_secs(1)).await;
    format!("TODO: {:?}", job)
}

#[component]
fn TodoList() -> impl IntoView {
    let get_todo_action = Action::new(|job: &String| {
        let job = job.to_owned();
        get_todo(job)
    });

    let pendding = get_todo_action.pending();
    let todo_work = get_todo_action.value();
    let node_ref = NodeRef::<Input>::new();

    view! {
        <div class:flex>
            <form on:submit=move |ev| {
                ev.prevent_default();
                get_todo_action.dispatch(node_ref.get().expect("input to exist").value());
            }>
                <input type="text" node_ref=node_ref placeholder="Todo" />
                <button type="submit">"request"</button>
            </form>
            <p>
                {move || { pendding.get().then_some("loading...".to_owned()).or(todo_work.get()) }}
            </p>
        </div>
    }
}
