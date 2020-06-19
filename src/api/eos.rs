/**
 * The purpose of this module is to manage connexions to API EOS.
 */
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, USER_AGENT};
use reqwest::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct EosAuthentication {
    token: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct EosUser {
    id: u32,
}

/**
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

/**
 * Get the EOS user id given her/his email
 */
pub fn user_id(token: String, email: String, url: &str) -> Result<u32, Error> {
    let client = reqwest::blocking::Client::new();
    let login_url = format!("{}api/getWorkerbyEmail/{}/exec", url, email);

    let mut headers = HeaderMap::new();
    let authorization_value = format!("Bearer {}", token);
    headers.insert("Authorization", authorization_value.parse().unwrap());
    let res = client.get(login_url.as_str()).headers(headers).send()?;

    match res.status() {
        reqwest::StatusCode::OK => {}
        s => {
            debug!("Unknown user: {}", s);

            return Ok(0);
        }
    }

    let body = res.text().unwrap();
    let user: EosUser = serde_json::from_str(body.as_str()).unwrap();

    Ok(user.id)
}
