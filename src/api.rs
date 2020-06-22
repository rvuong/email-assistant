use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, USER_AGENT};
use reqwest::Error;

use serde::{Deserialize, Serialize};

pub mod cooptation;
pub mod eos;
pub mod profiling;

#[derive(Serialize, Deserialize, Debug)]
struct EosAuthentication {
    token: String,
}

/**
 * @todo This should be refactored with EOS authenticate() function
 *
 * POST request with url, login, & password. Should return a JSON-encapsulated auth token
 */
pub fn authenticate(username: &str, password: &str, url: &str) -> Result<String, Error> {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("reqwest"));
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json")); // Mandatory

    let client = reqwest::blocking::Client::new();
    let login_url = format!("{}api/login_check", url);
    let request_body = format!(
        "{{ \"_username\": \"{}\", \"_password\": \"{}\" }}",
        username, password
    );
    let res = client
        .post(login_url.as_str())
        .headers(headers)
        .body(request_body)
        .send()?;

    // Handles errors
    match res.status() {
        reqwest::StatusCode::OK => debug!("Authentication successful"),
        s => {
            debug!("Authentication error: {}", s);

            return Ok(String::from(""));
        }
    }

    // Token deserialization
    let body = res.text().unwrap();
    let j: EosAuthentication = serde_json::from_str(body.as_str()).unwrap();

    Ok(j.token.to_string())
}
