use dioxus::prelude::*;
use dioxus_free_icons::{icons::fa_solid_icons, Icon};

use crate::Route;
#[component]
pub fn SearchBox(query:String)->Element{

    let route:Route = use_route();
    let path = route.to_string();
    let mut text = use_signal(||{
        if path =="/search"{
            if !query.is_empty(){
                return query;
            }
        }
        String::new()
    });

    rsx!(
        div{
            class:"mt-1 relative rounded-md shadow-sm",
            div{
                class:"absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none",
                span{
                    class:"text-gray-500 sm:text-sm dark:text-white",
                    Icon{
                        icon:fa_solid_icons::FaMagnifyingGlass,
                        width:15,
                        height:15,
                    }
                }
            }
            input{
                class:"focus:ring-indigo-500 focus:border-indigo-500 focus:border-indigo-500 block w-full pl-7 pr-12 sm:text-sm border-gray-300 rounded-md dark:bg-gray-600 dark:text-white",
                r#type:"text",
                value:"{text}",
                oninput: move |evt| text.set(evt.value()),
                onkeydown: move|evt:Event<KeyboardData>|{
                    if evt.code()==Code::Enter && !text.read().is_empty(){
                        Route::SearchBox{ query: text.read().clone() };
                    }
                }
            }
        }
    )
}