use super::{block_on, sync_error};
use crate::daemon::Daemon;
use crate::error::{Result, invalid};
use reqwest::Method;
use serde_json::json;

fn fingerprint_of(text: &str) -> Result<String> {
    let valid = text.len() == 64 && text.bytes().all(|b| b.is_ascii_hexdigit());
    valid.then(|| text.to_ascii_lowercase()).ok_or_else(|| invalid("give the fingerprint of an author key"))
}

fn public_of(text: &str) -> Result<String> {
    wprs_format::public_key(text).map_err(|e| invalid(e.to_string()))
}

impl Daemon {
    pub fn list_author_keys(&self) -> Result<String> {
        let (api, _) = self.account_api(true)?;
        let found = block_on(api.get("/api/v1/me/author-keys")).map_err(sync_error)?;
        Ok(json!({"items": found["items"]}).to_string())
    }

    pub fn add_author_key(&self, public_key: &str) -> Result<String> {
        let key = public_of(public_key)?;
        let (api, _) = self.account_api(true)?;
        let added =
            block_on(api.post("/api/v1/me/author-keys", json!({"public_key": key}))).map_err(sync_error)?;
        Ok(added["fingerprint"].as_str().unwrap_or_default().to_owned())
    }

    pub fn rotate_author_key(&self, from: &str, public_key: &str, signature: &str) -> Result<String> {
        let (from, key) = (fingerprint_of(from)?, public_of(public_key)?);
        let body = json!({"from": from, "public_key": key, "signature": signature});
        let (api, _) = self.account_api(true)?;
        let done = block_on(api.post("/api/v1/me/author-keys/rotate", body)).map_err(sync_error)?;
        Ok(done["fingerprint"].as_str().unwrap_or_default().to_owned())
    }

    pub fn remove_author_key(&self, fingerprint: &str) -> Result<()> {
        let path = format!("/api/v1/me/author-keys/{}", fingerprint_of(fingerprint)?);
        let (api, _) = self.account_api(true)?;
        block_on(api.slow(Method::DELETE, &path)).map_err(sync_error)?;
        Ok(())
    }

    pub fn list_uploads(&self) -> Result<String> {
        let (api, _) = self.account_api(true)?;
        let found = block_on(api.get("/api/v1/me/packages")).map_err(sync_error)?;
        Ok(json!({"uploads": found["uploads"]}).to_string())
    }

    pub fn cancel_upload(&self, id: &str) -> Result<()> {
        let fine =
            !id.is_empty() && id.len() <= 64 && id.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-');
        if !fine {
            return Err(invalid("give the id of an upload"));
        }
        let (api, _) = self.account_api(true)?;
        block_on(api.slow(Method::DELETE, &format!("/api/v1/uploads/{id}"))).map_err(sync_error)?;
        Ok(())
    }
}
