use gloo::net::http::{Request, RequestBuilder};
use url::Url;

pub async fn get(url:&str) ->RequestBuilder{
    let url = &Url::parse(root_path().await)
    .unwrap()
    .join(url)
    .unwrap()
    .to_string();
    Request::get(url)
}

pub async fn post(url:&str) ->RequestBuilder{
    let url = &Url::parse(root_path().await)
    .unwrap()
    .join(url)
    .unwrap()
    .to_string();
    Request::post(url)
}

pub async fn root_path()->&'static str{
    let resp = Request::get("/root.txt").send().await;
    if let Ok(resp) = resp{
        let res = resp.text().await;
        if res.is_err(){
            return "https://localhost:3000/";
        }else{
            let res = res.unwrap();
            if res.is_empty() || resp.status()!=200{
                return "https://localhost:3000/";
            }
            return Box::leak(Box::new(res));
        }
    }
    "https://localhost:3000/"
}