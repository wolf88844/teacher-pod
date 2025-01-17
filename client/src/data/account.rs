use super::request::{get, post};
use crate::hooks::use_storage;
use models::{account::Account, ApiData};
use serde::{Deserialize, Serialize};

pub fn token() -> String {
    let storage = use_storage().unwrap();
    storage.get_item("auth").unwrap().unwrap_or_default()
}

pub async fn current_user() -> Option<Account> {
    let req = get("/self")
        .await
        .header("Authorization", &format!("Bearer {}", token()))
        .build();

    if req.is_err() {
        return None;
    }

    let resp = req.unwrap().send().await;
    if !resp.is_ok() {
        return None;
    }

    let resp = resp.unwrap();

    Some(resp.json::<ApiData<Account>>().await.unwrap().data)
}

#[derive(Serialize, Deserialize)]
struct AuthInfo {
    expire: i64,
    token: String,
}

pub async fn login(email: &str, password: &str) -> anyhow::Result<()> {
    let path = format!("/login?email={}&password={}", email, password);
    let resp = post(&path).await.build().unwrap().send().await?;
    let data = resp.json::<ApiData<AuthInfo>>().await?;

    if resp.status() != 200 {
        return Err(anyhow::anyhow!(match data.message.as_str() {
            "data not found" => "User Info Not Found".to_string(),
            _ => data.message,
        }));
    }

    let storage = use_storage()?;
    storage.set_item("auth", &data.data.token).unwrap();
    Ok(())
}

pub async fn register(email: &str, username: &str, password: &str) -> anyhow::Result<()> {
    let path = format!(
        "/register?email={}&username={}&password={}",
        email, username, password
    );
    let resp = post(&path).await.build().unwrap().send().await?;
    if resp.status() != 200 {
        return Err(anyhow::anyhow!("Register Failed"));
    }
    Ok(())
}
