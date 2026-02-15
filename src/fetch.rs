use crate::error::HcError;
use crate::HTTP_CLIENT;

pub fn fetch(uri: &str) -> Result<String, HcError> {
    HTTP_CLIENT
        .get(uri)
        .call()
        .and_then(|mut resp| {
            resp.body_mut()
                .with_config()
                .limit(25 * 1024 * 1024)
                .read_to_string()
        })
        .map_err(|err| HcError::Fetch {
            url: uri.to_string(),
            source: Box::new(err),
        })
}
