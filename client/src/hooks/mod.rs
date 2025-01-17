use web_sys::{window, Storage};

pub fn use_storage() -> anyhow::Result<Storage> {
    if let Some(win) = window() {
        let storage = win.local_storage();
        if storage.is_ok() {
            let temp = storage.unwrap();
            if temp.is_some() {
                return Ok(temp.unwrap());
            }
        }
        return Err(anyhow::anyhow!("LocalStorage not found"));
    } else {
        Err(anyhow::anyhow!("Window not found"))
    }
}
