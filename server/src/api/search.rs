use models::data::SearchInfo;
use salvo::handler;
use salvo::prelude::*;
use crate::error::ApiResult;
use crate::error::Error;
use crate::models::data::SearchInfoQuery;
use crate::Routers;


#[handler]
async fn search_episode(req:&mut Request,res:&mut Response)->ApiResult{
    let query = req.param::<String>("query").unwrap_or_default();
    let info = SearchInfo::search_episode(&query).await;
    if info.count==0{
        return Err(Error::DataNotFound);
    }
    res.render(Json(info));
    Ok(())
}

pub struct SearchApi;
impl Routers for SearchApi{
    fn build()->Vec<salvo::Router>{
        vec![
            Router::with_path("search/<query>").get(search_episode),
        ]
    } 
}