use crate::HTTP_CLIENT;

pub fn fetch(uri: &str) -> Result<String, ureq::Error> {
    let resp = HTTP_CLIENT.get(uri).call()?;
    if resp.status() != 200 {

    }
    else {
	let mut body: String = String::new();
	resp.into_reader().read_to_string(&mut body)?;
        return Ok(body);
    }

    Ok(String::new())
}
