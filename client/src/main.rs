#![allow(non_snake_case)]

use components::form::SearchBox;
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
    Home,
    #[route("/search/:query")]
    SearchBox { query: String },
}

#[component]
fn Home() -> Element {
    rsx!()
}

fn app() -> Element {
    let toast = TOAST.signal();
    rsx!(
        ToastFrame{
            manager:toast,
        }
        NavBar{},
        Footer{},
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
