use reqwest::header::HeaderMap;
use serde::de::DeserializeOwned;

use crate::TbaError;

pub const TBA_URL: &str = "https://www.thebluealliance.com/api/v3";

/// Extensions for reqwest.
pub trait TbaReqwestExt {
    fn tba_send_for_json<T: DeserializeOwned>(self) -> Result<T, TbaError>;
}

impl TbaReqwestExt for reqwest::blocking::RequestBuilder {
    fn tba_send_for_json<T: DeserializeOwned>(self) -> Result<T, TbaError> {
        let res = self.send()?.error_for_status()?.text()?;
        let r: T = serde_json::from_str(&res)?;
        Ok(r)
    }
}

#[derive(Debug, Default)]
pub(crate) struct CachedHttpClient {
    inner: Option<reqwest::blocking::Client>,
}

impl CachedHttpClient {
    pub fn get(&mut self, api_key: &str) -> Result<&mut reqwest::blocking::Client, TbaError> {
        let http_client_field = &mut self.inner;
        loop {
            if let Some(client) = http_client_field {
                return Ok(client);
            }
            let client = reqwest::blocking::ClientBuilder::new()
                .default_headers({
                    let mut map = HeaderMap::new();
                    map.insert("X-TBA-Auth-Key", api_key.try_into()?);
                    map
                })
                .gzip(true)
                .deflate(true)
                .brotli(true)
                .build()?;
            *http_client_field = Some(client);
        }
    }
}
