use leptos::{ html::Div, logging::log, prelude::*, task::spawn_local};
use leptos_router::{
    components::{A, Form, Outlet, ParentRoute, Route, Router, Routes},
    hooks::{use_params_map, use_query_map},
    path,
};
use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

#[wasm_bindgen]
extern "C" {
    #[derive(Debug)]
    type Player;

    #[wasm_bindgen(method, getter, structural)]
    fn el(this: &Player) -> Option<String>;

    #[wasm_bindgen(method)]
    fn play(this: &Player);

    #[wasm_bindgen(method)]
    fn pause(this: &Player);

    #[wasm_bindgen(method, js_name=fire)]
    fn fire(this: &Player, cb: &Closure<dyn Fn(JsValue) -> JsValue>, data: JsValue) -> JsValue;

    #[wasm_bindgen(method)]
    fn resume(this: &Player);

    #[wasm_bindgen(method, js_name=getSrcUrl)]
    fn get_src_url(this: &Player, file: &str) -> String;

    #[wasm_bindgen(js_namespace=flvjs, js_name = createPlayer)]
    fn create_player() -> Player;

    #[wasm_bindgen(method, js_name=attachElement)]
    fn attach_element(this: &Player, el: JsValue) -> bool;

    #[wasm_bindgen(method, js_name=destroy)]
    async fn destroy(this: &Player) -> JsValue;
}

#[component]
pub fn Nav() -> impl IntoView {
    let pel = NodeRef::<Div>::new();

    Effect::new(move || {
        if let Some(node) = pel.get() {
            let player = create_player();
            log!("on player mount：{:?}", player.el());
            player.attach_element(node.into());
            player.play();
            player.pause();
            player.resume();
            log!("rust call js and return <{}>", player.get_src_url("funny_video.mp4"));

            let data = player.fire(&Closure::new(|params: JsValue| {
                log!("Js Call rust Defined callback with <{:?}>", params);
                params
            }), JsValue::from_str("hello world"));

            log!("Rust receive Js call defined in rust code <{:?}>", data);

            spawn_local(async move {
                let a = player.destroy().await;
                log!("player destroyed {:?}", a);
            });
            
            log!("player destroyed after spawn_local");
        }
    });

    view! {
        <Router>
            <nav class="flex hor" class:demo>
                <a href="/">"home"</a>
                <a href="/users">"users"</a>
                <a href="/form">"form"</a>
            </nav>
            <main>
                <div node_ref=pel id="player_container" />
                <Routes fallback=|| "Not found.">
                    <ParentRoute path=path!("/users") view=Users>
                        <Route path=path!("") view=EmailAndPhone />
                        <ParentRoute path=path!(":id") view=UserProfile>
                            <Route path=path!("address") view=Address />
                            <Route path=path!("messages") view=Messages />
                            <Route path=path!("email") view=Email />
                            <Route path=path!("") view=|| () />
                        </ParentRoute>
                    </ParentRoute>
                    <Route path=path!("form") view=FormDemo />
                </Routes>
            </main>
        </Router>
    }
}

async fn fetch_search(key: String) -> Vec<Pet> {
    log!("fetch_search: {:?}", key);
    vec![
       Pet { id: 10, name: format!("<{key}> 豆子")},
       Pet { id: 20, name: format!("<{key} ok> 王立")}
    ]
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Pet {
    id: u32,
    name: String,
}

#[component]
fn FormDemo() -> impl IntoView {
    let query = use_query_map();
    let search = move || query.read().get("q").unwrap_or_default();

    let search_results = Resource::new(search, |key| fetch_search(key));
    
    let async_result = move || {
        search_results.get().map_or(Default::default(), |value| value)
    };

    view! {
        <div class:flex>
            <Form method="GET" action="">
                <input type="search" name="q" value=search oninput="this.form.requestSubmit()" />
                <input type="submit" />
            </Form>
            <Show when=move || match query.read().get("q") {
                Some(q) if q.len() > 0 => true,
                _ => false,
            }>
                <Transition fallback=move || ()>
                    <ul>
                        <For
                            each=async_result
                            key=|item| format!("{}_{}", item.id, item.name)
                            let(child)
                        >
                            <li>{child.name}</li>
                        </For>
                    </ul>
                </Transition>
            </Show>
        </div>
    }
}

#[component]
fn Users() -> impl IntoView {
    view! {
        <div class:flex>
            <h1>"Users"</h1>
            <ul class:users>
                <li>
                    <A href="jone/email">"jone"</A>
                </li>
                <li>
                    <A href="lucky">"lucky"</A>
                </li>
                <li>
                    <A href="moth">"moth"</A>
                </li>
                <li>
                    <A href="july">"july"</A>
                </li>
            </ul>
            <Outlet />
        </div>
    }
}

#[component]
fn EmailAndPhone() -> impl IntoView {
    view! { <p>"选个用户"</p> }
}

#[component]
fn Address() -> impl IntoView {
    let query = use_query_map();

    view! { <p>"Address: " {move || query.read().get("debug")}</p> }
}

#[component]
fn Messages() -> impl IntoView {
    view! { <p>"Messages"</p> }
}

#[component]
fn Email() -> impl IntoView {
    let params = use_params_map();
    view! { <p>"Email: " {move || params.read().get("id")}</p> }
}

#[component]
fn UserProfile() -> impl IntoView {
    let params = use_params_map();
    let id = move || params.read().get("id");
    view! {
        <div class:flex>
            <h1>"User id is: " {id}</h1>
            <ul>
                <li>
                    <A href="email">"Email Link"</A>
                </li>
                <li>
                    <A href="address?debug=true">"Address Link"</A>
                </li>
                <li>
                    <A href="messages">"Messages Link"</A>
                </li>
            </ul>
            <Outlet />
        </div>
    }
}
