use dioxus::prelude::*;
use models::{data::SearchInfo, ApiData};
use serde::Deserialize;

use crate::{
    components::{card::Card, list::SearchResultList},
    data::request,
    pages::error::_404,
};

#[derive(Deserialize, Debug)]
struct RequestData {
    result: Option<SearchInfo>,
}

pub fn SearchResult(query: String) -> Element {
    if query.is_empty() {
        return rsx! {
            _404{}
        };
    }
    let query_str = query.clone();
    let request_data = use_resource(move || {
        let value = query_str.clone();
        async move {
        let res = request::get(&format!("/search/{}", value))
            .await
            .send()
            .await;
        let res = if let Ok(resp) = res {
            resp
        } else {
            return RequestData { result: None };
        };

        let result = res.json::<ApiData<SearchInfo>>().await.unwrap_or_default();
        let result = result.data;
        RequestData {
            result: Some(result),
        }
    }
    });

    let data = request_data.read();
    match data.as_ref() {
        Some(data) => {
            let data = data.result.as_ref();
            if data.is_none() {
                return rsx!();
            }
            rsx! (
                div {
                    class: "container mx-auto",
                    Card {
                        h1 {
                            class: "text-3xl font-semibold dark:text-white",
                            "Result for '",
                            span {
                                class: "font-bold",
                                "{query}"
                            },
                            "'"
                        }
                        br {}
                        hr {}
                        br {}
                        SearchResultList {
                            data: data.unwrap().clone()
                        }
                    }
                }
            )
        }
        None => {
            return rsx!(div {});
        }
    }
}
