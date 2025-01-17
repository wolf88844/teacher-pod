use dioxus::prelude::*;
use models::{account::Account, data::SearchInfo};

use crate::PLAYER_STATUS;

#[component]
pub fn SimpleUserList(data:Vec<Account>) ->Element{
    rsx!(
        ul{
            class: "divide-y divide-gray-200 dark:divide-gray-700",
            role:"list",
            for item in data{
                li{
                    class:"py-3 sm:py-4",
                    div{
                        class:"flex items-center space-x-4",
                        div{
                            class:"flex-shrink-0",
                            img{
                                class:"w-8 h-8 rounded-full",
                                alt:"Neil image",
                                src:"{item.avatar}",
                            }
                        }
                        div{
                            class:"flex-1 min-w-0",
                            p{
                                class:"text-sm font-medium text-gray-900 truncate dark:text-white",
                                "{item.username}"
                            }
                            p{
                                class:"text-sm text-gray-500 truncate dark:text-gray-400",
                                "{item.email}",
                            }
                        }
                    }
                }
            }
        }
    )
}

#[component]
pub fn SearchResultList(data:SearchInfo)->Element{
    let mut playbox = PLAYER_STATUS.signal();
    let list_display = data.results.iter().map(|v|{
        let v_clone = v.clone();
        let title = v.title_original.clone();
        let min = (v.audio_length_sec as f32)/60_f32;
        rsx!(
            div{
                class:"flex items-center justify-center",
                a{
                    class:"rounded-xl border p-5 shadow-md w-9/12 bg-white dark:bg-gray-700 hover:bg-gray-100 dark:hover:bg-gray-600",
                    href:"javascript:void(0)",
                    onclick:move|_|{
                        playbox.write().playlist = Some(vec![v_clone.clone()]);
                        playbox.write().current = 0;
                        playbox.write().display = true;
                    },
                    div{
                        class:"flex w-full items-center justify-between border-b pb-3",
                        div{
                            class:"flex items-center space-x-3",
                            img{
                                class:"h-8 w-8",
                                src:"{v.image}"
                            }
                            div{
                                class:"text-lg font-bold text-slate-700 dark:text-salte-100",
                                "{title}"
                            }
                        }
                        div{
                            class:"flex items-center space-x-8",
                            div{
                                class:"text-xs text-neutral-500 dark:text-white",
                                "{min:.2} min"
                            }
                        }
                    }
                    div{
                        class:"mt-4 mb-6",
                        div{
                            class:"text-sm text-neutral-600 dark:text-white",
                            dangerous_inner_html:"{v.description_original}",
                        }
                    }
                    div{
                        div{
                            class:"flex items-center justify-between text-slate-500",
                            div{
                                class:"flex space-x-4 md:space-x-8",
                            }
                        }
                    }
                }
            }
        )
    });
    rsx!(
        div{
            class:"flex flex-col space-y-4",
            {list_display}
        }
    )
}