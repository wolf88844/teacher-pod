use models::account::{Account, Auth};
use salvo::prelude::*;

use crate::{
    auth,
    error::{ApiResult, Error},
    models::account::{AccountFn, AuthFn},
};

pub mod account;
pub mod podcast;
pub mod search;

#[handler]
pub async fn block_unlogin(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
    ctrl: &mut FlowCtrl,
) -> ApiResult {
    let token = req.header::<String>("Authorization");
    if token.is_none() {
        return Err(Error::AuthorizationFailed("authorization not found".into()));
    }
    let token = token.unwrap();

    if !token.starts_with("Bearer ") {
        return Err(Error::AuthorizationFailed("bearer not found".into()));
    }

    let token = token[7..token.len()].to_string();

    let claims = auth::decode(&token);
    if claims.is_none() {
        return Err(Error::AuthorizationFailed("decode failed".into()));
    }
    let claims = claims.unwrap();

    if !Auth::chekc_auth_info(&claims.id, claims.user).await {
        return Err(Error::AuthorizationFailed("check auth failed".into()));
    }

    let user = Account::query_from_id(claims.user).await;
    if user.is_err() {
        return Err(Error::AuthorizationFailed("query user failed".into()));
    }
    let user = user.unwrap();
    depot.insert("user-info", user);

    ctrl.call_next(req, depot, res).await;

    Ok(())
}
