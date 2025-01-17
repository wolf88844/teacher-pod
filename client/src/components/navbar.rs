use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_solid_icons;
use dioxus_free_icons::Icon;
use crate::{data::account::current_user, mode::{is_dark, mode}, Route, DARK_MODE};

#[component]
pub fn NavBar()->Element{
    let login_button = rsx!(
        div{
            class:"ml-5 relative",
            Link{
                class:"bg-white dark:bg-gray-800 dark:text-white p-2 rounded-md hover:bg-gray-200 dark:bg-gray-900",
                to:"/login",
                "Sign In"
            }
        }
    );

    let current_url = use_route::<Route>().to_string();


    let default_class = "text-black dark:text-white px-3 py-2 rounded-md text-sm font-medium";
    let current_class = "bg-gray-200 dark:bg-gray-900 text-black dark:text-white px-3 py-2 rounded-md text-sm font-medium";

    let default_mobile_class = "text-black dark:text-white block px-3 py-2 rounded-md text-base font-medium";
    let current_mobile_class = "bg-gray-200 dark:bg-gray-900 text-black dark:text-white block px-3 py-2 rounded-md text-base font-medium";

    let user_center = use_resource(|| async move{
        current_user().await
    });


    let user_center = match user_center.read_unchecked().as_ref(){
        Some(Some(user))=>rsx!(
            div{
                class:"ml-3 relative",
                div{
                    Link{
                        class:"bg-white dark:bg-gray-800 flex text-sm rounded-full focus:outline-none foucus:ring-2 foucus:ring-offset-2 focus:ring-offset-gray-800 focus:ring-white",
                        id:"user-menu-button",
                        to:"/user/{user.id}",
                        img{
                            class:"h-8 w-8 rounded-full",
                            alt:"",
                            src:"{user.avatar}",
                        }
                    }
                }
            }
        ),
        _=>login_button,
    };

    let mode_icon = if *DARK_MODE.read(){
        rsx!(Icon{icon: fa_solid_icons::FaSun})
    }else{
        rsx!(Icon{icon: fa_solid_icons::FaMoon})
    };

    rsx!(
        nav{
            class:"bg-white dark:bg-gray-800",
            div{
                class:"max-w-7xl mx-auto px-2 sm:px-6 lg:px-8",
                div{
                    class:"relative flex items-center justify-between h-16",
                    div{
                        class:"absolute inset-y-0 left-0 flex items-center sm:hidden",
                        button{
                            class:"inline-flex items-center justify-center p-2 rounded-md text-gray-400 hover:text-white hover:bg-gray-700 focus:ring-2 foucus:ring-inset focus:ring-white",
                            "aria-controls":"mobile-menu",
                            "aria-expanded":"false",
                            r#type:"button",
                        }
                    }
                    div{
                        class:"flex-1 flex items-center justify-center sm:items-stretch sm:justify-start",
                        div{
                            class:"flex-shrink-0 flex items-center",
                            img{
                                class:"block h-8 w-auto",
                                alt:"WorkFlow",
                                src:"/asserts/image/logo.svg",
                            }
                        }
                        div{
                            class:"hidden sm:block sm:ml-6",
                            div{
                                class:"flex space-x-4",
                                Link{
                                    class: if current_url == "/"{current_class} else {default_class},
                                    to:"/",
                                    "Discover"
                                }
                                Link{
                                    class: if current_url == "/topics"{current_class} else {default_class},
                                    to:"/topics",
                                    "Topics"
                                }
                            }
                        }
                    }
                    div{
                        class:"absolute inset-y-0 right-0 flex items-center pr-2 sm:static sm:inset-auto sm:ml-6 sm:pr-0",
                        div{
                            class:"pr-6 hidden sm:block",
                            crate::components::form::SearchBox{}
                        }
                        button{
                            class:"bg-white dark:bg-gray-800 p-1 rounded-full text-gray-400 hover:text-white hover:bg-black dark:hover:bg-white dark:hover:text-black",
                            r#type:"button",
                            onclick: move|_|{
                                *DARK_MODE.write()=!is_dark();
                                mode(!is_dark());
                            },
                            {mode_icon}
                        }
                        {user_center},
                    }
                }
            }
            div{
                class:"sm:hidden",
                id:"mobile-menu",
                div{
                    class:"px-2 pt-2 pb-3 space-y-1",
                    Link{
                        class: if current_url == "/"{current_mobile_class} else {default_mobile_class},
                        to:"/",
                        "Discover"
                    }
                    Link{
                        class: if current_url == "/topics"{current_mobile_class} else {default_mobile_class},
                        to:"/topics",
                        "Topics"
                    }
                }
            }
        }
        br{}
    )
}