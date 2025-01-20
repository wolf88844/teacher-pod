#![allow(non_snake_case)]
use components::{modal::PlayBoxInfo, navbar::NavBar};
use dioxus::prelude::*;
use dioxus_free_icons::{icons::fa_solid_icons, Icon};
use dioxus_toast::{ToastFrame, ToastManager};
use mode::is_dark;
mod components;
mod data;
mod hooks;
mod mode;
mod pages;

static DARK_MODE: GlobalSignal<bool> = Global::new(|| {
    let dark = is_dark();
    mode::mode(dark);
    dark
});

static TOAST: GlobalSignal<ToastManager> = Global::new(|| ToastManager::default());

static PLAYER_STATUS: GlobalSignal<PlayBoxInfo> = Global::new(|| PlayBoxInfo {
    display: true,
    pause: true,
    current: usize::MAX,
    playlist: None,
});

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
    log::info!("TeacherPod Init");
    dioxus::launch(app);
}

#[derive(Routable, Clone)]
enum Route {
    #[route("/")]
    Discover {},

    #[route("/login")]
    Login {},
    #[route("/register")]
    Register {},

    #[route("/user/:userid")]
    User { userid: String },

    #[route("/content/:id")]
    Content { id: String },

    #[route("/search/:query")]
    SearchResultList { query: String },

    #[route("/..route")]
    _404 {},
}

#[component]
fn Discover() -> Element {
    rsx!(pages::discover::Discover {})
}

#[component]
fn Login() -> Element {
    rsx!(pages::login::Login {})
}

#[component]
fn Register() -> Element {
    rsx!(pages::login::Register {})
}

#[component]
fn User(userid: String) -> Element {
    rsx!(pages::user::User { userid })
}

#[component]
fn Content(id: String) -> Element {
    rsx!(pages::content::Content { id })
}

#[component]
fn SearchResultList(query: String) -> Element {
    rsx!(pages::search::SearchResult { query })
}
#[component]
fn _404() -> Element {
    rsx!(pages::error::_404 {})
}

fn app() -> Element {
    let toast = TOAST.signal();
    rsx!(
        ToastFrame{
            manager:toast,
        }
        //NavBar{None},
        Footer{},
        Outlet::<Route>{}
    )
}

fn Footer() -> Element {
    rsx!(
        br{}
        div{
            class:"bg-gray-100 dark:bg-gray-600 pt-2",
            div{
                class:"flex pb-5 px-3 m-auto pt-5 border-t text-gray-800 text-sm
                flex-col md:flex-row max-w-6xl",
                div{
                    class:"mt-2 text-black dark:text-white",
                    "© Copyright 2025 All rights reserved."
                }
                div{
                    class:"md:flex-auto md:flex-row-reverse mt-2 flex-row flex",
                    a{
                        class:"w-6 mx-1 text-black dark:text-white",
                        href:"",
                        Icon{
                            icon: fa_solid_icons::FaAddressBook,
                        }
                    }
                }
            }

        }
    )
}
