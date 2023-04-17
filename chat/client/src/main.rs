#![allow(unused_imports)]
use std::sync::{Arc, Mutex};

use gloo::timers::callback::Interval;
use gloo_net::http::Request;
use serde::Deserialize;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{console, HtmlInputElement};
use yew::prelude::*;

enum Msg {
    Login(String),
    Send(String),
    Receive(Message),
}

#[derive(Clone, Debug, Deserialize, Eq, Hash)]
struct Message {
    sender: String,
    body: Vec<u8>,
    timestamp: i64,
}

impl PartialEq for Message {
    fn eq(&self, other: &Self) -> bool {
        self.sender == other.sender && self.timestamp == other.timestamp
    }
}

#[derive(Default)]
struct Model {
    user_token: String,
    history: Vec<Message>,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            ..Default::default()
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Login(value) => {
                // let window = web_sys::window().unwrap();
                // let storage = match window.local_storage() {
                //     Ok(Some(local_storage)) => local_storage,
                //     Err(_) => panic!("something went wrong..."),
                //     Ok(None) => panic!("no storage..."),
                // };

                // if let Some(ut) = storage.get_item("user_token").unwrap() {
                //     self.user_token = ut;
                // } else {
                //     storage.set_item("user_token", &value).unwrap();
                self.user_token = value;
                // }

                true
            }
            Msg::Send(value) => {
                let body = value.clone();
                let user_token = self.user_token.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let endpoint = format!("http://0.0.0.0:3000/user/{user_token}/send");
                    Request::put(&endpoint).body(&body).send().await.unwrap();
                });

                true
            }
            Msg::Receive(value) => {
                if !self.history.contains(&value) {
                    console::log_1(&format!("{:#?}", String::from_utf8(value.body.clone())).into());
                    self.history.push(value);
                    self.history.sort_by_key(|s| s.timestamp);
                    return true;
                }

                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let onkeypress = link.batch_callback(|e: KeyboardEvent| {
            if e.key() == "Enter" {
                let input: HtmlInputElement = e.target_unchecked_into();
                let value = input.value();
                input.set_value("");
                Some(Msg::Send(value))
            } else {
                None
            }
        });

        html! {
            <div class="container">
                <p>{ format!("You are logged in as: '{}'", &self.user_token) } </p>
                {
                    self.history.iter().map(|m| {
                        html!{
                            <>
                            <p class="border rounded p-2" style={ if m.sender == self.user_token { "background-color:grey;text-align:right;color:white;" } else { "text-align:left;" }} key={format!("{}-{}", m.timestamp, m.sender)}>{
                                String::from_utf8(m.body.clone()).unwrap()
                            }</p>
                            </>
                        }
                    }).collect::<Html>()
                }
                <input type="text" class="mt-5 fixed-bottom form-control input-lg" placeholder="What do you want to say?" {onkeypress}/>
            </div>
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let first_render_link = ctx.link().clone();
            wasm_bindgen_futures::spawn_local(async move {
                let user_token = Request::get("http://0.0.0.0:3000/login")
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap();
                // console::log_1(&user_token.into());

                first_render_link.send_message(Msg::Login(user_token));
            });
        } else {
            let window = web_sys::window().unwrap();
            let user_token = self.user_token.clone();
            let link = ctx.link().clone();
            // Interval::new(100, move || {
            //     let ut = user_token.clone();
            //     let lk = link.clone();
            //     wasm_bindgen_futures::spawn_local(async move {
            //         let endpoint = format!("http://0.0.0.0:3000/user/{}/receive/latest", ut);
            //         if let Ok(r) = Request::get(&endpoint).send().await {
            //             if let Ok(m) = r.json().await {
            //                 lk.send_message(Msg::Receive(m));
            //             } else {
            //                 console::log_1(&"no new messages".into());
            //             }
            //         } else {
            //             console::log_1(&"no new messages".into());
            //         }
            //     });
            // })
            // .forget();

            let callback = Closure::wrap(Box::new(move || {
                let ut = user_token.clone();
                let lk = link.clone();

                wasm_bindgen_futures::spawn_local(async move {
                    let endpoint = format!("http://0.0.0.0:3000/user/{}/receive/latest", ut);
                    if let Ok(r) = Request::get(&endpoint).send().await {
                        console::log_1(&format!("{:#?}", r).into());
                        if let Ok(m) = r.json().await {
                            lk.send_message(Msg::Receive(m));
                        } else {
                            console::log_1(&"no new messages".into());
                        }
                    } else {
                        console::log_1(&"no new messages".into());
                    }
                });
            }) as Box<dyn Fn()>);

            window
                .set_interval_with_callback_and_timeout_and_arguments_0(
                    callback.as_ref().unchecked_ref(),
                    500,
                )
                .unwrap();

            callback.forget();
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
