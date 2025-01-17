use dioxus::prelude::*;
use models::{podcast::Podcast, ApiData};

use crate::{data::request, pages::error::_404};

#[derive(Debug)]
struct ContentInfo{
    content:Podcast,
}

#[component]
pub fn Content(id:String)->Element{
    let info = use_resource(use_reactive!(|id| async move {
        let res = request::get(&format!("/podcasts/{}",id))
        .await.build().unwrap().send().await.unwrap();
    let content = res.json::<ApiData<Podcast>>().await.unwrap();
    let content = content.data;
    Some(ContentInfo{content})
    }));

    log::info!("{:?}",info.read());
    
    match info.read_unchecked().as_ref(){
        Some(Some(info))=>{
            let content = &info.content;
            let description = content.description.clone();
            let description = if description.len()>350{
                format!("{}...",&description[0..349])
            }else{
                description
            };

            return rsx!(
                div{
                    class:"container mx-auto",
                    Card{

                    }
                    br{}
                    EpisodeList{
                        data:content.clone(),
                    }
                }
            );

        },
        Some(None)=>{
            return rsx!{
                _404{}
            };
        },
        None=>{
            return rsx!{
                _404{}
            };
        }
    }
}