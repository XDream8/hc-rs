use crate::HTTP_CLIENT;
use ureq::Error;

pub fn fetch(uri: &str) -> Result<String, Error> {
    HTTP_CLIENT
        .get(uri)
        .call()?
        .body_mut()
        .with_config()
        .limit(25 * 1024 * 1024)
        .read_to_string()
}
