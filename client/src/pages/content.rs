use dioxus::prelude::*;
use dioxus_free_icons::icons::fa_solid_icons;
use dioxus_free_icons::Icon;
use models::{podcast::Podcast, ApiData};

use crate::{
    components::card::{Card, EpisodeList},
    data::request,
    pages::error::_404,
};

#[derive(Debug)]
struct ContentInfo {
    content: Podcast,
}

#[component]
pub fn Content(id: String) -> Element {
    let info = use_resource(use_reactive!(|id| async move {
        let res = request::get(&format!("/podcasts/{}", id))
            .await
            .build()
            .unwrap()
            .send()
            .await
            .unwrap();
        let content = res.json::<ApiData<Podcast>>().await.unwrap();
        let content = content.data;
        Some(ContentInfo { content })
    }));

    log::info!("{:?}", info.read());

    match info.read_unchecked().as_ref() {
        Some(Some(info)) => {
            let content = &info.content;
            let description = content.description.clone();
            let description = if description.len() > 350 {
                format!("{}...", &description[0..349])
            } else {
                description
            };

            return rsx!(
                div{
                    class:"container mx-auto",
                    Card{
                        div{
                            class:"grid grid-cols-4 gap-4",
                            div{
                                class:"col-span-1",
                                img{
                                    class:"w-full h-auto rounded-md",
                                    src:"{content.thumbnail}"
                                }
                            }
                            div{
                                class:"col-span-2",
                                h1{
                                    class:"text-3xl font-semibold dark:text-white",
                                    "{content.title}"
                                }
                                p{
                                    class:"font-semibold text-gray-500 dark:text-gray-300 mt-4",
                                    "{description}"
                                }
                                p{
                                    class:"mt-6 justify-center space-x-2",
                                    button{
                                        class:"inline-block px-6 py-2 border-2 border-blue-600 text-blue-600 font-medium text-xs
                                        leading-tight uppercase rounded hover:bg-black hover:bg-opacity-5 focus:outline-none focus:ring-0 
                                        transition duration-150 ease-in-out",
                                        onclick:|_|{

                                        },
                                        Icon{
                                            icon:fa_solid_icons::FaCirclePlay,
                                        }
                                    }
                                    button{
                                        class:"inline-block px-6 py-2 border-2 border-blue-600 text-blue-600 font-medium text-xs
                                        leading-tight uppercase rounded hover:bg-black hover:bg-opacity-5 focus:outline-none focus:ring-0 
                                        transition duration-150 ease-in-out",
                                        Icon{
                                            icon:fa_solid_icons::FaStar,
                                        }
                                    }

                                }
                            }
                            div{
                                ul{
                                    class:"list-reset flex flex-col h-full",
                                    li{
                                        class:"rounded-t relative -mb-px block border p-4 border-grey dark:text-white",
                                        strong{"Language"}
                                        "{content.language}"
                                    }
                                    li{
                                        class:"rounded-t relative -mb-px block border p-4 border-grey dark:text-white",
                                        strong{"Total Episodes Number: "}
                                        "{content.total_episodes}"
                                    }
                                    li{
                                        class:"rounded-t relative -mb-px block border p-4 border-grey dark:text-white",
                                        strong{"Publish Country: "}
                                        "{content.country}"
                                    }
                                    li{
                                        class:"rounded-t relative -mb-px block border p-4 border-grey dark:text-white",
                                        strong{"Favorites Number: "}
                                        "32k"
                                    }
                                }
                            }
                        }
                    }
                    br{}
                    EpisodeList{
                        data:content.clone(),
                    }
                }
            );
        }
        Some(None) => {
            return rsx! {
                _404{}
            };
        }
        None => {
            return rsx! {
                _404{}
            };
        }
    }
}
