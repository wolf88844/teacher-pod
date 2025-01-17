use std::usize;

use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::fa_solid_icons;
use serde::{Deserialize, Serialize};
use models::podcast::Episode;
use dioxus::prelude::*;

use crate::PLAYER_STATUS;


#[derive(Serialize, Deserialize)]
pub struct PlayBoxInfo{
    pub playlist:Option<Vec<Episode>>,
    pub current:usize,
    pub display:bool,
    pub pause:bool,
}

pub fn PlayBox()->Element{
    let mut current_content = use_signal(||usize::MAX);
    let mut status = PLAYER_STATUS.signal();
    let status_content = status.read().current;

    let playlist = if status.read().playlist.is_none(){
        vec![]
    }else{
        status.read().playlist.clone().unwrap()
    };

    if status_content != *current_content.read(){
        current_content.set(status_content);
        return rsx!{
            div{}
        }
    }

    if playlist.get(status.read().current).is_none(){
        return rsx!(
            div{
                class: "fixed bottom-12 left-2 rounded-full w-10 h-10
                bg-white dark:bg-gray-900 hover:bg-block dark:hover:bg-white",
                button{
                    class: "justify-center w-full h-full text-black dark:text-white hover:text-white dark:hover:text-black",
                    Icon{
                        class:"w-full h-full",
                        icon: fa_solid_icons::FaCirclePlay,
                    }
                }
            }
        )
    }

    let info = playlist.get(status.read().current).unwrap();

    let player_hidden = if !status.read().display{
        "hidden"
    }else{
        "hidden sm:block"
    };

    let icon_hidden = if status.read().display{
        "hidden"
    }else{
        "hidden sm:block"
    };

    let full_titile = if info.title.is_empty(){
        &info.title_original
    }else{
        &info.title
    };

    let simple_tilte = if full_titile.len()>32{
        format!("{}...",&full_titile[0..32])
    }else{
        full_titile.to_string()
    };

    rsx!(
        div{
            class: "{icon_hidden} fixed bottom-12 left-2 rounded-full w-10 h-10 
            bg-white dark:bg-gray-900 hover:bg-block dark:hover:bg-white",
            button{
                class:"justify-center w-full h-full text-black dark:text-white hover:text-white dark:hover:text-black",
                onclick: move|_|{
                    status.write().display = true;
                },
                Icon{
                    class:"w-full h-full",
                    icon: fa_solid_icons::FaCirclePlay,
                }
            }
        }
        div{
            class:"{player_hidden} fixed bottom-8 left-0 w-1/3 h-20 rounded bg-white dark:bg-gray-900 shadow-2xl px-2 py-2 z-40",
            div{
                class: "flex h-full gap-2",
                div{
                    class:"flex-initial w-16",
                    img{
                        class:"h-full rounded",
                        src:"{info.thumbnail}",
                    }
                }
                div{
                    class: "grow relative",
                    div{
                        span{
                            class:"text-black dark:text-white",
                            title:"{info.title}",
                            "{simple_tilte}"
                        }
                        span{
                            class:"absolute right-0",
                            div{
                                class:"flex items-center justify-center",
                                div{
                                    class:"inline-flex",
                                    role:"group",
                                    button{
                                        class:"rounded-full inline-block px-1 py-1 text-black dark:text-white font-medium 
                                        text-xs leading-tight hover:bg-gray-800 hover:text-white transition duration-150 ease-in-out",
                                        r#type:"button",
                                        Icon{
                                            icon: fa_solid_icons::FaStar
                                        }
                                    }
                                    button{
                                        class:"rounded-full inline-block px-1 py-1 text-black dark:text-white font-medium 
                                        text-xs leading-tight hover:bg-gray-800 hover:text-white transition duration-150 ease-in-out",
                                        r#type:"button",
                                        onclick:move|_|{
                                            status.write().display=false;
                                        },
                                        Icon{
                                            icon: fa_solid_icons::FaCircleMinus,
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                div{
                    class:"absolute bottom-0 w-full",
                    audio{
                        id:"podcast-player",
                        class:"w-full h-8",
                        controls:"controls",
                        "controlsList":"nodownload",
                        autoplay:"true",
                        "oncontextmenu":"return false",
                        onpause:|e|{
                            log::info!("{:?}",e);
                        },
                        source{
                            id:"audio-source",
                            src:"{info.audio}",
                            "type":"audio/mp3"
                        }
                    }
                    script{
                        src:"/script/audio.js"
                    }
                }
            }
        }
    )
}