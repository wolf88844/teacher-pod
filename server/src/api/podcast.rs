use models::podcast::{BestPodcasts, Podcast};
use salvo::prelude::*;

use crate::{error::{ApiResult, Error}, models::podcast::{BestPodcasesQuery, PodcastQuery}, Routers};

#[handler]
async fn get_podcast(req: &mut Request, res: &mut Response) -> ApiResult {
    let pid = req.param::<String>("id").unwrap_or_default();
    let info = Podcast::fetch_by_id(&pid).await;
    if info.is_none() {
        return Err(Error::DataNotFound);
    }
    let info = info.unwrap();
    res.render(Json(info));
    Ok(())
}

#[handler]
async fn recommend_podcasts(res:&mut Response)->ApiResult{
    let info = BestPodcasts::get_recommend().await.unwrap_or_default();
    res.render(Json(info));
    Ok(())
}

pub struct PodcastApi;
impl Routers for PodcastApi{
    fn build()->Vec<salvo::Router>{
        vec![
            Router::with_path("podcasts").get(recommend_podcasts),
            Router::with_path("podcasts/<id>").get(get_podcast),
        ]
    }
}
