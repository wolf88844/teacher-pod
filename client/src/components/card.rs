use dioxus::prelude::*;
use models::podcast::{BestPodcasts, Podcast};

use crate::PLAYER_STATUS;

#[component]
pub fn Card(children: Element) -> Element {
    rsx!(
        div{
            class:"bg-white dark:bg-gray-800 shadow overflow-hidden sm:rounded-lg",
            div{
                class:"px-4 py-5 sm:px-6",
                {children}
            }
        }
    )
}

#[derive(Props, PartialEq, Clone)]
pub struct RecommendListPrpos {
    data: BestPodcasts,
}

#[component]
pub fn RecommendList(props: RecommendListPrpos) -> Element {
    rsx!(
        div{
            class:"mt-6 grid grid-cols-1 gap-y-10 sm:grid-cols-2 gap-x-6 lg:grid-cols-3 xl:grid-cols-4 xl:gap-x-8",
            for item in props.data.podcasts{
                    Link{
                        class:"group",
                        to:"/content/{item.id}",
                        div{
                            class:"w-full aspect-w-1 aspect-h-1 bg-gray-200 rounded-lg overflow-hidden xl:aspect-w-7 xl:aspect-h-8",
                            img{
                                class:"w-full h-full object-center object-cover group-hover:opacity-75",
                                src:"{item.thumbnail}",
                            }
                        }
                        p{
                            class:"mt-1 text-lg font-medium text-gray-900 dark:text-white",
                            "{item.title}"
                        }
                        p{
                            class:"text-sm text-gray-500",
                            "Author: {item.publisher}"
                        }
                    }
            }
        }
    )
}

#[component]
pub fn EpisodeList(data: Podcast) -> Element {
    let playbox = PLAYER_STATUS.signal();
    let episodes = data.episodes.clone();

    Card(
        div{}
    )
}

#[component]
pub fn PopularTopics(data: Vec<String>) -> Element {
    rsx!(
     div{
         class:"mt-6 grid grid-cols-6 gap-x-4 gap-y-1 max-w-2xl",
         for item in data{
             div{
                 class:"col-span-2",
                 Link{
                     to:"/topic/{item}",
                     img{
                         class:"rounded-xl brightness-75",
                         src:"{item}",
                     }
                 }
                 p{
                     class:"text-xs -translate-y-6 text-white font-semibold sm:-translate-y-8 sm:text-base translate-x-3",
                     "{item}"
                 }
             }
         }
     }
    )
}
