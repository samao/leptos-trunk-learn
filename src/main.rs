mod nested;
use nested::Layout;

mod res;
use res::Counter;

mod nav;
use nav::Nav;

use leptos::{html, prelude::*};
use reactive_stores::Store;

#[component]
fn App() -> impl IntoView {
    let (data, set_data) = signal(vec![
        DatabaseEntry {
            key: "foo".to_string(),
            value: 1,
        },
        DatabaseEntry {
            key: "bar".to_string(),
            value: 2,
        },
    ]);
    let (name, set_name) = signal("Controlled".to_owned());
    view! {
        <Nav />
        <Show when=move || true>
            <Counter />
            <button on:click=move |_| {
                set_data
                    .update(|entries| {
                        entries
                            .iter_mut()
                            .for_each(|entry| {
                                entry.value += 1;
                            });
                    });
                leptos::logging::log!("{:?}", data.get());
            }>"update all values"</button>
            <For
                each=move || data.get().into_iter().enumerate()
                key=|(_, entry)| entry.key.clone()
                children=move |(id, _)| {
                    let value = Memo::new(move |_| {
                        data.with(|data| { data.get(id).map(|entry| entry.value).unwrap_or(0) })
                    });

                    view! { <p>{value}</p> }
                }
            />
            <div class:flex>
                <h1>"受控组件"</h1>
                <input
                    type="text"
                    prop:value=name
                    on:input:target=move |e| {
                        set_name.set(e.target().value());
                    }
                />
                <p>{name}</p>
            </div>
            <div class:flex>
                <h1>"受控组件:bind"</h1>
                <Controlled />
            </div>
            <FormUpload />
            <ErrorParseComponent age=10 />
            <Layout />
        </Show>
    }
}

#[component]
fn ErrorParseComponent(age: u8) -> impl IntoView {
    let age = RwSignal::new(Ok(age));

    view! {
        <div class:flex>
            <input
                type="text"
                prop:value=move || age.get().unwrap()
                on:input:target=move |e| {
                    age.set(e.target().value().parse::<u8>());
                }
            />
            <ErrorBoundary fallback=move |errors| {
                view! {
                    <div>
                        <p>"Not a number! Error:"</p>
                        <ul>
                            {move || {
                                errors
                                    .get()
                                    .into_iter()
                                    .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                                    .collect_view()
                            }}
                        </ul>
                    </div>
                }
            }>
                <p>"I Got: "{age}</p>
            </ErrorBoundary>
        </div>
    }
}

#[component]
fn FormUpload() -> impl IntoView {
    let name = RwSignal::new("".to_owned());
    let input_ref: NodeRef<html::Input> = NodeRef::new();
    let msg = RwSignal::new("guud".to_owned());
    let city = RwSignal::new("北京".to_owned());
    view! {
        <div class:flex>
            <h1>"表单上传"</h1>
            <form
                class:flex
                on:submit=move |e| {
                    e.prevent_default();
                    name.update(|name| {
                        *name = input_ref.get().expect("input ref is not set").value();
                    });
                }
            >
                <input type="text" value=name node_ref=input_ref />
                <textarea
                    prop:value=move || msg.get()
                    on:input:target=move |e| {
                        msg.update(|msg| *msg = e.target().value());
                    }
                >
                    {msg.get_untracked()}
                </textarea>
                <div>
                    <select
                        on:change:target=move |e| {
                            city.update(|city| *city = e.target().value());
                            leptos::logging::log!("timeout: {}", city.get());
                        }
                        prop:value=city
                    >
                        {vec!["北京", "上海", "广州"]
                            .into_iter()
                            .map(|city| {
                                view! { <option value=city>{city}</option> }
                            })
                            .collect_view()}
                    </select>
                    <button on:click=move |_| {
                        city.update(|city| *city = "北京".to_owned());
                    }>"reset"</button>
                </div>
                <input type="submit" value="Submit" />
            </form>
            <Show when=move || !name.get().is_empty()>
                <p class:red>{name}</p>
            </Show>

        </div>
    }
}

#[component]
fn Controlled() -> impl IntoView {
    let (name, set_name) = signal("".to_string());
    let email = RwSignal::new("".to_string());
    let spam_me = RwSignal::new(true);

    view! {
        <input type="text" bind:value=(name, set_name) placeholder="please enter your name" />
        <input type="text" bind:value=email placeholder="please enter your email" />
        <label>
            "please spam me lots of spam email." <input type="checkbox" bind:checked=spam_me />
        </label>
        <p>"Name is: " {name}</p>
        <p>"Email is: " {email}</p>
        <Show when=move || spam_me.get()>
            <p>"You'll receive cool bonus content!"</p>
        </Show>
    }
}

#[derive(Debug, Clone, Store)]
struct DatabaseEntry {
    key: String,
    value: i32,
}

fn main() {
    leptos::mount::mount_to_body(App)
}
