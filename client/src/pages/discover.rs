use dioxus::prelude::*;
use models::{podcast::BestPodcasts, ApiData};
use serde::Deserialize;

use crate::{
    components::card::{Card, RecommendList},
    data::request,
};

#[derive(Deserialize, Debug)]
struct RequestData {
    recommend: Option<BestPodcasts>,
}

#[component]
pub fn Discover() -> Element {
    let request_data = use_resource(|| async move {
        let res = request::get("/podcasts/").await.send().await;
        let res = if let Ok(resp) = res {
            resp
        } else {
            return RequestData { recommend: None };
        };

        let recommend = res
            .json::<ApiData<BestPodcasts>>()
            .await
            .unwrap_or_default();
        let recommend = recommend.data;
        RequestData {
            recommend: Some(recommend),
        }
    });

    let data = request_data.read();
    match data.as_ref() {
        Some(v) => {
            rsx!(
                div{
                    class:"container mx-auto",
                    Card{
                        span{
                            class:"text-2xl font-extrabold tracking-tight text-gray-900 dark:text-gray-100",
                            "Recommended Podcasts"
                        }
                        if v.recommend.is_some(){
                            RecommendList{
                                data: v.recommend.clone().unwrap(),
                            }
                        }else{
                            div{
                                "Not Found"
                            }
                        }
                    }
                    br{}
                }
            )
        }
        _ => {
            rsx!(div {})
        }
    }
}
