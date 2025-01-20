use dioxus::prelude::*;
use dioxus_free_icons::{icons::fa_solid_icons, Icon};
use models::account::Account;

use crate::{
    components::card::Card,
    data::{account::current_user, request},
};

use super::error::_404;

#[component]
pub fn User(userid: String) -> Element {
    let current_user_page = use_signal(|| false);

    let user_info = use_resource(move || {
        let mut current_user_page = current_user_page.clone();
        let userid = userid.clone();
        async move {
            if let Some(u) = current_user().await {
                current_user_page.set(true);
                return Some(u);
            }
            let resp = request::get(&format!("/user/{}", userid))
                .await
                .send()
                .await
                .ok()?;
            resp.json::<Account>().await.ok()
        }
    });
    let data = user_info.read();
    match data.as_ref() {
        Some(Some(user)) => {
            rsx!(
                div{
                    class:"container mx-auto",
                    Card{
                        div{
                            class:"h-56",
                            div{
                                class:"flex flex-col justify-center items-center gap-3",
                                img{
                                    class:"h-28 w-28 object-cover rounded-full",
                                    src:"{user.avatar}",
                                }
                                h1{
                                    class:"text-2xl font-semibold dark:text-white",
                                    "{user.username}",
                                }
                                if *current_user_page.read(){
                                    div{
                                        class:"inline-block px-6 py-2 border-2 border-blue-600 text-blue-600
                                        font-medium text-xs leading-tight uppercase rounded 
                                        hover:bg-black hover:bg-opacity-5 focus:outline-none focus:ring-0 
                                        transition duration-150 ease-in-out",
                                        "edit info"
                                    }
                                }else{
                                    div{
                                        class:"inline-block px-6 py-2 border-2 border-blue-600 text-blue-600
                                        font-medium text-xs leading-tight uppercase rounded 
                                        hover:bg-black hover:bg-opacity-5 focus:outline-none focus:ring-0 
                                        transition duration-150 ease-in-out",
                                        "follow"
                                    }
                                }
                            }
                        }
                    }
                    div{
                        div{
                            class:"grid sm:grid-cols-4 gap-4 dark:text-white",
                            div{
                                class:"col-span-3",
                                ol{
                                    class:"border-l-2 border-purple-600",
                                    li{
                                        div{
                                            class:"md:flex flex-start",
                                            div{
                                                class:"bg-purple-600 w-6 h-6 flex items-center justify-center rounded-full -ml-3 text-white",
                                                Icon{
                                                    icon:fa_solid_icons::FaStar
                                                }
                                            }
                                            div{
                                                class:"block p-6 rounded-lg shadow-lg bg-gray-100 dark:bg-gray-900 ml-6 mb-10 w-full",
                                                div{
                                                    class:"flex justify-between mb-4",
                                                    a{
                                                        class:"font-medium text-purle-600 hover:text-purple-700 focus:text-purple-800 duration-300 transition ease-in-out text-sm",
                                                        href:"#",
                                                        "New We Design"
                                                    }
                                                    a{
                                                        class:"font-medium text-purle-600 hover:text-purple-700 focus:text-purple-800 duration-300 transition ease-in-out text-sm",
                                                        href:"#",
                                                        "12/12/2022"
                                                    }
                                                }
                                                p{
                                                    class:"text-gray-700 dark:text-white mb-6",
                                                    "this a test content"
                                                }
                                                button{
                                                    class:"inline-block px-6 py-2 border-2 border-purple-600 text-purple-600
                                                    font-medium text-xs leading-tight uppercase rounded
                                                    hover:bg-black hover:bg-opacity-5 focus:outline-none focus:ring-0
                                                    transition duration-150 ease-in-out",
                                                    "data-mdb-ripple":"true",
                                                    r#type:"button",
                                                    "Preview"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            div{
                                Card {
                                    h1{
                                        class:"text-lg font-semibold dark:text-white",
                                        "#{user.id} - {user.username}"
                                    }
                                    div{
                                        class:"flex justify-center",
                                        ul{
                                            class:"bg-white dark:bg-gray-800 rounded-lg w-full",
                                            li{
                                                class:"px-2 py-2 border-b border-gray-200 w-full rounded-t-lg",
                                                span{
                                                    class:"font-semibold",
                                                    "Gender: "
                                                }
                                                "{user.gender}"
                                            }
                                            li{
                                                class:"px-2 py-2 border-b border-gray-200 w-full",
                                                span{
                                                    class:"font-semibold",
                                                    "Email: "
                                                }
                                                "{user.email}"
                                            }
                                            li{
                                                class:"px-2 py-2 border-b border-gray-200 w-full",
                                                span{
                                                    class:"font-semibold",
                                                    "Reg-Date: "
                                                }
                                                "{user.reg_date}"
                                            }
                                            li{
                                                class:"px-2 py-2 border-b border-gray-200 w-full",
                                                span{
                                                    class:"font-semibold",
                                                    "Bio: "
                                                }
                                                "{user.introduction}"
                                            }
                                        }
                                    }
                                 }
                            }
                        }
                    }
                }
            )
        }
        _ => _404(),
    }
}
