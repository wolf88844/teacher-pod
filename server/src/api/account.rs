use std::iter::repeat_with;

use models::account::{Account, Auth};
use salvo::prelude::*;
use salvo::{Depot, handler};
use serde_json::json;

use crate::api::block_unlogin;
use crate::error::{ApiResult, Error};
use crate::models::account::AccountFn;
use crate::models::account::AuthFn;
use crate::{Routers, auth};

#[handler]
async fn current_account(depot: &mut Depot, resp: &mut Response) -> ApiResult {
    let user = depot.get::<Account>("user-info");
    if user.is_err() {
        return Err(Error::Unauthorized);
    }
    resp.render(Json(user.unwrap()));
    Ok(())
}

#[handler]
async fn login(req: &mut Request, resp: &mut Response) -> ApiResult {
    let email = req.query::<String>("email");
    if email.is_none() {
        return Err(Error::QueryNotFound("email".into()));
    }
    let password = req.query::<String>("password");
    if password.is_none() {
        return Err(Error::QueryNotFound("password".into()));
    }

    let email = email.unwrap();
    let password = password.unwrap();

    let user = Account::query_from_email(&email).await?;

    let checker = Account::check_password(&password, &user.password, &user.salt);

    if checker {
        let now = chrono::Local::now().timestamp() as i32;
        let expire = now + 60 * 60 * 24 * 2;
        let auth_id: String = repeat_with(fastrand::alphanumeric).take(12).collect();
        let token = auth::encode(&auth::AuthClaims {
            exp: expire,
            iat: now,
            id: auth_id.clone(),
            user: user.id,
        });

        Auth::insert_auth_info(&auth_id, user.id, expire).await?;
        resp.render(Json(json!({
            "token":token,
            "expire":expire,
        })));
    } else {
        resp.status_code(StatusCode::BAD_REQUEST);
        resp.render(Json(json!({"message": "data not found"})));
    }

    Ok(())
}

#[handler]
async fn register(req: &mut Request, resp: &mut Response) -> ApiResult {
    let email = req.query::<String>("email");
    if email.is_none() {
        return Err(Error::QueryNotFound("email".into()));
    }

    let password = req.query::<String>("password");
    if password.is_none() {
        return Err(Error::QueryNotFound("password".into()));
    }

    let username = req.query::<String>("username");
    if username.is_none() {
        return Err(Error::QueryNotFound("username".into()));
    }

    let email = email.unwrap();
    let password = password.unwrap();
    let username = username.unwrap();

    if Account::email_exists(&email).await {
        return Err(Error::DataExists);
    }

    let _ = Account::insert_new_user(&username, &email, &password).await;
    resp.render(Json(json!({"message":"success"})));

    Ok(())
}

pub struct AccountApi;

impl Routers for AccountApi {
    fn build() -> Vec<Router> {
        vec![
            Router::new().path("/login").post(login),
            Router::new().path("/regisiter").post(register),
            Router::new()
                .path("self")
                .hoop(block_unlogin)
                .get(current_account),
        ]
    }
}
