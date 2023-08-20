use crate::HTTP_CLIENT;
use ureq::{Error, Response};

pub fn fetch(uri: &str) -> Result<String, Box<Error>> {
    let resp: Response = HTTP_CLIENT.get(uri).call()?;

    let mut body: String = String::new();

    if let Err(err) = resp.into_reader().read_to_string(&mut body) {
        Err(Box::new(err.into()))
    } else {
        Ok(body)
    }
}
