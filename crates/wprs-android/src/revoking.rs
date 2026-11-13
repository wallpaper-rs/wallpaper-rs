use crate::state::{api, linked};
use crate::{Error, keys};
use serde_json::{Value, json};

pub(crate) async fn revoke(device: &str, password: &str) -> Result<Value, Error> {
    linked()?;
    let (api, _) = api()?;
    let peer = keys::peer(&api, device).await?;
    api.post(&format!("/api/v1/me/devices/{}/revoke", peer.id), json!({ "password": password })).await?;
    keys::refresh(&api).await?;
    Ok(json!({ "revoked": peer.name }))
}

pub(crate) fn closed<T>(result: &Result<T, Error>) -> bool {
    matches!(result, Err(Error::Sync(e)) if e.code() == "unauthorized")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_a_refused_token_closes_the_session() {
        let server =
            |code: &str| wprs_sync::Error::Server { status: 401, code: code.into(), detail: String::new() };
        assert!(closed::<()>(&Err(server("unauthorized").into())));
        assert!(!closed::<()>(&Err(server("forbidden").into())));
        assert!(!closed::<()>(&Err(wprs_sync::Error::Network("down".into()).into())));
        assert!(!closed(&Ok(())));
    }
}
